use super::{command::CommandManager, event::Event, key_binding::KeyConfig};

pub trait Component<E>: Send {
    fn on_event(&mut self, evt: Event) -> Result<Vec<Event>, E>;
}

pub trait DrawableComponent<E>: Component<E> {
    fn draw(&self) -> Result<(), E>;
}

pub trait KeybindingComponent<E>: Component<E> {
    fn register_keybindings(&self, key_config: &mut KeyConfig);
}

pub trait CommandComponent<E>: Component<E> {
    fn register_commands(&self, cmd_mgr: &mut CommandManager);
}
