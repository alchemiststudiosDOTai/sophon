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

impl Default for ReqwestHttpClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl ReqwestHttpClient {
    pub fn new() -> Self {
        Self::default()
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
    #[tracing::instrument(skip(self, headers), fields(url = %url))]
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

        tracing::debug!(status = %resp.status(), "received HTTP response");
        Self::decode_response(resp).await
    }

    #[tracing::instrument(skip(self, headers, body), fields(url = %url))]
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

        tracing::debug!(status = %resp.status(), "received HTTP response");
        Self::decode_response(resp).await
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpClient, ReqwestHttpClient};
    use serde::{Deserialize, Serialize};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct TestResponse {
        ok: bool,
    }

    #[derive(Debug, Serialize)]
    struct TestRequest {
        message: &'static str,
    }

    #[tokio::test]
    async fn test_post_json_decodes_success_response() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buf = [0_u8; 2048];
            let bytes_read = stream.read(&mut buf).await.unwrap();
            let request = String::from_utf8_lossy(&buf[..bytes_read]);

            assert!(request.starts_with("POST /exa HTTP/1.1"));
            assert!(request.contains("x-api-key: test-key"));
            assert!(request.contains("\"message\":\"hello\""));

            let response = concat!(
                "HTTP/1.1 200 OK\r\n",
                "content-type: application/json\r\n",
                "content-length: 11\r\n",
                "connection: close\r\n\r\n",
                "{\"ok\":true}"
            );

            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client = ReqwestHttpClient::new();
        let response: TestResponse = client
            .post_json(
                &format!("http://{addr}/exa"),
                vec![("x-api-key".to_string(), "test-key".to_string())],
                TestRequest { message: "hello" },
            )
            .await
            .unwrap();

        assert_eq!(response, TestResponse { ok: true });
        server.await.unwrap();
    }
}
