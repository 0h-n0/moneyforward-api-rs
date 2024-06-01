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
        let api_key = std::env::var("MF_ACCESS_TOKEN").unwrap();
        let office_id = std::env::var("MF_OFFICE_ID").unwrap();
        let params: ExpenseReportParameters = ExpenseReportParameters {
            ..Default::default()
        };

        let client = Client::new(api_key);
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
