use utils::component::Component;

use crate::KeyConfig;

pub trait KeybindingComponent: Component {
    fn register_keybindings(&self, key_config: &mut KeyConfig);
}
