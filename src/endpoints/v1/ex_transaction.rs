use chrono::format;
use reqwest::Version;

use crate::client::Client;
use crate::client::VERSION;
use crate::models::v1::ex_transaction::{ExTransaction, ExTransactionParameters, ExTransactionResponse};
use std::fmt;
use tracing_test::traced_test;

pub struct ExTransactionFunc<'a> {
    client: &'a Client,
}

impl Client {
    pub fn ex_transaction(&self) -> ExTransactionFunc {
        ExTransactionFunc { client: self }
    }
}

impl ExTransactionFunc<'_> {
    async fn send_query<T, Q>(
        &self,
        path: &str,
        version: VERSION,
        query: Option<Q>,
    ) -> Result<(T, reqwest::StatusCode), Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::de::DeserializeOwned + serde::Serialize,
    {
        let (text, status) = self.client.get_with_query(&path, version, &query).await?;
        let parsed: T = serde_json::from_str(&text)?;

        Ok((parsed, status))
    }
    async fn get<T>(
        &self,
        path: &str,
        version: VERSION,
    ) -> Result<(T, reqwest::StatusCode), Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let (text, status) = self.client.get(&path, version).await?;
        let parsed: T = serde_json::from_str(&text)?;
        Ok((parsed, status))
    }

    pub async fn list(
        &self,
        office_id: String,
        query: Option<ExTransactionParameters>,
    ) -> Result<ExTransactionResponse, Box<dyn std::error::Error>> {
        let version = VERSION::V1;
        let path = format!("{}/me/ex_transactions", office_id);
        let (res, _status): (ExTransactionResponse, reqwest::StatusCode) =
            self.send_query(&path, version, query).await?;
        Ok(res)
    }
    pub async fn create_transaction(&self) -> String {
        todo!();
    }
    pub async fn find_transaction(
        &self,
        office_id: String,
        id: u32,
    ) -> Result<ExTransaction, Box<dyn std::error::Error>> {
        let version = VERSION::V1;
        let path = format!("{}/me/ex_transactions/{}", office_id, id);
        let (res, _status): (ExTransaction, reqwest::StatusCode) =
            self.get(&path, version).await?;
        Ok(res)
    }
    pub async fn update_transaction(&self) -> String {
        todo!();
    }
    pub async fn delete_transaction(&self) -> String {
        todo!();
    }
    pub async fn create_member_transaction(&self) -> String {
        todo!();
    }
    pub async fn list_office_transaction(&self) -> String {
        todo!();
    }
    pub async fn find_office_transaction(&self) -> String {
        todo!();
    }
    pub async fn update_office_transaction(&self) -> String {
        todo!();
    }
    pub async fn delete_office_transaction(&self) -> String {
        todo!();
    }
    pub async fn list_on_report(&self) -> String {
        todo!();
    }
    pub async fn list_on_report_unit(&self) -> String {
        todo!();
    }
    pub async fn upload_receipt(&self) -> String {
        todo!();
    }
    pub async fn upload_member_receipt(&self) -> String {
        todo!();
    }
    pub async fn find_office_uploaded_file(&self) -> String {
        todo!();
    }
    pub async fn find_uploaded_file(&self) -> String {
        todo!();
    }
    pub async fn find_ex_journal(&self) -> String {
        todo!();
    }
    pub async fn list_ex_journal(&self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use mockito;
    use std::default;
    use tracing::debug;

    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let office_id = "111111";
        let path = "/v1/offices/111111/me/ex_transactions";
        let body = r#"{
            "ex_transactions": [
              {
                "id": "string",
                "number": 0,
                "remark": "string",
                "recognized_at": "2024-05-31",
                "value": 110,
                "memo": "string",
                "report_number": "string",
                "jpyrate": 123.45,
                "currency": "USD",
                "use_custom_jpy_rate": false,
                "automatic_status": "manual",
                "error_message": "string",
                "warning_message": "string",
                "waiting_message": "string",
                "office_member_id": "string",
                "ex_item_id": "string",
                "dr_excise_id": "string",
                "dept_id": "string",
                "project_code_id": "string",
                "cr_item_id": "string",
                "cr_sub_item_id": "string",
                "attendants_count": 0,
                "created_at": "2024-05-31T18:20:02.403+09:00",
                "updated_at": "2024-05-31T18:20:02.403+09:00",
                "receipt_type": "e_doc",
                "office_member": {
                  "id": "string",
                  "identification_code": "string",
                  "number": "string",
                  "name": "string",
                  "created_at": "2024-05-31T18:20:03.154+09:00",
                  "updated_at": "2024-05-31T18:20:03.154+09:00",
                  "ex_activated_at": "2024-05-31T18:20:03.154+09:00",
                  "is_ex_user": true,
                  "is_ex_authorizer": true,
                  "is_ex_administrator": true,
                  "ex_office_member_setting": {
                    "id": "string",
                    "use_agent": false,
                    "ex_member_role_id": 1,
                    "approving_priority": 100,
                    "is_dept_on_display": true,
                    "is_project_code_on_display": true,
                    "is_cr_item_on_display": true,
                    "is_dr_excise_on_display": true,
                    "is_expense_sub_account_on_display": true,
                    "is_template_on_display": true,
                    "is_report_number_on_display": true,
                    "is_auto_update_default_dept_id": true,
                    "default_cr_item": {
                      "id": "string",
                      "excise_id": "string",
                      "name": "勘定科目名称",
                      "code": "item-code-xxx"
                    },
                    "default_cr_sub_item": {
                      "id": "string",
                      "item_id": "string",
                      "excise_id": "string",
                      "name": "補助勘定科目名称",
                      "code": "sub-item-code-xxx"
                    },
                    "default_transportation_ex_item": {
                      "id": "string",
                      "name": "経費科目名称",
                      "code": "string",
                      "is_active": true,
                      "item_id": "string",
                      "sub_item_id": "string",
                      "default_excise_id": "string",
                      "item": {
                        "id": "string",
                        "excise_id": "string",
                        "name": "勘定科目名称",
                        "code": "item-code-xxx"
                      },
                      "sub_item": {
                        "id": "string",
                        "item_id": "string",
                        "excise_id": "string",
                        "name": "補助勘定科目名称",
                        "code": "sub-item-code-xxx"
                      },
                      "default_dr_excise": {
                        "id": "string",
                        "long_name": "課税仕入 8%",
                        "code": "税区分コード",
                        "rate": 0.08
                      }
                    },
                    "default_driving_expense_ex_item": {
                      "id": "string",
                      "name": "経費科目名称",
                      "code": "string",
                      "is_active": true,
                      "item_id": "string",
                      "sub_item_id": "string",
                      "default_excise_id": "string",
                      "item": {
                        "id": "string",
                        "excise_id": "string",
                        "name": "勘定科目名称",
                        "code": "item-code-xxx"
                      },
                      "sub_item": {
                        "id": "string",
                        "item_id": "string",
                        "excise_id": "string",
                        "name": "補助勘定科目名称",
                        "code": "sub-item-code-xxx"
                      },
                      "default_dr_excise": {
                        "id": "string",
                        "long_name": "課税仕入 8%",
                        "code": "税区分コード",
                        "rate": 0.08
                      }
                    },
                    "default_project_code": {
                      "id": "string",
                      "name": "本社移転プロジェクト",
                      "code": "project-code-001",
                      "priority": 0,
                      "is_active": true,
                      "parent_id": "string",
                      "root_id": "string"
                    },
                    "default_dept": {
                      "id": "string",
                      "name": "営業部",
                      "code": "string",
                      "disp_order": 0,
                      "is_active": true,
                      "parent_id": "string",
                      "ancestry_depth": 0,
                      "ex_journal_dept_id": "string",
                      "root_id": "string"
                    }
                  },
                  "reimburse_bank_account": {
                    "id": "string",
                    "account_type": 1,
                    "number": "1234567",
                    "holder_name": "山田　太郎",
                    "holder_name_kana": "ﾔﾏﾀﾞ ﾀﾛｳ",
                    "bank": {
                      "code": "0001",
                      "name": "みずほ銀行",
                      "name_kana": "ﾐｽﾞﾎ"
                    },
                    "bank_branch": {
                      "code": "093",
                      "name": "本店",
                      "name_kana": "ﾎﾝﾃﾝ"
                    }
                  },
                  "position": {
                    "id": "string",
                    "name": "部長",
                    "is_authorizer": true,
                    "priority": 0
                  },
                  "depts": [
                    {
                      "id": "string",
                      "name": "営業部",
                      "code": "string",
                      "disp_order": 0,
                      "is_active": true,
                      "parent_id": "string",
                      "ancestry_depth": 0,
                      "ex_journal_dept_id": "string",
                      "root_id": "string"
                    }
                  ]
                },
                "ex_item": {
                  "id": "string",
                  "name": "経費科目名称",
                  "code": "string",
                  "is_active": true,
                  "item_id": "string",
                  "sub_item_id": "string",
                  "default_excise_id": "string",
                  "item": {
                    "id": "string",
                    "excise_id": "string",
                    "name": "勘定科目名称",
                    "code": "item-code-xxx"
                  },
                  "sub_item": {
                    "id": "string",
                    "item_id": "string",
                    "excise_id": "string",
                    "name": "補助勘定科目名称",
                    "code": "sub-item-code-xxx"
                  },
                  "default_dr_excise": {
                    "id": "string",
                    "long_name": "課税仕入 8%",
                    "code": "税区分コード",
                    "rate": 0.08
                  }
                },
                "dr_excise": {
                  "id": "string",
                  "long_name": "課税仕入 8%",
                  "code": "税区分コード",
                  "rate": 0.08
                },
                "cr_item": {
                  "id": "string",
                  "excise_id": "string",
                  "name": "勘定科目名称",
                  "code": "item-code-xxx"
                },
                "cr_sub_item": {
                  "id": "string",
                  "item_id": "string",
                  "excise_id": "string",
                  "name": "補助勘定科目名称",
                  "code": "sub-item-code-xxx"
                },
                "dept": {
                  "id": "string",
                  "name": "営業部",
                  "code": "string",
                  "disp_order": 0,
                  "is_active": true,
                  "parent_id": "string",
                  "ancestry_depth": 0,
                  "ex_journal_dept_id": "string",
                  "root_id": "string"
                },
                "project_code": {
                  "id": "string",
                  "name": "本社移転プロジェクト",
                  "code": "project-code-001",
                  "priority": 0,
                  "is_active": true,
                  "parent_id": "string",
                  "root_id": "string"
                },
                "ex_report": {
                  "id": "string",
                  "ex_report_type_id": "string",
                  "office_member_id": "string",
                  "number": 0,
                  "title": "string",
                  "submitted_at": "2024-05-31T18:20:03.951+09:00",
                  "approved_at": "2024-05-31T18:20:03.951+09:00",
                  "created_at": "2024-05-31T18:20:03.951+09:00",
                  "updated_at": "2024-05-31T18:20:03.951+09:00",
                  "status": "approved",
                  "suspense_payment_reports": [
                    {
                      "id": "string",
                      "number": 0
                    }
                  ],
                  "ex_report_approvals": [
                    {
                      "step": 1,
                      "is_active": true,
                      "approved_at": "2024-05-31T18:20:04.034+09:00",
                      "approve_office_member": {
                        "id": "string",
                        "identification_code": "string",
                        "number": "string",
                        "name": "string",
                        "created_at": "2024-05-31T18:20:03.154+09:00",
                        "updated_at": "2024-05-31T18:20:03.154+09:00",
                        "ex_activated_at": "2024-05-31T18:20:03.154+09:00",
                        "is_ex_user": true,
                        "is_ex_authorizer": true,
                        "is_ex_administrator": true,
                        "ex_office_member_setting": {
                          "id": "string",
                          "use_agent": false,
                          "ex_member_role_id": 1,
                          "approving_priority": 100,
                          "is_dept_on_display": true,
                          "is_project_code_on_display": true,
                          "is_cr_item_on_display": true,
                          "is_dr_excise_on_display": true,
                          "is_expense_sub_account_on_display": true,
                          "is_template_on_display": true,
                          "is_report_number_on_display": true,
                          "is_auto_update_default_dept_id": true,
                          "default_cr_item": {
                            "id": "string",
                            "excise_id": "string",
                            "name": "勘定科目名称",
                            "code": "item-code-xxx"
                          },
                          "default_cr_sub_item": {
                            "id": "string",
                            "item_id": "string",
                            "excise_id": "string",
                            "name": "補助勘定科目名称",
                            "code": "sub-item-code-xxx"
                          },
                          "default_transportation_ex_item": {
                            "id": "string",
                            "name": "経費科目名称",
                            "code": "string",
                            "is_active": true,
                            "item_id": "string",
                            "sub_item_id": "string",
                            "default_excise_id": "string",
                            "item": {
                              "id": "string",
                              "excise_id": "string",
                              "name": "勘定科目名称",
                              "code": "item-code-xxx"
                            },
                            "sub_item": {
                              "id": "string",
                              "item_id": "string",
                              "excise_id": "string",
                              "name": "補助勘定科目名称",
                              "code": "sub-item-code-xxx"
                            },
                            "default_dr_excise": {
                              "id": "string",
                              "long_name": "課税仕入 8%",
                              "code": "税区分コード",
                              "rate": 0.08
                            }
                          },
                          "default_driving_expense_ex_item": {
                            "id": "string",
                            "name": "経費科目名称",
                            "code": "string",
                            "is_active": true,
                            "item_id": "string",
                            "sub_item_id": "string",
                            "default_excise_id": "string",
                            "item": {
                              "id": "string",
                              "excise_id": "string",
                              "name": "勘定科目名称",
                              "code": "item-code-xxx"
                            },
                            "sub_item": {
                              "id": "string",
                              "item_id": "string",
                              "excise_id": "string",
                              "name": "補助勘定科目名称",
                              "code": "sub-item-code-xxx"
                            },
                            "default_dr_excise": {
                              "id": "string",
                              "long_name": "課税仕入 8%",
                              "code": "税区分コード",
                              "rate": 0.08
                            }
                          },
                          "default_project_code": {
                            "id": "string",
                            "name": "本社移転プロジェクト",
                            "code": "project-code-001",
                            "priority": 0,
                            "is_active": true,
                            "parent_id": "string",
                            "root_id": "string"
                          },
                          "default_dept": {
                            "id": "string",
                            "name": "営業部",
                            "code": "string",
                            "disp_order": 0,
                            "is_active": true,
                            "parent_id": "string",
                            "ancestry_depth": 0,
                            "ex_journal_dept_id": "string",
                            "root_id": "string"
                          }
                        },
                        "reimburse_bank_account": {
                          "id": "string",
                          "account_type": 1,
                          "number": "1234567",
                          "holder_name": "山田　太郎",
                          "holder_name_kana": "ﾔﾏﾀﾞ ﾀﾛｳ",
                          "bank": {
                            "code": "0001",
                            "name": "みずほ銀行",
                            "name_kana": "ﾐｽﾞﾎ"
                          },
                          "bank_branch": {
                            "code": "093",
                            "name": "本店",
                            "name_kana": "ﾎﾝﾃﾝ"
                          }
                        },
                        "position": {
                          "id": "string",
                          "name": "部長",
                          "is_authorizer": true,
                          "priority": 0
                        },
                        "depts": [
                          {
                            "id": "string",
                            "name": "営業部",
                            "code": "string",
                            "disp_order": 0,
                            "is_active": true,
                            "parent_id": "string",
                            "ancestry_depth": 0,
                            "ex_journal_dept_id": "string",
                            "root_id": "string"
                          }
                        ]
                      }
                    }
                  ],
                  "ex_report_form_inputs": [
                    {
                      "label": "string",
                      "value": "string",
                      "is_required": true,
                      "is_hidden": true
                    }
                  ]
                },
                "ex_report_unit": {
                  "id": "string",
                  "office_id": "string",
                  "number": 0,
                  "title": "string",
                  "payment_date": "2024-05-31",
                  "created_at": "2024-05-31T18:20:02.382+09:00",
                  "updated_at": "2024-05-31T18:20:02.382+09:00"
                },
                "mf_file": {
                  "id": "string",
                  "name": "領収書",
                  "byte_size": 270800,
                  "content_type": "string",
                  "created_at": "2024-05-31T18:20:04.000+09:00",
                  "updated_at": "2024-05-31T18:20:04.000+09:00"
                },
                "attendants": [
                  {
                    "id": "string",
                    "name": "山田太郎",
                    "company_name": "A株式会社",
                    "department_name": "営業部",
                    "position_name": "部長",
                    "is_own_company": false
                  }
                ],
                "attendants_summary": {
                  "own_name": "自社代表者",
                  "own_count": 0,
                  "other_name": "他社代表者",
                  "other_count": 0
                },
                "family_state": 0,
                "ex_transaction_family": [
                  {
                    "id": "string"
                  }
                ],
                "invoice_registration_number": "T1234567891012",
                "special_exception_status": "特例区分",
                "invoice_kind": 1,
                "excise_code": "001_1",
                "excise_value": 800,
                "purchase_tax_credit": 800,
                "ex_transaction_custom_field_values": [
                  {
                    "ex_transaction_custom_field_id": "string",
                    "ex_transaction_custom_field_label": "カスタムマスタ1",
                    "ex_transaction_custom_field_type": "汎用マスタ",
                    "display_value": "NAME_XXXX（CODE_XXXX）",
                    "value": "CODE_XXXX",
                    "name": "NAME_XXXX"
                  }
                ]
              }
            ],
            "next": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/me/ex_transactions?page=3",
            "prev": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/me/ex_transactions?page=1"
          }"#;
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create();

        let url = server.url();
        let client = Client {
            http_client: reqwest::Client::new(),
            base_url: url.clone(),
            api_key: "".to_string(),
        };
        let params: ExTransactionParameters = ExTransactionParameters {
            ..Default::default()
        };
        let a = client
            .ex_transaction()
            .list(office_id.to_string(), Some(params))
            .await?;
        println!("================={:?}", a);
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn test_find_transaction() -> Result<(), Box<dyn std::error::Error>> {
        let office_id = "111111";
        let ex_id = 222222;
        let path = "/v1/offices/111111/me/ex_transactions/222222";
        let body = r#"{
            "id": "string",
            "number": 0,
            "remark": "string",
            "recognized_at": "2024-05-31",
            "value": 110,
            "memo": "string",
            "report_number": "string",
            "jpyrate": 123.45,
            "currency": "USD",
            "use_custom_jpy_rate": false,
            "automatic_status": "manual",
            "error_message": "string",
            "warning_message": "string",
            "waiting_message": "string",
            "office_member_id": "string",
            "ex_item_id": "string",
            "dr_excise_id": "string",
            "dept_id": "string",
            "project_code_id": "string",
            "cr_item_id": "string",
            "cr_sub_item_id": "string",
            "attendants_count": 0,
            "created_at": "2024-05-31T18:20:02.403+09:00",
            "updated_at": "2024-05-31T18:20:02.403+09:00",
            "receipt_type": "e_doc",
            "office_member": {
              "id": "string",
              "identification_code": "string",
              "number": "string",
              "name": "string",
              "created_at": "2024-05-31T18:20:03.154+09:00",
              "updated_at": "2024-05-31T18:20:03.154+09:00",
              "ex_activated_at": "2024-05-31T18:20:03.154+09:00",
              "is_ex_user": true,
              "is_ex_authorizer": true,
              "is_ex_administrator": true,
              "ex_office_member_setting": {
                "id": "string",
                "use_agent": false,
                "ex_member_role_id": 1,
                "approving_priority": 100,
                "is_dept_on_display": true,
                "is_project_code_on_display": true,
                "is_cr_item_on_display": true,
                "is_dr_excise_on_display": true,
                "is_expense_sub_account_on_display": true,
                "is_template_on_display": true,
                "is_report_number_on_display": true,
                "is_auto_update_default_dept_id": true,
                "default_cr_item": {
                  "id": "string",
                  "excise_id": "string",
                  "name": "勘定科目名称",
                  "code": "item-code-xxx"
                },
                "default_cr_sub_item": {
                  "id": "string",
                  "item_id": "string",
                  "excise_id": "string",
                  "name": "補助勘定科目名称",
                  "code": "sub-item-code-xxx"
                },
                "default_transportation_ex_item": {
                  "id": "string",
                  "name": "経費科目名称",
                  "code": "string",
                  "is_active": true,
                  "item_id": "string",
                  "sub_item_id": "string",
                  "default_excise_id": "string",
                  "item": {
                    "id": "string",
                    "excise_id": "string",
                    "name": "勘定科目名称",
                    "code": "item-code-xxx"
                  },
                  "sub_item": {
                    "id": "string",
                    "item_id": "string",
                    "excise_id": "string",
                    "name": "補助勘定科目名称",
                    "code": "sub-item-code-xxx"
                  },
                  "default_dr_excise": {
                    "id": "string",
                    "long_name": "課税仕入 8%",
                    "code": "税区分コード",
                    "rate": 0.08
                  }
                },
                "default_driving_expense_ex_item": {
                  "id": "string",
                  "name": "経費科目名称",
                  "code": "string",
                  "is_active": true,
                  "item_id": "string",
                  "sub_item_id": "string",
                  "default_excise_id": "string",
                  "item": {
                    "id": "string",
                    "excise_id": "string",
                    "name": "勘定科目名称",
                    "code": "item-code-xxx"
                  },
                  "sub_item": {
                    "id": "string",
                    "item_id": "string",
                    "excise_id": "string",
                    "name": "補助勘定科目名称",
                    "code": "sub-item-code-xxx"
                  },
                  "default_dr_excise": {
                    "id": "string",
                    "long_name": "課税仕入 8%",
                    "code": "税区分コード",
                    "rate": 0.08
                  }
                },
                "default_project_code": {
                  "id": "string",
                  "name": "本社移転プロジェクト",
                  "code": "project-code-001",
                  "priority": 0,
                  "is_active": true,
                  "parent_id": "string",
                  "root_id": "string"
                },
                "default_dept": {
                  "id": "string",
                  "name": "営業部",
                  "code": "string",
                  "disp_order": 0,
                  "is_active": true,
                  "parent_id": "string",
                  "ancestry_depth": 0,
                  "ex_journal_dept_id": "string",
                  "root_id": "string"
                }
              },
              "reimburse_bank_account": {
                "id": "string",
                "account_type": 1,
                "number": "1234567",
                "holder_name": "山田　太郎",
                "holder_name_kana": "ﾔﾏﾀﾞ ﾀﾛｳ",
                "bank": {
                  "code": "0001",
                  "name": "みずほ銀行",
                  "name_kana": "ﾐｽﾞﾎ"
                },
                "bank_branch": {
                  "code": "093",
                  "name": "本店",
                  "name_kana": "ﾎﾝﾃﾝ"
                }
              },
              "position": {
                "id": "string",
                "name": "部長",
                "is_authorizer": true,
                "priority": 0
              },
              "depts": [
                {
                  "id": "string",
                  "name": "営業部",
                  "code": "string",
                  "disp_order": 0,
                  "is_active": true,
                  "parent_id": "string",
                  "ancestry_depth": 0,
                  "ex_journal_dept_id": "string",
                  "root_id": "string"
                }
              ]
            },
            "ex_item": {
              "id": "string",
              "name": "経費科目名称",
              "code": "string",
              "is_active": true,
              "item_id": "string",
              "sub_item_id": "string",
              "default_excise_id": "string",
              "item": {
                "id": "string",
                "excise_id": "string",
                "name": "勘定科目名称",
                "code": "item-code-xxx"
              },
              "sub_item": {
                "id": "string",
                "item_id": "string",
                "excise_id": "string",
                "name": "補助勘定科目名称",
                "code": "sub-item-code-xxx"
              },
              "default_dr_excise": {
                "id": "string",
                "long_name": "課税仕入 8%",
                "code": "税区分コード",
                "rate": 0.08
              }
            },
            "dr_excise": {
              "id": "string",
              "long_name": "課税仕入 8%",
              "code": "税区分コード",
              "rate": 0.08
            },
            "cr_item": {
              "id": "string",
              "excise_id": "string",
              "name": "勘定科目名称",
              "code": "item-code-xxx"
            },
            "cr_sub_item": {
              "id": "string",
              "item_id": "string",
              "excise_id": "string",
              "name": "補助勘定科目名称",
              "code": "sub-item-code-xxx"
            },
            "dept": {
              "id": "string",
              "name": "営業部",
              "code": "string",
              "disp_order": 0,
              "is_active": true,
              "parent_id": "string",
              "ancestry_depth": 0,
              "ex_journal_dept_id": "string",
              "root_id": "string"
            },
            "project_code": {
              "id": "string",
              "name": "本社移転プロジェクト",
              "code": "project-code-001",
              "priority": 0,
              "is_active": true,
              "parent_id": "string",
              "root_id": "string"
            },
            "ex_report": {
              "id": "string",
              "ex_report_type_id": "string",
              "office_member_id": "string",
              "number": 0,
              "title": "string",
              "submitted_at": "2024-05-31T18:20:03.951+09:00",
              "approved_at": "2024-05-31T18:20:03.951+09:00",
              "created_at": "2024-05-31T18:20:03.951+09:00",
              "updated_at": "2024-05-31T18:20:03.951+09:00",
              "status": "approved",
              "suspense_payment_reports": [
                {
                  "id": "string",
                  "number": 0
                }
              ],
              "ex_report_approvals": [
                {
                  "step": 1,
                  "is_active": true,
                  "approved_at": "2024-05-31T18:20:04.034+09:00",
                  "approve_office_member": {
                    "id": "string",
                    "identification_code": "string",
                    "number": "string",
                    "name": "string",
                    "created_at": "2024-05-31T18:20:03.154+09:00",
                    "updated_at": "2024-05-31T18:20:03.154+09:00",
                    "ex_activated_at": "2024-05-31T18:20:03.154+09:00",
                    "is_ex_user": true,
                    "is_ex_authorizer": true,
                    "is_ex_administrator": true,
                    "ex_office_member_setting": {
                      "id": "string",
                      "use_agent": false,
                      "ex_member_role_id": 1,
                      "approving_priority": 100,
                      "is_dept_on_display": true,
                      "is_project_code_on_display": true,
                      "is_cr_item_on_display": true,
                      "is_dr_excise_on_display": true,
                      "is_expense_sub_account_on_display": true,
                      "is_template_on_display": true,
                      "is_report_number_on_display": true,
                      "is_auto_update_default_dept_id": true,
                      "default_cr_item": {
                        "id": "string",
                        "excise_id": "string",
                        "name": "勘定科目名称",
                        "code": "item-code-xxx"
                      },
                      "default_cr_sub_item": {
                        "id": "string",
                        "item_id": "string",
                        "excise_id": "string",
                        "name": "補助勘定科目名称",
                        "code": "sub-item-code-xxx"
                      },
                      "default_transportation_ex_item": {
                        "id": "string",
                        "name": "経費科目名称",
                        "code": "string",
                        "is_active": true,
                        "item_id": "string",
                        "sub_item_id": "string",
                        "default_excise_id": "string",
                        "item": {
                          "id": "string",
                          "excise_id": "string",
                          "name": "勘定科目名称",
                          "code": "item-code-xxx"
                        },
                        "sub_item": {
                          "id": "string",
                          "item_id": "string",
                          "excise_id": "string",
                          "name": "補助勘定科目名称",
                          "code": "sub-item-code-xxx"
                        },
                        "default_dr_excise": {
                          "id": "string",
                          "long_name": "課税仕入 8%",
                          "code": "税区分コード",
                          "rate": 0.08
                        }
                      },
                      "default_driving_expense_ex_item": {
                        "id": "string",
                        "name": "経費科目名称",
                        "code": "string",
                        "is_active": true,
                        "item_id": "string",
                        "sub_item_id": "string",
                        "default_excise_id": "string",
                        "item": {
                          "id": "string",
                          "excise_id": "string",
                          "name": "勘定科目名称",
                          "code": "item-code-xxx"
                        },
                        "sub_item": {
                          "id": "string",
                          "item_id": "string",
                          "excise_id": "string",
                          "name": "補助勘定科目名称",
                          "code": "sub-item-code-xxx"
                        },
                        "default_dr_excise": {
                          "id": "string",
                          "long_name": "課税仕入 8%",
                          "code": "税区分コード",
                          "rate": 0.08
                        }
                      },
                      "default_project_code": {
                        "id": "string",
                        "name": "本社移転プロジェクト",
                        "code": "project-code-001",
                        "priority": 0,
                        "is_active": true,
                        "parent_id": "string",
                        "root_id": "string"
                      },
                      "default_dept": {
                        "id": "string",
                        "name": "営業部",
                        "code": "string",
                        "disp_order": 0,
                        "is_active": true,
                        "parent_id": "string",
                        "ancestry_depth": 0,
                        "ex_journal_dept_id": "string",
                        "root_id": "string"
                      }
                    },
                    "reimburse_bank_account": {
                      "id": "string",
                      "account_type": 1,
                      "number": "1234567",
                      "holder_name": "山田　太郎",
                      "holder_name_kana": "ﾔﾏﾀﾞ ﾀﾛｳ",
                      "bank": {
                        "code": "0001",
                        "name": "みずほ銀行",
                        "name_kana": "ﾐｽﾞﾎ"
                      },
                      "bank_branch": {
                        "code": "093",
                        "name": "本店",
                        "name_kana": "ﾎﾝﾃﾝ"
                      }
                    },
                    "position": {
                      "id": "string",
                      "name": "部長",
                      "is_authorizer": true,
                      "priority": 0
                    },
                    "depts": [
                      {
                        "id": "string",
                        "name": "営業部",
                        "code": "string",
                        "disp_order": 0,
                        "is_active": true,
                        "parent_id": "string",
                        "ancestry_depth": 0,
                        "ex_journal_dept_id": "string",
                        "root_id": "string"
                      }
                    ]
                  }
                }
              ],
              "ex_report_form_inputs": [
                {
                  "label": "string",
                  "value": "string",
                  "is_required": true,
                  "is_hidden": true
                }
              ]
            },
            "ex_report_unit": {
              "id": "string",
              "office_id": "string",
              "number": 0,
              "title": "string",
              "payment_date": "2024-05-31",
              "created_at": "2024-05-31T18:20:02.382+09:00",
              "updated_at": "2024-05-31T18:20:02.382+09:00"
            },
            "mf_file": {
              "id": "string",
              "name": "領収書",
              "byte_size": 270800,
              "content_type": "string",
              "created_at": "2024-05-31T18:20:04.000+09:00",
              "updated_at": "2024-05-31T18:20:04.000+09:00"
            },
            "attendants": [
              {
                "id": "string",
                "name": "山田太郎",
                "company_name": "A株式会社",
                "department_name": "営業部",
                "position_name": "部長",
                "is_own_company": false
              }
            ],
            "attendants_summary": {
              "own_name": "自社代表者",
              "own_count": 0,
              "other_name": "他社代表者",
              "other_count": 0
            },
            "family_state": 0,
            "ex_transaction_family": [
              {
                "id": "string"
              }
            ],
            "invoice_registration_number": "T1234567891012",
            "special_exception_status": "特例区分",
            "invoice_kind": 1,
            "excise_code": "001_1",
            "excise_value": 800,
            "purchase_tax_credit": 800,
            "ex_transaction_custom_field_values": [
              {
                "ex_transaction_custom_field_id": "string",
                "ex_transaction_custom_field_label": "カスタムマスタ1",
                "ex_transaction_custom_field_type": "汎用マスタ",
                "display_value": "NAME_XXXX（CODE_XXXX）",
                "value": "CODE_XXXX",
                "name": "NAME_XXXX"
              }
            ]
          }"#;
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create();

        let url = server.url();
        let client = Client {
            http_client: reqwest::Client::new(),
            base_url: url.clone(),
            api_key: "".to_string(),
        };
        let params: ExTransactionParameters = ExTransactionParameters {
            ..Default::default()
        };
        let a = client
            .ex_transaction()
            .find_transaction(office_id.to_string(), ex_id)
            .await?;
        println!("================={:?}", a);
        Ok(())
    }

    
}
