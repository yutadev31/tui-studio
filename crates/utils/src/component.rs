use crate::event::Event;

pub trait Component<E>: Send {
    fn on_event(&mut self, evt: Event) -> Result<Vec<Event>, E>;
}

pub trait DrawableComponent<E>: Component<E> {
    fn draw(&self) -> Result<(), E>;
}
