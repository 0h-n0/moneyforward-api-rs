use core::ascii;

use reqwest;
use serde;
use tokio;

static AUTH_TOKEN_URL: &str = "https://moneyforward.com/oauth/token";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TokenModel {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth_code: String,
    grant_type: String,
}

pub struct TokenManager {
    http_client: reqwest::Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth_code: String,
    access_token: Option<String>,
}

impl TokenManager {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_code: String,
    ) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            client_id,
            client_secret,
            redirect_uri,
            auth_code,
            access_token: None,
        }
    }

    pub async fn create(&self) -> Result<(), reqwest::Error> {
        //let c = self.http_client.post(AUTH_TOKEN_URL).header("Content-Type", "application/json").body().await?;
        Ok(())
    }
    pub async fn reflesh(&self) -> String {
        todo!();
    }
    pub async fn revoke(&self) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let token_manager = TokenManager::new(
            "client_id".to_string(),
            "client_secret".to_string(),
            "redirect_uri".to_string(),
            "auth_code".to_string(),
        );
        assert_eq!(token_manager.client_id, "client_id");
        assert_eq!(token_manager.client_secret, "client_secret");
        assert_eq!(token_manager.redirect_uri, "redirect_uri");
        assert_eq!(token_manager.auth_code, "auth_code");
    }
}
