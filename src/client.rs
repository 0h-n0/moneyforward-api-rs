use tokio;
use std::{collections::HashMap, fs::create_dir};
use reqwest;

static MF_V1_BASE_URL: &str = "https://expense.moneyforward.com/api/external/v1/offices";
static MF_V2_BASE_URL: &str = "https://expense.moneyforward.com/api/external/v2/offices";

pub struct Client {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
    headers: reqwest::header::HeaderMap
}

impl Client {    
    pub fn new(token: Option<String>) -> Self {
        let client = Client::new();
        let headers = MFExpenseClient::create_header(&token);        
        Self {token, client, headers}
    }
    fn create_header(token: &Option<String>) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        let token = match std::env::var("MF_ACCESS_TOKEN") {
                Ok(t) => t,
                Err(e) => "error".to_string(),
            };
            headers.insert("Authorization", format!("Bearer {}", &token).parse().unwrap());
            headers.insert("Accept", "application/json".parse().unwrap());
        headers
    }

    pub async fn offices(&self, page: u32) -> reqwest::Result<reqwest::Response>{
        let url = format!("{}external/v1/offices?page={}", MF_URL, page);
        println!("{:?}", &url);
        let c = self.client.get(url)  
            .headers(MFExpenseClient::create_header(&None));
        println!("{:?}", &c);

        let res = c.send()
            .await?;        
        Ok(res)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = MFExpenseClient::new(Some("hello".into()));
        let res = client.offices(1).await;
        match res {
            Ok(x) => println!("{:?}", x.json().await),
            _ => println!("{:?}", res),
        }
        
    }
}
