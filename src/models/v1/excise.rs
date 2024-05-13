use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Excise {
    id: String,
    long_name: Option<String>,
    code: Option<String>,
    rate: f32,
}
