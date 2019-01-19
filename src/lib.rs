#[macro_use]
extern crate gst;
#[macro_use]
extern crate glib;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError > {
    Ok(())
}

gst::gst_plugin_define!(
    "ssdtf",
    "Single Shot Multi-box detector in TensorFlow",
    plugin_init,
    "0.1",
    "MIT",
    "ssdtf",
    "ssdtf",
    "https://github.com/mstallmo/gst-ssd",
    "2019-01-19"
);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
