use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuspensePaymentReport {
    id: Option<String>,
    number: Option<u32>,
}
