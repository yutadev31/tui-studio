use utils::component::Component;

use crate::CommandManager;

pub trait CommandComponent: Component {
    fn register_commands(&self, cmd_mgr: &mut CommandManager);
}
