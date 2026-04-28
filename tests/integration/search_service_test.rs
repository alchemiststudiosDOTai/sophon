use async_trait::async_trait;
use sophon_cli::app::fanout_search_service::FanoutSearchService;
use sophon_cli::app::search_service::SearchService;
use sophon_cli::domain::{
    ProviderCapabilities, SearchError, SearchProvider, SearchQuery, SearchResponse, SearchResult,
    SearchType, WebResult,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
enum ProviderOutcome {
    Success(SearchResponse),
    Failure(&'static str),
}

struct SpyProvider {
    id: &'static str,
    seen_queries: Arc<Mutex<Vec<SearchQuery>>>,
    outcome: ProviderOutcome,
}

#[async_trait]
impl SearchProvider for SpyProvider {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            web: true,
            ..ProviderCapabilities::default()
        }
    }

    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
        self.seen_queries.lock().unwrap().push(query.clone());

        match &self.outcome {
            ProviderOutcome::Success(response) => Ok(response.clone()),
            ProviderOutcome::Failure(message) => Err(SearchError::Provider((*message).to_string())),
        }
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

fn response(provider: &str, total_estimated: Option<u64>) -> SearchResponse {
    SearchResponse {
        query: "provider supplied query".to_string(),
        provider: provider.to_string(),
        results: vec![],
        total_estimated,
        next_page: None,
    }
}

fn provider(
    id: &'static str,
    outcome: ProviderOutcome,
) -> (SpyProvider, Arc<Mutex<Vec<SearchQuery>>>) {
    let seen_queries = Arc::new(Mutex::new(Vec::new()));
    (
        SpyProvider {
            id,
            seen_queries: Arc::clone(&seen_queries),
            outcome,
        },
        seen_queries,
    )
}

fn web_result(title: &str, url: &str, snippet: Option<&str>) -> SearchResult {
    SearchResult::Web(WebResult {
        title: title.to_string(),
        url: url.to_string(),
        snippet: snippet.map(str::to_string),
        display_url: None,
    })
}

#[tokio::test]
async fn search_service_passes_full_query_to_provider_and_returns_exact_response() {
    let expected_query = query();
    let expected_response = SearchResponse {
        query: "canonical provider query".to_string(),
        provider: "mock".to_string(),
        results: vec![],
        total_estimated: None,
        next_page: None,
    };
    let (provider, seen_queries) =
        provider("mock", ProviderOutcome::Success(expected_response.clone()));
    let service = SearchService::new(Box::new(provider));

    let result = service.search(expected_query.clone()).await.unwrap();

    assert_eq!(result, expected_response);
    assert_eq!(*seen_queries.lock().unwrap(), vec![expected_query]);
}

#[tokio::test]
async fn search_service_propagates_provider_errors() {
    let (provider, seen_queries) = provider("broken", ProviderOutcome::Failure("network timeout"));
    let expected_query = query();
    let service = SearchService::new(Box::new(provider));

    let error = service
        .search(expected_query.clone())
        .await
        .expect_err("provider error should propagate");

    match error {
        SearchError::Provider(message) => assert_eq!(message, "network timeout"),
        other => panic!("expected provider error, got {other}"),
    }
    assert_eq!(*seen_queries.lock().unwrap(), vec![expected_query]);
}

#[tokio::test]
async fn search_service_preserves_result_count() {
    let provider_response = SearchResponse {
        query: "provider supplied query".to_string(),
        provider: "brave".to_string(),
        results: vec![
            web_result("First", "https://example.com/1", Some("snippet one")),
            web_result("Second", "https://example.com/2", None),
        ],
        total_estimated: Some(2),
        next_page: None,
    };
    let (provider, _) = provider("brave", ProviderOutcome::Success(provider_response));
    let service = SearchService::new(Box::new(provider));

    let result = service.search(query()).await.unwrap();

    assert_eq!(result.results.len(), 2);
    assert_eq!(result.total_estimated, Some(2));
}

#[tokio::test]
async fn fanout_service_aggregates_multiple_providers() {
    let (brave, _) = provider(
        "brave",
        ProviderOutcome::Success(response("brave", Some(100))),
    );
    let (exa, _) = provider("exa", ProviderOutcome::Success(response("exa", Some(50))));
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
    let (working, _) = provider(
        "working",
        ProviderOutcome::Success(response("working", None)),
    );
    let (broken, _) = provider("broken", ProviderOutcome::Failure("network timeout"));
    let service = FanoutSearchService::new(vec![Box::new(working), Box::new(broken)]);

    let batch = service.search_all(query()).await;

    assert_eq!(batch.responses.len(), 1);
    assert_eq!(batch.failures.len(), 1);
    assert_eq!(batch.responses[0].provider, "working");
    assert_eq!(batch.failures[0].provider, "broken");
}

#[tokio::test]
async fn fanout_service_preserves_stable_order() {
    let providers: Vec<Box<dyn SearchProvider>> = ["alpha", "beta", "gamma"]
        .into_iter()
        .map(|id| {
            let (provider, _) = provider(id, ProviderOutcome::Success(response(id, None)));
            Box::new(provider) as Box<dyn SearchProvider>
        })
        .collect();
    let service = FanoutSearchService::new(providers);

    let batch = service.search_all(query()).await;
    let ids: Vec<&str> = batch
        .responses
        .iter()
        .map(|response| response.provider.as_str())
        .collect();

    assert_eq!(ids, vec!["alpha", "beta", "gamma"]);
}
