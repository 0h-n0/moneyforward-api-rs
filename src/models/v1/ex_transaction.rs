use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Debug, Clone, Default)]
pub struct ExTransactionParameters {
    pub page: Option<u32>,
    pub dept_id: Option<String>,
    pub project_code_id: Option<String>,
    pub ex_item_id: Option<String>,
    pub office_member_id: Option<String>,
    pub number: Option<u32>,
    pub number_from: Option<u32>,
    pub number_to: Option<u32>,
    pub value_min: Option<f32>,
    pub value_max: Option<f32>,
    pub is_exported: Option<bool>,
    pub is_reported: Option<bool>,
    pub approved_at_from: Option<String>,
    pub approved_at_to: Option<String>,
    pub recoginized_at_from: Option<String>,
    pub recoginized_at_to: Option<String>,
    pub is_recoginized_at_bank: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExTransactionModel {
    id: String,
    identification_code: String,
    office_type_id: u8,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExTransactionResponse {
    pub offices: Vec<ExTransactionModel>,
    pub next: Option<String>,
    pub prev: Option<String>,
}
