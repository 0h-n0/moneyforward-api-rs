use crate::client::Client;
use crate::client::VERSION;
use crate::models::v1::office::{OfficeParameters, OfficeResponse};
use std::fmt;

pub struct Office<'a> {
    client: &'a Client,
}

impl Client {
    pub fn office(&self) -> Office {
        Office { client: self }
    }
}

impl Office<'_> {
    pub async fn page(
        &self,
        version: VERSION,
        query: Option<OfficeParameters>,
    ) -> Result<OfficeResponse, fmt::Error> {
        let res = self.client.get_with_query("", version, &query).await?;
        let model = serde_json::from_str::<OfficeResponse>(&res).unwrap();
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let params = OfficeParameters { page: 1 };
        let api_key = std::env::var("MF_ACCESS_TOKEN").unwrap();
        let client = Client::new(api_key);
        let a = client
            .office()
            .page(VERSION::V1, Some(params))
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
