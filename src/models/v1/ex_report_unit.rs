use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExReportUnit {
    id: String,
    officd_id: Option<String>,
    number: Option<u32>,
    title: Option<String>,
    payment_date: Option<DateTime<Local>>,
    create_at: Option<DateTime<Local>>,
    update_at: Option<DateTime<Local>>,
}
