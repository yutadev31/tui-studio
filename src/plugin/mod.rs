pub(crate) mod manager;

pub trait Plugin: Send {
    fn get_name(&self) -> &'static str;
}
