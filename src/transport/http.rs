use crate::domain::error::SearchError;
use async_trait::async_trait;

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get_json<T>(
        &self,
        url: &str,
        headers: Vec<(String, String)>,
        query: Vec<(String, String)>,
    ) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send;
}

use reqwest::Client;

pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn get_json<T>(
        &self,
        url: &str,
        headers: Vec<(String, String)>,
        query: Vec<(String, String)>,
    ) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        let mut req = self.client.get(url);
        for (k, v) in headers {
            req = req.header(k, v);
        }
        req = req.query(&query);
        let resp = req
            .send()
            .await
            .map_err(|e| SearchError::Transport(e.to_string()))?;

        if resp.status() == 401 || resp.status() == 403 {
            return Err(SearchError::Auth);
        }
        if resp.status() == 429 {
            return Err(SearchError::RateLimited);
        }
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(SearchError::Provider(format!("HTTP {}: {}", status, text)));
        }

        resp.json::<T>()
            .await
            .map_err(|e| SearchError::Decode(e.to_string()))
    }
}
