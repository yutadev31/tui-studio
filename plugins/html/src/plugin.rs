use tui_studio::plugin::Plugin;

#[derive(Default)]
pub struct HTMLPlugin;

impl Plugin for HTMLPlugin {
    fn get_name(&self) -> &'static str {
        "HTML Plugin"
    }
}

#[no_mangle]
pub unsafe extern "C" fn load_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(HTMLPlugin::default()))
}
