use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bank {
    code: String,
    name: String,
    name_kana: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BankBranch {
    code: String,
    name: String,
    name_kana: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReimburseBankAccount {
    id: String,
    account_type: String,
    number: String,
    holder_name: String,
    holder_name_kana: String,
    bank: Bank,
    bank_branch: BankBranch,
}
