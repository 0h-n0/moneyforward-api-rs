use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficeParameters {
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficeModel {
    id: String,
    identification_code: String,
    office_type_id: u8,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficeResponse {
    pub offices: Vec<OfficeModel>,
    pub next: Option<String>,
    pub prev: Option<String>,
}
