use anyhow::Result;
use async_trait::async_trait;

use crate::event::Event;

#[async_trait]
pub trait Component: Send {
    async fn on_event(&mut self, evt: Event) -> Result<()>;
}

#[async_trait]
pub trait DrawableComponent: Component {
    async fn draw(&self) -> Result<()>;
}
