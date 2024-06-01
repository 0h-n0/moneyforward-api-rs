use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use crate::models::v1::ex_report::ExReport;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExpenseReportParameters {
    pub page: Option<u32>,
    pub create_at_from: Option<DateTime<Local>>,
    pub create_at_to: Option<DateTime<Local>>,
    pub submitted_at_from: Option<DateTime<Local>>,
    pub submitted_at_to: Option<DateTime<Local>>,
    pub approved_at_from: Option<DateTime<Local>>,
    pub approved_at_to: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseReportResponse {
    pub expense_reports: Vec<ExReport>,
    pub next: Option<String>,
    pub prev: Option<String>,
}
