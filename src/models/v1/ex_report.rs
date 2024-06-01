use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use crate::models::v1::ex_office_member_setting::OfficeMember;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Approval {
    id: String,
    number: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExForm {
    label: String,
    value: String,
    is_required: bool,
    is_hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExReport {
    id: String,
    ex_report_type_id: Option<String>,
    office_number_id: Option<String>,
    number: Option<u32>,
    title: Option<String>,
    submitted_at: Option<DateTime<Local>>,
    approved_at: Option<DateTime<Local>>,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
    status: Option<String>,
    suspense_payment_reports: Option<Vec<Approval>>,
    ex_report_approvals: Option<Vec<ExReportApproval>>,
    ex_report_from_inputs: Option<Vec<ExForm>>,
    // (label, value, is_required, is_hidden)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExReportApproval {
    step: u32,
    is_active: bool,
    approved_at: Option<DateTime<Local>>,
    approve_office_member: Option<OfficeMember>,
}
