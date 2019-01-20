use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst_base;
use gst_base::subclass::prelude::*;
use gst_video;

use std::i32;
use std::sync::Mutex;

struct SsdTf {
    cat: gst::DebugCategory,
    srcpad: gst::Pad,
    sinkpad: gst::Pad,
}

impl SsdTf {
    fn set_pad_functions(sinkpad: &gst::Pad, srcpad: &gst::Pad) {
        sinkpad.set_chain_function(|pad, parent, buffer| {
            SsdTf::catch_panic_pad_function(
                parent,
                || Err(gst::FlowError::Error),
                |ssdtf, element| ssdtf.sink_chain(pad, element, buffer),
            )
        });
        sinkpad.set_event_function(|pad, parent, event| {
            SsdTf::catch_panic_pad_function(
                parent,
                || false,
                |ssdtf, element| ssdtf.sink_event(pad, element, event),
            )
        });
        sinkpad.set_query_function(|pad, parent, query| {
            SsdTf::catch_panic_pad_function(
                parent,
                || false,
                |ssdtf, element| ssdtf.sink_query(pad, element, query),
            )
        });

        srcpad.set_event_function(|pad, parent, event| {
            SsdTf::catch_panic_pad_function(
                parent,
                || false,
                |ssdtf, element| ssdtf.src_event(pad, element, event),
            )
        });
        srcpad.set_query_function(|pad, parent, query| {
            SsdTf::catch_panic_pad_function(
                parent,
                || false,
                |ssdtf, element| ssdtf.src_query(pad, element, query),
            )
        });
    }

    // Called whenever a new buffer is passed to our sink pad. Here buffers should be processed and
    // whenever some output buffer is available have to push it out of the source pad.
    // Here we just pass through all buffers directly
    //
    // See the documentation of gst::Buffer and gst::BufferRef to see what can be done with
    // buffers.
    fn sink_chain(
        &self,
        pad: &gst::Pad,
        _element: &gst::Element,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        gst_log!(self.cat, obj: pad, "Handling buffer {:?}", buffer);
        self.srcpad.push(buffer)
    }

    // Called whenever an event arrives on the sink pad. It has to be handled accordingly and in
    // most cases has to be either passed to Pad::event_default() on this pad for default handling,
    // or Pad::push_event() on all pads with the opposite direction for direct forwarding.
    // Here we just pass through all events directly to the source pad.
    //
    // See the documentation of gst::Event and gst::EventRef to see what can be done with
    // events, and especially the gst::EventView type for inspecting events.
    fn sink_event(&self, pad: &gst::Pad, _element: &gst::Element, event: gst::Event) -> bool {
        gst_log!(self.cat, obj: pad, "Handling event {:?}", event);
        self.srcpad.push_event(event)
    }

    // Called whenever a query is sent to the sink pad. It has to be answered if the element can
    // handle it, potentially by forwarding the query first to the peer pads of the pads with the
    // opposite direction, or false has to be returned. Default handling can be achieved with
    // Pad::query_default() on this pad and forwarding with Pad::peer_query() on the pads with the
    // opposite direction.
    // Here we just forward all queries directly to the source pad's peers.
    //
    // See the documentation of gst::Query and gst::QueryRef to see what can be done with
    // queries, and especially the gst::QueryView type for inspecting and modifying queries.
    fn sink_query(
        &self,
        pad: &gst::Pad,
        _element: &gst::Element,
        query: &mut gst::QueryRef,
    ) -> bool {
        gst_log!(self.cat, obj: pad, "Handling query {:?}", query);
        self.srcpad.peer_query(query)
    }

    // Called whenever an event arrives on the source pad. It has to be handled accordingly and in
    // most cases has to be either passed to Pad::event_default() on the same pad for default
    // handling, or Pad::push_event() on all pads with the opposite direction for direct
    // forwarding.
    // Here we just pass through all events directly to the sink pad.
    //
    // See the documentation of gst::Event and gst::EventRef to see what can be done with
    // events, and especially the gst::EventView type for inspecting events.
    fn src_event(&self, pad: &gst::Pad, _element: &gst::Element, event: gst::Event) -> bool {
        gst_log!(self.cat, obj: pad, "Handling event {:?}", event);
        self.sinkpad.push_event(event)
    }

