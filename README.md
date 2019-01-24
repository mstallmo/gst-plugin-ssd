# Single Shot Multi-box Detector Plugin
Single Shot Multi-box Detector (ssd) is a GStreamer plugin implementation of the computer vision algorithm created by Google written in Rust. 


This plugin can fit into any GStreamer pipeline to add multi-box object detection to an application. SSDTF takes in a raw video frame and returns a raw video frame with the box detector overlaid on the input frame.

## Build
To build just run the standard cargo build command: `cargo build --release`. This will create a libgstssd.so file that can be used as a GStreamer plugin.

## Install
This plugin installs just like any standard GSteamer plugin. Make sure to place the *.so artifact of the build into your `GST_PLUGIN_PATH` so it can be picked up by GStreamer.  

See the [GStreamer Docs](https://gstreamer.freedesktop.org/data/doc/gstreamer/head/gstreamer/html/gst-running.html) for more information on how to install custom plugins.

Note: If you are having any issues building the plugin, make sure you have `libgstreamer-plugins-base-1.0-dev` installed as well as gstreamer itself. To install pluins-base-dev run
`sudo apt-get install libgstreamer-plugins-base-1.0-dev`

## Testing
To do a simple test after building the plugin run the following from the bash shell `gst-launch-1.0 -v -e v4l2src device=/dev/video0 ! queue ! video/x-h264,width=1920,height=1080,framerate=30/1 ! h264parse ! avdec_h264 ! ssmbd ! xvimagesink sync=false`

## Author
Mason Stallmo <masonstallmo@gmail.com>