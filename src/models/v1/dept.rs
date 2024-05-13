use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dept {
    id: Option<String>,
    name: String,
    code: String,
    disp_order: Option<u32>,
    is_active: Option<bool>,
    parent_id: Option<String>,
    ancestry_depth: Option<u32>,
    ex_journal_dept_id: Option<String>,
    root_id: Option<String>,
}
