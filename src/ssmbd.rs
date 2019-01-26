use aperature::obd::ObjectDetection;
use glib;
use glib::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use gst;
use gst::prelude::*;
use gst::subclass::prelude::*;

struct SingleShotMultiBox {
    cat: gst::DebugCategory,
    srcpad: gst::Pad,
    sinkpad: gst::Pad,
    detector: ObjectDetection,
}

impl SingleShotMultiBox {
    fn set_pad_functions(sinkpad: &gst::Pad, srcpad: &gst::Pad) {
        sinkpad.set_chain_function(|pad, parent, buffer| {
            SingleShotMultiBox::catch_panic_pad_function(
                parent,
                || Err(gst::FlowError::Error),
                |identity, element| identity.sink_chain(pad, element, buffer),
            )
        });
        sinkpad.set_event_function(|pad, parent, event| {
            SingleShotMultiBox::catch_panic_pad_function(
                parent,
                || false,
                |identity, element| identity.sink_event(pad, element, event),
            )
        });
        sinkpad.set_query_function(|pad, parent, query| {
            SingleShotMultiBox::catch_panic_pad_function(
                parent,
                || false,
                |identity, element| identity.sink_query(pad, element, query),
            )
        });

        srcpad.set_event_function(|pad, parent, event| {
            SingleShotMultiBox::catch_panic_pad_function(
                parent,
                || false,
                |identity, element| identity.src_event(pad, element, event),
            )
        });
        srcpad.set_query_function(|pad, parent, query| {
            SingleShotMultiBox::catch_panic_pad_function(
                parent,
                || false,
                |identity, element| identity.src_query(pad, element, query),
            )
        });
    }

    fn sink_chain(
        &self,
        pad: &gst::Pad,
        _element: &gst::Element,
        buffer: gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        gst_log!(self.cat, obj: pad, "Handling buffer {:?}", buffer);
        println!("Handling buffer: {:?}", buffer);
        self.srcpad.push(buffer)
    }

    fn sink_event(&self, pad: &gst::Pad, _element: &gst::Element, event: gst::Event) -> bool {
        gst_log!(self.cat, obj: pad, "Handling event {:?}", event);
        println!("Got sink Event {:?}", event);
        self.srcpad.push_event(event)
    }

    fn sink_query(
        &self,
        pad: &gst::Pad,
        _element: &gst::Element,
        query: &mut gst::QueryRef,
    ) -> bool {
        gst_log!(self.cat, obj: pad, "Handling query {:?}", query);
        self.srcpad.peer_query(query)
    }

    fn src_event(&self, pad: &gst::Pad, _element: &gst::Element, event: gst::Event) -> bool {
        gst_log!(self.cat, obj: pad, "Handling event {:?}", event);
        println!("Got src Event {:?}", event);
        self.sinkpad.push_event(event)
    }

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

impl ObjectSubclass for SingleShotMultiBox {
    const NAME: &'static str = "SsDtf";
    type ParentType = gst::Element;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new_with_class(klass: &subclass::simple::ClassStruct<Self>) -> Self {
        let templ = klass.get_pad_template("sink").unwrap();
        let sinkpad = gst::Pad::new_from_template(&templ, "sink");
        let templ = klass.get_pad_template("src").unwrap();
        let srcpad = gst::Pad::new_from_template(&templ, "src");
        let detector = ObjectDetection::init();

        SingleShotMultiBox::set_pad_functions(&sinkpad, &srcpad);

        Self {
            cat: gst::DebugCategory::new(
                "ssmbd",
                gst::DebugColorFlags::empty(),
                "SingleShotMultiBox Element",
            ),
            srcpad,
            sinkpad,
            detector,
        }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        klass.set_metadata(
            "SingleShotMultiBox",
            "Generic",
            "Does nothing with the data",
            "Mason Stallmo <masonstallmo@gmail.com>",
        );

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

impl ObjectImpl for SingleShotMultiBox {
    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);

        let element = obj.downcast_ref::<gst::Element>().unwrap();
        element.add_pad(&self.sinkpad).unwrap();
        element.add_pad(&self.srcpad).unwrap();
    }
}

impl ElementImpl for SingleShotMultiBox {
    fn change_state(
        &self,
        element: &gst::Element,
        transition: gst::StateChange,
    ) -> Result<gst::StateChangeSuccess, gst::StateChangeError> {
        gst_trace!(self.cat, obj: element, "Changing state {:?}", transition);
        println!("Changing plugin State to {:?}", transition);
        match transition {
            gst::StateChange::NullToReady => {
                //TODO: initialize tensorflow here
               // println!("Initializing TensorFlow!");
            },
            _ => {}
        }
        self.parent_change_state(element, transition)
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(plugin, "ssmbd", 0, SingleShotMultiBox::get_type())
}
