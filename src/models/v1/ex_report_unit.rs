use chrono::{DateTime, NaiveDate, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};



#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExReportUnitParameters {
    pub page: Option<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExReportUnit {
    id: String,
    officd_id: Option<String>,
    number: Option<u32>,
    title: Option<String>,
    payment_date: Option<NaiveDate>,
    create_at: Option<DateTime<Local>>,
    update_at: Option<DateTime<Local>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExReportUnitResponse {
    pub ex_report_units: Vec<ExReportUnit>,
    pub next: Option<String>,
    pub prev: Option<String>
}
