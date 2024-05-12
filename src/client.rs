use core::fmt;
use reqwest;
use serde::Serialize;
use std::{collections::HashMap, fs::create_dir};
use tokio;


static MF_BASE_URL: &str = "https://expense.moneyforward.com/api/external";

#[derive(Debug, Copy, Clone)]
pub enum VERSION {
    V1,
    V2,
}

impl fmt::Display for VERSION {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VERSION::V1 => write!(f, "v1"),
            VERSION::V2 => write!(f, "v2"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: MF_BASE_URL.to_string(),
            api_key: api_key,
        }
    }
    pub fn build_request(
        &self,
        method: reqwest::Method,
        path: &str,
        version: VERSION,
        content_type: &str,
    ) -> reqwest::RequestBuilder {
        let url = format!("{}/{}/offices/{}", self.base_url, version, path);
        let mut request = self
            .http_client
            .request(method, &url)
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .bearer_auth(&self.api_key);
        request
    }
    pub async fn get(&self, path: &str, version: VERSION) -> Result<String, fmt::Error> 
    {
        let response = self
            .build_request(reqwest::Method::GET, path, version, "application/json")
            .send()
            .await
            .unwrap();
        let text = response.text().await.unwrap();
        Ok(text)
    }
    pub async fn get_with_query<Q>(
        &self,
        path: &str,
        version: VERSION,
        query: &Q,
    ) -> Result<String, fmt::Error>
    where
        Q: Serialize,
    {
        let response = self
            .build_request(reqwest::Method::GET, path, version, "application/json")
            .query(&query)
            .send()
            .await
            .unwrap();
        let text = response.text().await.unwrap();
        Ok(text)
    }

    pub async fn post(self) {}
}
