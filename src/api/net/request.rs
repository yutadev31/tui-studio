use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<T: Serialize> {
    pub command: String,
    pub content: T,
}
