use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};
use chrono::{DateTime, Local};

#[derive(Deserialize, Debug, Clone)]
pub struct MFFile {
    id: String,
    name: String,
    byte_size: u64,
    content_type: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}