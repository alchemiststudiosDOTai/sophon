use crate::domain::error::SearchError;
use crate::domain::provider::{ProviderCapabilities, SearchProvider};
use crate::domain::query::SearchQuery;
use crate::domain::result::SearchResponse;
use crate::domain::types::{SafeSearch, SearchType, TimeRange};
use crate::providers::exa::config::ExaConfig;
use crate::providers::exa::dto::{ExaContentsRequest, ExaSearchRequest, ExaSearchResponse};
use crate::providers::exa::mapper::{map_news_response, map_web_response};
use crate::transport::http::HttpClient;
use async_trait::async_trait;
use chrono::{Duration, Utc};

pub struct ExaProvider<C: HttpClient> {
    client: C,
    config: ExaConfig,
}

impl<C: HttpClient> ExaProvider<C> {
    pub fn new(client: C, config: ExaConfig) -> Self {
        Self { client, config }
    }

    fn build_request(&self, query: &SearchQuery) -> Result<ExaSearchRequest, SearchError> {
        match query.search_type {
            SearchType::Images => {
                return Err(SearchError::InvalidQuery(
                    "exa provider does not support image search".to_string(),
                ));
            }
            SearchType::Videos => {
                return Err(SearchError::InvalidQuery(
                    "exa provider does not support video search".to_string(),
                ));
            }
            SearchType::Web | SearchType::News => {}
        }

        if query.offset.is_some() {
            return Err(SearchError::InvalidQuery(
                "exa provider does not support offset".to_string(),
            ));
        }

        if query.language.is_some() {
            return Err(SearchError::InvalidQuery(
                "exa provider does not support language selection".to_string(),
            ));
        }

        let (start_published_date, end_published_date) =
            published_date_window(query.time_range.as_ref());

        Ok(ExaSearchRequest {
            query: query.text.clone(),
            search_type: "auto".to_string(),
            num_results: query.limit,
            category: match query.search_type {
                SearchType::News => Some("news".to_string()),
                SearchType::Web | SearchType::Images | SearchType::Videos => None,
            },
            user_location: query.country.clone(),
            start_published_date,
            end_published_date,
            moderation: match query.safe_search {
                Some(SafeSearch::Off) => Some(false),
                // Exa exposes a boolean moderation flag, so Moderate and Strict collapse.
                Some(SafeSearch::Moderate | SafeSearch::Strict) => Some(true),
                None => None,
            },
            contents: ExaContentsRequest { text: true },
        })
    }
}

#[async_trait]
impl<C: HttpClient> SearchProvider for ExaProvider<C> {
    fn id(&self) -> String {
        "exa".to_string()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            web: true,
            news: true,
            images: false,
            videos: false,
            pagination: false,
            safe_search: true,
            time_range_filter: true,
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
        let request = self.build_request(query)?;
        let response: ExaSearchResponse = self
            .client
            .post_json(
                &format!("{}/search", self.config.base_url),
                vec![("x-api-key".to_string(), self.config.api_key.clone())],
                request,
            )
            .await?;

        match query.search_type {
            SearchType::Web => Ok(map_web_response(&query.text, response)),
            SearchType::News => Ok(map_news_response(&query.text, response)),
            SearchType::Images | SearchType::Videos => unreachable!("validated above"),
        }
    }
}

fn published_date_window(time_range: Option<&TimeRange>) -> (Option<String>, Option<String>) {
    let Some(time_range) = time_range else {
        return (None, None);
    };

    let end = Utc::now();
    let start = match time_range {
        TimeRange::Day => end - Duration::days(1),
        TimeRange::Week => end - Duration::weeks(1),
        TimeRange::Month => end - Duration::days(30),
        TimeRange::Year => end - Duration::days(365),
    };

    (Some(start.to_rfc3339()), Some(end.to_rfc3339()))
}

#[cfg(test)]
mod tests {
    use super::ExaProvider;
    use crate::domain::error::SearchError;
    use crate::domain::provider::SearchProvider;
    use crate::domain::query::SearchQuery;
    use crate::domain::result::SearchResult;
    use crate::domain::types::{SafeSearch, SearchType, TimeRange};
    use crate::providers::exa::config::ExaConfig;
    use crate::transport::http::HttpClient;
    use async_trait::async_trait;
    use serde::Serialize;
    use serde_json::Value;

    struct MockHttpClient {
        response_json: String,
    }

    #[async_trait]
    impl HttpClient for MockHttpClient {
        async fn get_json<T>(
            &self,
            _url: &str,
            _headers: Vec<(String, String)>,
            _query: Vec<(String, String)>,
        ) -> Result<T, SearchError>
        where
            T: serde::de::DeserializeOwned + Send,
        {
            panic!("unexpected GET request");
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
            assert_eq!(url, "https://api.exa.ai/search");
            assert_eq!(
                headers,
                vec![("x-api-key".to_string(), "test-key".to_string())]
            );

            let body = serde_json::to_value(body).unwrap();
            assert_eq!(body.get("category"), Some(&Value::String("news".to_string())));
            assert_eq!(body.get("numResults"), Some(&Value::from(3)));
            assert_eq!(body.get("moderation"), Some(&Value::Bool(true)));
            assert_eq!(body.get("type"), Some(&Value::String("auto".to_string())));
            assert_eq!(
                body.get("contents")
                    .and_then(|contents| contents.get("text")),
                Some(&Value::Bool(true))
            );

            let start = body
                .get("startPublishedDate")
                .and_then(Value::as_str)
                .unwrap();
            let end = body
                .get("endPublishedDate")
                .and_then(Value::as_str)
                .unwrap();
            assert!(start.contains('T'));
            assert!(end.contains('T'));

            serde_json::from_str(&self.response_json)
                .map_err(|error| SearchError::Decode(error.to_string()))
        }
    }

    #[tokio::test]
    async fn test_exa_provider_news_search_posts_expected_payload() {
        let client = MockHttpClient {
            response_json: r#"{
                "requestId": "req_123",
                "searchType": "auto",
                "results": [
                    {
                        "title": "Example headline",
                        "url": "https://example.com/news",
                        "publishedDate": "2026-04-15T00:00:00Z",
                        "author": "Example Reporter",
                        "summary": "Summary text"
                    }
                ]
            }"#
            .to_string(),
        };
        let provider = ExaProvider::new(
            client,
            ExaConfig {
                api_key: "test-key".to_string(),
                base_url: "https://api.exa.ai".to_string(),
            },
        );
        let query = SearchQuery {
            text: "ai news".to_string(),
            search_type: SearchType::News,
            limit: Some(3),
            offset: None,
            safe_search: Some(SafeSearch::Strict),
            country: Some("US".to_string()),
            language: None,
            time_range: Some(TimeRange::Week),
        };

        let response = provider.search(&query).await.unwrap();

        assert_eq!(response.query, "ai news");
        assert_eq!(response.provider, "exa");
        assert_eq!(response.results.len(), 1);
        match &response.results[0] {
            SearchResult::News(result) => {
                assert_eq!(result.title, "Example headline");
                assert_eq!(result.source.as_deref(), Some("Example Reporter"));
                assert_eq!(result.snippet.as_deref(), Some("Summary text"));
            }
            other => panic!("expected news result, got {other:?}"),
        }
    }
}
