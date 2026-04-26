use crate::domain::error::SearchError;
use crate::domain::provider::{ProviderCapabilities, SearchProvider};
use crate::domain::query::SearchQuery;
use crate::domain::result::SearchResponse;
use crate::domain::types::SearchType;
use crate::providers::brave::config::BraveConfig;
use crate::providers::brave::dto::*;
use crate::providers::brave::mapper::*;
use crate::transport::http::HttpClient;
use async_trait::async_trait;

pub struct BraveProvider<C: HttpClient> {
    client: C,
    config: BraveConfig,
}

impl<C: HttpClient> BraveProvider<C> {
    pub fn new(client: C, config: BraveConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl<C: HttpClient> SearchProvider for BraveProvider<C> {
    fn id(&self) -> String {
        "brave".to_string()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            web: true,
            news: true,
            images: true,
            videos: true,
            pagination: false,
            safe_search: true,
            time_range_filter: true,
        }
    }

    #[tracing::instrument(skip(self), fields(query = %query.text, search_type = ?query.search_type))]
    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
        let endpoint = match query.search_type {
            SearchType::Web => "web/search",
            SearchType::News => "news/search",
            SearchType::Images => "images/search",
            SearchType::Videos => "videos/search",
        };
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let mut params: Vec<(String, String)> = vec![("q".to_string(), query.text.clone())];
        if let Some(limit) = query.limit {
            params.push(("count".to_string(), limit.to_string()));
        }
        if let Some(offset) = query.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(ss) = query.safe_search {
            let val = match ss {
                crate::domain::types::SafeSearch::Off => "off",
                crate::domain::types::SafeSearch::Moderate => "moderate",
                crate::domain::types::SafeSearch::Strict => "strict",
            };
            params.push(("safesearch".to_string(), val.to_string()));
        }
        if let Some(ref country) = query.country {
            params.push(("country".to_string(), country.clone()));
        }
        if let Some(ref lang) = query.language {
            params.push(("search_lang".to_string(), lang.clone()));
        }
        if let Some(ref tr) = query.time_range {
            let val = match tr {
                crate::domain::types::TimeRange::Day => "day",
                crate::domain::types::TimeRange::Week => "week",
                crate::domain::types::TimeRange::Month => "month",
                crate::domain::types::TimeRange::Year => "year",
            };
            params.push(("freshness".to_string(), val.to_string()));
        }

        let headers = vec![
            ("Accept".to_string(), "application/json".to_string()),
            (
                "X-Subscription-Token".to_string(),
                self.config.api_key.clone(),
            ),
        ];

        match query.search_type {
            SearchType::Web => {
                let dto: BraveWebResponse = self.client.get_json(&url, headers, params).await?;
                Ok(map_web_response(dto))
            }
            SearchType::News => {
                let dto: BraveNewsResponse = self.client.get_json(&url, headers, params).await?;
                Ok(map_news_response(dto))
            }
            SearchType::Images => {
                let dto: BraveImagesResponse = self.client.get_json(&url, headers, params).await?;
                Ok(map_images_response(dto))
            }
            SearchType::Videos => {
                let dto: BraveVideosResponse = self.client.get_json(&url, headers, params).await?;
                Ok(map_videos_response(dto))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::types::{SafeSearch, SearchType};
    use crate::transport::http::HttpClient;
    use async_trait::async_trait;
    use serde::Serialize;

    struct MockHttpClient {
        response_json: String,
    }

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn get_json<T>(
            &self,
            _url: &str,
            headers: Vec<(String, String)>,
            query: Vec<(String, String)>,
        ) -> Result<T, SearchError>
        where
            T: serde::de::DeserializeOwned + Send,
        {
            assert!(headers.iter().any(|(k, _)| k == "X-Subscription-Token"));
            assert!(query.iter().any(|(k, _)| k == "q"));
            assert!(query.iter().any(|(k, _)| k == "count"));
            assert!(query.iter().any(|(k, _)| k == "safesearch"));
            serde_json::from_str(&self.response_json)
                .map_err(|e| SearchError::Decode(e.to_string()))
        }

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
            panic!("unexpected POST request");
        }
    }

    #[tokio::test]
    async fn test_brave_provider_web_search() {
        let json = r#"{
            "query": {"original": "rust"},
            "web": {
                "results": [
                    {"title": "Rust", "url": "https://rust-lang.org", "description": "Safe systems"}
                ],
                "total": 100
            }
        }"#;
        let client = MockHttpClient {
            response_json: json.to_string(),
        };
        let config = BraveConfig {
            api_key: "test-key".to_string(),
            base_url: "https://api.search.brave.com/res/v1".to_string(),
        };
        let provider = BraveProvider::new(client, config);
        let query = SearchQuery {
            text: "rust".to_string(),
            search_type: SearchType::Web,
            limit: Some(10),
            offset: None,
            safe_search: Some(SafeSearch::Moderate),
            country: None,
            language: None,
            time_range: None,
        };
        let resp = provider.search(&query).await.unwrap();
        assert_eq!(resp.query, "rust");
        assert_eq!(resp.provider, "brave");
        assert_eq!(resp.total_estimated, Some(100));
        assert_eq!(resp.results.len(), 1);
        match &resp.results[0] {
            crate::domain::result::SearchResult::Web(r) => {
                assert_eq!(r.title, "Rust");
                assert_eq!(r.url, "https://rust-lang.org");
            }
            _ => panic!("expected web result"),
        }
    }
}
