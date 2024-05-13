use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, vec};

use crate::models::v1::excise::Excise;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    id: String,
    excise_id: Option<String>,
    name: Option<String>,
    code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubItem {
    id: String,
    item_id: Option<String>,
    excise_id: Option<String>,
    name: Option<String>,
    code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExItem {
    id: String,
    name: Option<String>,
    code: Option<String>,
    is_active: Option<bool>,
    item_id: Option<String>,
    sub_item_id: Option<String>,
    default_excise_id: Option<String>,
    item: Option<Item>,
    sub_item: Option<SubItem>,
    default_dr_excise: Option<Excise>,
}
