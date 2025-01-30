use utils::component::Component;

use crate::CommandManager;

pub trait CommandComponent<E>: Component<E> {
    fn register_commands(&self, cmd_mgr: &mut CommandManager);
}
