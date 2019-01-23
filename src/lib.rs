#[macro_use]
extern crate gstreamer as gst;
#[macro_use]
extern crate glib;

mod ssmbd;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    ssmbd::register(plugin)?;
    Ok(())
}

gst_plugin_define!(
    "ssmbd",
    "Single Shot Multi-box detector",
    plugin_init,
    "1.0",
    "MIT/X11",
    "ssmbd",
    "ssmbd",
    "https://github.com/mstallmo/gst-plugin-ssd",
    "2019-01-21"
);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
