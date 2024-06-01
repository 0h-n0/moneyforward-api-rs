use std::fmt;

use chrono::format;
use chrono::format::parse;
use reqwest::Version;
use tracing_test::traced_test;
use reqwest::StatusCode;

use crate::client::Client;
use crate::client::VERSION;
use crate::{
    endpoints::v1::utils::parse_response,
    models::v1::{
        ex_transaction::{
            ExTransactionParameters,
            ExTransactionResponse
        },
        ex_report_unit::{
            ExReportUnit,
            ExReportUnitParameters,
            ExReportUnitResponse
        }
    }
};


pub struct ExReportUnitFunc<'a> {
    client: &'a Client,
}

impl Client {
    pub fn ex_report_unit(&self) -> ExReportUnitFunc {
        ExReportUnitFunc { client: self }
    }
}

impl ExReportUnitFunc<'_> {    
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
        match parse_response(text, status) {
            Ok((obj, status)) => Ok((obj, status)),
            Err(m) => Err(format!("{}, path=>{}", m, path).into()),
        }
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
        parse_response(text, status)
    }
    pub async fn list_transactions(
        &self,
        office_id: String,
        query: Option<ExReportUnitParameters>,
    ) -> Result<ExReportUnitResponse, Box<dyn std::error::Error>> {
        todo!()
    }
    pub async fn list(
        &self,
        office_id: String,
        query: Option<ExReportUnitParameters>,
    ) -> Result<ExReportUnitResponse, Box<dyn std::error::Error>> {
        let version = VERSION::V1;
        let path = format!("{}/ex_report_units", office_id);
        let (res, _status): (ExReportUnitResponse, reqwest::StatusCode) =
            self.send_query(&path, version, query).await?;    
        Ok(res)    
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
    async fn test_list_ex_report_units() -> Result<(), Box<dyn std::error::Error>> {
        let office_id = "111111";
        let path = "/v1/offices/111111/ex_report_units";
        let body = r#"{
            "ex_report_units": [
                {
                    "id": "string",
                    "office_id": "string",
                    "number": 0,
                    "title": "string",
                    "payment_date": "2024-05-31",
                    "created_at": "2024-05-31T18:20:02.382+09:00",
                    "updated_at": "2024-05-31T18:20:02.382+09:00"
                }
            ],
            "next": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/ex_report_units?page=3",
            "prev": "/api/external/v1/offices/XrB9-TG0I1o6KLTynyc36w/ex_report_units?page=1"
        }"#;
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create();

        let url = server.url();
        debug!("server url: {}", url.clone());
        let client = Client {
            http_client: reqwest::Client::new(),
            base_url: url.clone(),
            api_key: "".to_string(),
        };
        let params: ExReportUnitParameters = ExReportUnitParameters {
            page: None,
        };
        println!("{:?}", params);
        let a = client
            .ex_report_unit()
            .list(office_id.to_string(), Some(params))
            .await?;


        Ok(())
    }
}