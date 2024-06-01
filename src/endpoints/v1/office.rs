use crate::{
    client::{
        Client,
        VERSION
    },
    models::v1::office::{
        OfficeParameters, 
        OfficeResponse
    },
    endpoints::v1::utils::parse_response
};


pub struct Office<'a> {
    client: &'a Client,
}

impl Client {
    pub fn office(&self) -> Office {
        Office { client: self }
    }
}

impl Office<'_> {
    pub async fn list(
        &self,
        version: VERSION,
        query: Option<OfficeParameters>,
    ) -> Result<OfficeResponse, Box<dyn std::error::Error>> {
        let (text, status) = self.client.get_with_query("", version, &query).await?;
        let (res, status) = parse_response(text, status)?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list() {
        let path = "/v1/offices/";
        let body = r#"{
            "offices": [
              {
                "id": "string",
                "identification_code": "09999999",
                "office_type_id": 2,
                "name": "string"
              }
            ],
            "next": "/api/external/v1/offices?page=3",
            "prev": "/api/external/v1/offices?page=1"
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
        let params = OfficeParameters { page: None };        
        let a = client
            .office()
            .list(VERSION::V1, Some(params))
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
