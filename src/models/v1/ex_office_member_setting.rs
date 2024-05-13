use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::models::v1::bank::ReimburseBankAccount;
use crate::models::v1::dept::Dept;
use crate::models::v1::ex_item::{ExItem, Item, SubItem};
use crate::models::v1::position::Position;
use crate::models::v1::project_code::ProjectCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExOfficeMemberSetting {
    id: String,
    use_agent: Option<bool>,
    ex_member_role_id: Option<u32>,
    approving_priority: Option<u32>,
    is_dept_on_display: Option<bool>,
    is_project_code_on_display: Option<bool>,
    is_cr_item_on_display: Option<bool>,
    is_dr_excise_on_display: Option<bool>,
    is_expense_sub_account_on_display: Option<bool>,
    is_template_on_display: Option<bool>,
    is_report_number_on_display: Option<bool>,
    is_auto_update_default_dept_id: Option<bool>,
    default_cr_item: Option<Item>,
    default_cr_sub_item: Option<SubItem>,
    default_transportation_ex_item: Option<ExItem>,
    default_driving_expense_ex_item: Option<ExItem>,
    default_project_code: Option<ProjectCode>,
    default_dept: Option<Dept>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OfficeMember {
    id: String,
    identification_code: Option<String>,
    number: Option<String>,
    name: Option<String>,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
    ex_activated_at: Option<DateTime<Local>>,
    is_ex_user: Option<bool>,
    is_ex_authorizer: Option<bool>,
    is_ex_administrator: Option<bool>,
    ex_office_member_setting: Option<ExOfficeMemberSetting>,
    reimburse_bank_account: Option<ReimburseBankAccount>,
    position: Option<Position>,
    depts: Option<Vec<Dept>>,
}
