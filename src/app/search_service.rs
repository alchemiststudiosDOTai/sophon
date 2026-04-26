use crate::domain::error::SearchError;
use crate::domain::provider::SearchProvider;
use crate::domain::query::SearchQuery;
use crate::domain::result::SearchResponse;

pub struct SearchService {
    provider: Box<dyn SearchProvider>,
}

impl SearchService {
    pub fn new(provider: Box<dyn SearchProvider>) -> Self {
        Self { provider }
    }

    #[tracing::instrument(skip(self), fields(query = %query.text, provider = %self.provider.id()))]
    pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
        tracing::debug!("delegating search to provider");
        let result = self.provider.search(&query).await;
        if let Err(ref e) = result {
            tracing::warn!(error = %e, "provider search returned error");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::provider::ProviderCapabilities;
    use crate::domain::types::SearchType;
    use async_trait::async_trait;

    struct MockProvider {
        response: SearchResponse,
    }

    #[async_trait]
    impl SearchProvider for MockProvider {
        fn id(&self) -> String {
            "mock".to_string()
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                web: true,
                news: false,
                images: false,
                videos: false,
                pagination: false,
                safe_search: false,
                time_range_filter: false,
            }
        }

        async fn search(&self, _query: &SearchQuery) -> Result<SearchResponse, SearchError> {
            Ok(self.response.clone())
        }
    }

    #[tokio::test]
    async fn test_search_service_delegates() {
        let expected = SearchResponse {
            query: "test".to_string(),
            provider: "mock".to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        };
        let provider = MockProvider {
            response: expected.clone(),
        };
        let service = SearchService::new(Box::new(provider));
        let query = SearchQuery {
            text: "test".to_string(),
            search_type: SearchType::Web,
            limit: None,
            offset: None,
            safe_search: None,
            country: None,
            language: None,
            time_range: None,
        };
        let result = service.search(query).await.unwrap();
        assert_eq!(result.query, expected.query);
        assert_eq!(result.provider, expected.provider);
    }
}
