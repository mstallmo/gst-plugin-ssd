# Single Shot Multi-box Detector Plugin
Single Shot Multi-box Detector (ssd) is a GStreamer plugin implementation of the computer vision algorithm created by Google written in Rust. 


This plugin can fit into any GStreamer pipeline to add multi-box object detection to an application. SSDTF takes in a raw video frame and returns a raw video frame with the box detector overlaid on the input frame.

##Build
To build just run the standard cargo build command: `cargo build --release`. This will create a libgstssd.so file that can be used as a GStreamer plugin.

##Install
This plugin installs just like any standard GSteamer plugin. Make sure to place the *.so artifact of the build into your `GST_PLUGIN_PATH` so it can be picked up by GStreamer.  

See the [GStreamer Docs](https://gstreamer.freedesktop.org/data/doc/gstreamer/head/gstreamer/html/gst-running.html) for more information on how to install custom plugins.

## Author
Mason Stallmo <masonstallmo@gmail.com>