use crate::client::Client;
use crate::client::VERSION;
use crate::models::v1::expense_report::{ExpenseReportParameters, ExpenseReportResponse};
use std::fmt;
use tracing_test::traced_test;

pub struct ExpenseReport<'a> {
    client: &'a Client,
}

impl Client {
    pub fn expense_report(&self) -> ExpenseReport {
        ExpenseReport { client: self }
    }
}

impl ExpenseReport<'_> {
    pub async fn list(
        &self,
        version: VERSION,
        office_id: String,
        query: Option<ExpenseReportParameters>,
    ) -> Result<ExpenseReportResponse, Box<dyn std::error::Error>> {
        let path = format!("{}/expense_reports", office_id);
        let (res, status) = self.client.get_with_query(&path, version, &query).await?;
        match status {
            reqwest::StatusCode::OK => {
                let res = serde_json::from_str::<ExpenseReportResponse>(&res).unwrap();
                return Ok(res);
            }
            _ => {
                return Err(format!("Status code: {}, msg: {:?}", status, res).into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default;

    use super::*;

    #[tokio::test]
    #[traced_test]
    async fn list() {
        let office_id = "111111".into();
        let path = "/v1/offices/111111/expense_reports";
        let body = r#"{
            "expense_reports": [
              {
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
              }
            ],
            "next": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/expense_reports?page=3",
            "prev": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/expense_reports?page=1"
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

        let params: ExpenseReportParameters = ExpenseReportParameters {
            ..Default::default()
        };
        let a = client
            .expense_report()
            .list(VERSION::V1, office_id, Some(params))
            .await;
        match a {
            Ok(a) => println!("{:?}", a),
            Err(e) => println!("{:?}", e),
        }
    }
}