    // Called whenever a query is sent to the source pad. It has to be answered if the element can
    // handle it, potentially by forwarding the query first to the peer pads of the pads with the
    // opposite direction, or false has to be returned. Default handling can be achieved with
    // Pad::query_default() on this pad and forwarding with Pad::peer_query() on the pads with the
    // opposite direction.
    // Here we just forward all queries directly to the sink pad's peers.
    //
    // See the documentation of gst::Query and gst::QueryRef to see what can be done with
    // queries, and especially the gst::QueryView type for inspecting and modifying queries.
    fn src_query(
        &self,
        pad: &gst::Pad,
        _element: &gst::Element,
        query: &mut gst::QueryRef,
    ) -> bool {
        gst_log!(self.cat, obj: pad, "Handling query {:?}", query);
        self.sinkpad.peer_query(query)
    }
}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
impl ObjectSubclass for SsdTf {
    const NAME: &'static str = "RsSsdTf";
    type ParentType = gst::Element;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    // This macro provides some boilerplate.
    glib_object_subclass!();

    // Called when a new instance is to be created. We need to return an instance
    // of our struct here and also get the class struct passed in case it's needed
    fn new_with_class(klass: &subclass::simple::ClassStruct<Self>) -> Self {
        // Create our two pads from the templates that were registered with
        // the class
        let templ = klass.get_pad_template("sink").unwrap();
        let sinkpad = gst::Pad::new_from_template(&templ, "sink");
        let templ = klass.get_pad_template("src").unwrap();
        let srcpad = gst::Pad::new_from_template(&templ, "src");

        // And then set all our pad functions for handling anything that happens
        // on these pads
        SsdTf::set_pad_functions(&sinkpad, &srcpad);

        // Return an instance of our struct and also include our debug category here.
        // The debug category will be used later whenever we need to put something
        // into the debug logs
        Self {
            cat: gst::DebugCategory::new(
                "rsssdtf",
                gst::DebugColorFlags::empty(),
                "SsdTf Element",
            ),
            srcpad,
            sinkpad,
        }
    }

    // Called exactly once when registering the type. Used for
    // setting up metadata for all instances, e.g. the name and
    // classification and the pad templates with their caps.
    //
    // Actual instances can create pads based on those pad templates
    // with a subset of the caps given here.
    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        // Set the element specific metadata. This information is what
        // is visible from gst-inspect-1.0 and can also be programatically
        // retrieved from the gst::Registry after initial registration
        // without having to load the plugin in memory.
        klass.set_metadata(
            "SsdTf",
            "Generic",
            "Does nothing with the data",
            "Sebastian Dröge <sebastian@centricular.com>",
        );

        // Create and add pad templates for our sink and source pad. These
        // are later used for actually creating the pads and beforehand
        // already provide information to GStreamer about all possible
        // pads that could exist for this type.

        // Our element can accept any possible caps on both pads
        let caps = gst::Caps::new_any();
        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &caps,
        );
        klass.add_pad_template(src_pad_template);

        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        );
        klass.add_pad_template(sink_pad_template);
    }
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for SsdTf {
    // This macro provides some boilerplate
    glib_object_impl!();

    // Called right after construction of a new instance
    fn constructed(&self, obj: &glib::Object) {
        // Call the parent class' ::constructed() implementation first
        self.parent_constructed(obj);

        // Here we actually add the pads we created in SsdTf::new() to the
        // element so that GStreamer is aware of their existence.
        let element = obj.downcast_ref::<gst::Element>().unwrap();
        element.add_pad(&self.sinkpad).unwrap();
        element.add_pad(&self.srcpad).unwrap();
    }
}

// Implementation of gst::Element virtual methods
impl ElementImpl for SsdTf {
    // Called whenever the state of the element should be changed. This allows for
    // starting up the element, allocating/deallocating resources or shutting down
    // the element again.
    fn change_state(
        &self,
        element: &gst::Element,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        gst_trace!(self.cat, obj: element, "Changing state {:?}", transition);

        // Call the parent class' implementation of ::change_state()
        self.parent_change_state(element, transition)
    }
}


pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(plugin, "ssdtf", 0, SsdTf::get_type())
}
