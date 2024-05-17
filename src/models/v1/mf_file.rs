use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MFFile {
    id: String,
    name: String,
    byte_size: u64,
    content_type: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}
