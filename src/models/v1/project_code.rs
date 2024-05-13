use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectCode {
    id: Option<String>,
    name: String,
    code: String,
    priority: Option<u32>,
    is_active: Option<bool>,
    parent_id: Option<String>,
    root_id: Option<String>,
}
