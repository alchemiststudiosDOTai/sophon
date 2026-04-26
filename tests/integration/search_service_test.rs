use async_trait::async_trait;
use sophon_cli::app::fanout_search_service::FanoutSearchService;
use sophon_cli::app::search_service::SearchService;
use sophon_cli::domain::error::SearchError;
use sophon_cli::domain::provider::{ProviderCapabilities, SearchProvider};
use sophon_cli::domain::query::SearchQuery;
use sophon_cli::domain::result::{SearchResponse, SearchResult, WebResult};
use sophon_cli::domain::types::SearchType;

struct MockProvider {
    id: String,
    response: SearchResponse,
}

#[async_trait]
impl SearchProvider for MockProvider {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            web: true,
            ..ProviderCapabilities::default()
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
        Ok(SearchResponse {
            query: query.text.clone(),
            provider: self.id.clone(),
            results: self.response.results.clone(),
            total_estimated: self.response.total_estimated,
            next_page: self.response.next_page.clone(),
        })
    }
}

fn query() -> SearchQuery {
    SearchQuery {
        text: "distributed systems".to_string(),
        search_type: SearchType::Web,
        limit: Some(10),
        offset: None,
        safe_search: None,
        country: None,
        language: None,
        time_range: None,
    }
}

#[tokio::test]
async fn search_service_routes_query_to_provider() {
    let provider = MockProvider {
        id: "mock".to_string(),
        response: SearchResponse {
            query: "test".to_string(),
            provider: "mock".to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        },
    };
    let service = SearchService::new(Box::new(provider));
    let result = service.search(query()).await.unwrap();
    assert_eq!(result.query, "distributed systems");
    assert_eq!(result.provider, "mock");
}

#[tokio::test]
async fn search_service_preserves_result_count() {
    let provider = MockProvider {
        id: "brave".to_string(),
        response: SearchResponse {
            query: "test".to_string(),
            provider: "brave".to_string(),
            results: vec![
                SearchResult::Web(WebResult {
                    title: "First".to_string(),
                    url: "https://example.com/1".to_string(),
                    snippet: Some("snippet one".to_string()),
                    display_url: None,
                }),
                SearchResult::Web(WebResult {
                    title: "Second".to_string(),
                    url: "https://example.com/2".to_string(),
                    snippet: None,
                    display_url: None,
                }),
            ],
            total_estimated: Some(2),
            next_page: None,
        },
    };
    let service = SearchService::new(Box::new(provider));
    let result = service.search(query()).await.unwrap();
    assert_eq!(result.results.len(), 2);
    assert_eq!(result.total_estimated, Some(2));
}

#[tokio::test]
async fn fanout_service_aggregates_multiple_providers() {
    let brave = MockProvider {
        id: "brave".to_string(),
        response: SearchResponse {
            query: "test".to_string(),
            provider: "brave".to_string(),
            results: vec![],
            total_estimated: Some(100),
            next_page: None,
        },
    };
    let exa = MockProvider {
        id: "exa".to_string(),
        response: SearchResponse {
            query: "test".to_string(),
            provider: "exa".to_string(),
            results: vec![],
            total_estimated: Some(50),
            next_page: None,
        },
    };
    let service = FanoutSearchService::new(vec![Box::new(brave), Box::new(exa)]);
    let batch = service.search_all(query()).await;
    assert_eq!(batch.query, "distributed systems");
    assert_eq!(batch.responses.len(), 2);
    assert!(batch.failures.is_empty());
    assert_eq!(batch.responses[0].provider, "brave");
    assert_eq!(batch.responses[0].total_estimated, Some(100));
    assert_eq!(batch.responses[1].provider, "exa");
    assert_eq!(batch.responses[1].total_estimated, Some(50));
}

#[tokio::test]
async fn fanout_service_records_failures_without_short_circuiting() {
    struct FailingProvider {
        id: String,
    }

    #[async_trait]
    impl SearchProvider for FailingProvider {
        fn id(&self) -> String {
            self.id.clone()
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::default()
        }

        async fn search(&self, _query: &SearchQuery) -> Result<SearchResponse, SearchError> {
            Err(SearchError::Provider("network timeout".to_string()))
        }
    }

    let working = MockProvider {
        id: "working".to_string(),
        response: SearchResponse {
            query: "test".to_string(),
            provider: "working".to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        },
    };
    let broken = FailingProvider {
        id: "broken".to_string(),
    };
    let service = FanoutSearchService::new(vec![Box::new(working), Box::new(broken)]);
    let batch = service.search_all(query()).await;
    assert_eq!(batch.responses.len(), 1);
    assert_eq!(batch.failures.len(), 1);
    assert_eq!(batch.responses[0].provider, "working");
    assert_eq!(batch.failures[0].provider, "broken");
}

#[tokio::test]
async fn fanout_service_preserves_stable_order() {
    let providers: Vec<Box<dyn SearchProvider>> = vec![
        Box::new(MockProvider {
            id: "alpha".to_string(),
            response: SearchResponse {
                query: "test".to_string(),
                provider: "alpha".to_string(),
                results: vec![],
                total_estimated: None,
                next_page: None,
            },
        }),
        Box::new(MockProvider {
            id: "beta".to_string(),
            response: SearchResponse {
                query: "test".to_string(),
                provider: "beta".to_string(),
                results: vec![],
                total_estimated: None,
                next_page: None,
            },
        }),
        Box::new(MockProvider {
            id: "gamma".to_string(),
            response: SearchResponse {
                query: "test".to_string(),
                provider: "gamma".to_string(),
                results: vec![],
                total_estimated: None,
                next_page: None,
            },
        }),
    ];
    let service = FanoutSearchService::new(providers);
    let batch = service.search_all(query()).await;
    let ids: Vec<&str> = batch
        .responses
        .iter()
        .map(|r| r.provider.as_str())
        .collect();
    assert_eq!(ids, vec!["alpha", "beta", "gamma"]);
}
