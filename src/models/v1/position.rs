use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    id: String,
    name: String,
    is_authorizer: bool,
    priority: u32,
}
