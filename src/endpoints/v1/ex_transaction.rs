use chrono::format;

use crate::client::Client;
use crate::client::VERSION;
use crate::models::v1::ex_transaction::{ExTransactionParameters};
use std::fmt;
use tracing_test::traced_test;


pub struct ExTransaction<'a> {
    client: &'a Client,
}

impl Client {
    pub fn ex_transaction(&self) -> ExTransaction {
        ExTransaction { client: self }
    }
}

impl ExTransaction<'_> {
    pub async fn list(
        &self,
        version: VERSION,
        office_id: String,        
        query: Option<ExTransactionParameters>,
    ) -> Result<String, fmt::Error> {
        let path = format!("{}/me/ex_transactions", office_id);
        let res = self.client.get_with_query(&path, version, &query).await?;
        //let model = serde_json::from_str::<OfficeResponse>(&res).unwrap();
        Ok(res)
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
        let params: ExTransactionParameters = ExTransactionParameters { 
            ..Default::default() 
        };
        
        let client = Client::new(api_key);
        let a = client
            .ex_transaction()
            .list(VERSION::V1, office_id, Some(params))            
            .await
            .unwrap();
    }
}
