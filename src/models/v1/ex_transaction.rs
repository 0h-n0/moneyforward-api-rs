use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use crate::models::v1::ex_item::{ExItem, Item, SubItem};
use crate::models::v1::ex_office_member_setting::OfficeMember;
use crate::models::v1::ex_report::ExReport;
use crate::models::v1::ex_report_unit::ExReportUnit;
use crate::models::v1::excise::Excise;
use crate::models::v1::mf_file::MFFile;
use crate::models::v1::project_code::ProjectCode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExTransactionModel {
    id: String,
    number: Option<u32>,
    remark: Option<String>,
    value: Option<f32>,
    report_number: Option<String>,
    currency: Option<String>,
    use_custom_jpy_rate: Option<String>,
    automatic_status: Option<String>,
    error_message: Option<String>,
    warning_message: Option<String>,
    office_member_id: Option<String>,
    ex_item_id: Option<String>,
    dr_excise_id: Option<String>,
    dept_id: Option<String>,
    project_code_id: Option<String>,
    cr_item_id: Option<String>,
    cr_sub_item_id: Option<String>,
    attendants_count: Option<u8>,
    created_at: Option<String>,
    updated_at: Option<String>,
    receipt_type: Option<String>,
    office_member: Option<OfficeMember>,
    ex_item: Option<ExItem>,
    dr_excise: Option<Excise>,
    cr_item: Option<Item>,
    cr_sub_item: Option<SubItem>,
    project_code: Option<ProjectCode>,
    ex_report: Option<ExReport>,
    ex_report_unit: Option<ExReportUnit>,
    mf_file: Option<MFFile>,
    attendants: Option<Vec<Attendant>>,
    attendants_summary: Option<AttentdantsSummay>,
    family_state: Option<u8>,
    ex_transaction_family: Option<Vec<ExTransactionFamily>>,
    invoice_registration_number: Option<String>,
    special_exception_status: Option<String>,
    invoice_kind: Option<u32>,
    excise_code: Option<String>,
    excise_value: Option<u32>,
    purchase_tax_credit: Option<u32>,
    ex_transaction_custom_field_values: Option<Vec<ExTransactionCustomFieldValues>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attendant {
    id: String,
    name: String,
    company_name: String,
    depertment_name: Option<String>,
    position_name: Option<String>,
    is_own_company: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AttentdantsSummay {
    own_name: Option<String>,
    own_count: Option<u8>,
    other_name: Option<String>,
    other_count: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExTransactionFamily {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExTransactionCustomFieldValues {
    ex_transaction_custom_field_id: Option<String>,
    ex_transaction_custom_field_label: Option<String>,
    ex_transaction_custom_field_type: Option<String>,
    diplay_value: Option<String>,
    value: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExTransactionResponse {
    pub ex_transactions: Vec<ExTransactionModel>,
    pub next: Option<String>,
    pub prev: Option<String>,
}
