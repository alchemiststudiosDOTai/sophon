use crate::domain::{
    ProviderSearchFailure, SearchBatchResponse, SearchProvider, SearchQuery, SearchResponse,
};

pub struct FanoutSearchService {
    providers: Vec<Box<dyn SearchProvider>>,
}

impl FanoutSearchService {
    pub fn new(providers: Vec<Box<dyn SearchProvider>>) -> Self {
        Self { providers }
    }

    pub async fn search_all(&self, query: SearchQuery) -> SearchBatchResponse {
        let mut responses: Vec<SearchResponse> = Vec::new();
        let mut failures: Vec<ProviderSearchFailure> = Vec::new();

        for provider in &self.providers {
            let provider_id = provider.id();
            match provider.search(&query).await {
                Ok(response) => responses.push(response),
                Err(error) => failures.push(ProviderSearchFailure {
                    provider: provider_id,
                    error,
                }),
            }
        }

        SearchBatchResponse {
            query: query.text,
            responses,
            failures,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        ProviderCapabilities, SearchError, SearchProvider, SearchQuery, SearchResponse, SearchType,
    };
    use async_trait::async_trait;

    enum MockOutcome {
        Success(SearchResponse),
        Failure(&'static str),
    }

    struct MockProvider {
        id: &'static str,
        outcome: MockOutcome,
    }

    #[async_trait]
    impl SearchProvider for MockProvider {
        fn id(&self) -> String {
            self.id.to_string()
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                web: true,
                ..ProviderCapabilities::default()
            }
        }

        async fn search(&self, _query: &SearchQuery) -> Result<SearchResponse, SearchError> {
            match &self.outcome {
                MockOutcome::Success(response) => Ok(response.clone()),
                MockOutcome::Failure(message) => Err(SearchError::Provider(message.to_string())),
            }
        }
    }

    fn response(provider: &str) -> SearchResponse {
        SearchResponse {
            query: "rust".to_string(),
            provider: provider.to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        }
    }

    fn query() -> SearchQuery {
        SearchQuery {
            text: "rust".to_string(),
            search_type: SearchType::Web,
            limit: None,
            offset: None,
            safe_search: None,
            country: None,
            language: None,
            time_range: None,
        }
    }

    #[tokio::test]
    async fn search_all_preserves_provider_order_and_records_failures() {
        let service = FanoutSearchService::new(vec![
            Box::new(MockProvider {
                id: "brave",
                outcome: MockOutcome::Success(response("brave")),
            }),
            Box::new(MockProvider {
                id: "broken",
                outcome: MockOutcome::Failure("temporary outage"),
            }),
            Box::new(MockProvider {
                id: "exa",
                outcome: MockOutcome::Success(response("exa")),
            }),
        ]);

        let batch = service.search_all(query()).await;

        assert_eq!(batch.query, "rust");
        assert_eq!(batch.responses[0].provider, "brave");
        assert_eq!(batch.responses[1].provider, "exa");
        assert_eq!(batch.failures.len(), 1);
        assert_eq!(batch.failures[0].provider, "broken");
        assert_eq!(
            batch.failures[0].error.to_string(),
            "provider error: temporary outage"
        );
    }
}
