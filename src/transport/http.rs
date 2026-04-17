use crate::domain::error::SearchError;
use async_trait::async_trait;
use reqwest::{Client, Response};
use serde::Serialize;

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

    async fn post_json<T, B>(
        &self,
        _url: &str,
        _headers: Vec<(String, String)>,
        _body: B,
    ) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send,
        B: Serialize + Send,
    {
        Err(SearchError::Transport(
            "HTTP client does not implement POST JSON support".to_string(),
        ))
    }
}

pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn decode_response<T>(response: Response) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send,
    {
        if response.status() == 401 || response.status() == 403 {
            return Err(SearchError::Auth);
        }
        if response.status() == 429 {
            return Err(SearchError::RateLimited);
        }
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(SearchError::Provider(format!("HTTP {}: {}", status, text)));
        }

        response
            .json::<T>()
            .await
            .map_err(|e| SearchError::Decode(e.to_string()))
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

        Self::decode_response(resp).await
    }

    async fn post_json<T, B>(
        &self,
        url: &str,
        headers: Vec<(String, String)>,
        body: B,
    ) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send,
        B: Serialize + Send,
    {
        let mut req = self.client.post(url).json(&body);
        for (k, v) in headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| SearchError::Transport(e.to_string()))?;

        Self::decode_response(resp).await
    }
}
