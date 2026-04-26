use async_trait::async_trait;
use sophon_cli::bootstrap::provider_registry::{
    BuildSearchServiceError, ProviderId, ProviderRegistry,
};
use sophon_cli::domain::error::SearchError;
use sophon_cli::domain::provider::{ProviderCapabilities, SearchProvider};
use sophon_cli::domain::query::SearchQuery;
use sophon_cli::domain::result::SearchResponse;
use sophon_cli::domain::types::SearchType;

struct StubProvider;

#[async_trait]
impl SearchProvider for StubProvider {
    fn id(&self) -> String {
        "stub".to_string()
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::default()
    }

    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
        Ok(SearchResponse {
            query: query.text.clone(),
            provider: "stub".to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        })
    }
}

#[test]
fn empty_registry_build_fails_with_provider_unavailable() {
    let registry = ProviderRegistry::empty();
    let result = registry.build(ProviderId::Brave);
    match result {
        Err(BuildSearchServiceError::ProviderUnavailable {
            provider,
            available,
        }) => {
            assert_eq!(provider, ProviderId::Brave);
            assert!(available.is_empty());
        }
        other => panic!(
            "expected ProviderUnavailable error, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn empty_registry_build_all_enabled_fails_with_no_providers() {
    let registry = ProviderRegistry::empty();
    let result = registry.build_all_enabled();
    match result {
        Err(BuildSearchServiceError::NoProvidersAvailable) => {}
        other => panic!(
            "expected NoProvidersAvailable error, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn registered_provider_is_available() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Brave, Box::new(|| Box::new(StubProvider)));

    assert_eq!(registry.available_providers(), vec![ProviderId::Brave]);
}

#[test]
fn available_providers_returned_in_stable_order() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Exa, Box::new(|| Box::new(StubProvider)));
    registry.register(ProviderId::Brave, Box::new(|| Box::new(StubProvider)));

    assert_eq!(
        registry.available_providers(),
        vec![ProviderId::Brave, ProviderId::Exa]
    );
}

#[tokio::test]
async fn registered_provider_builds_service_that_searches() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Brave, Box::new(|| Box::new(StubProvider)));

    let service = registry.build(ProviderId::Brave).expect("build succeeds");
    let query = SearchQuery {
        text: "integration test".to_string(),
        search_type: SearchType::Web,
        limit: None,
        offset: None,
        safe_search: None,
        country: None,
        language: None,
        time_range: None,
    };
    let result = service.search(query).await.unwrap();
    assert_eq!(result.query, "integration test");
    assert_eq!(result.provider, "stub");
}

#[tokio::test]
async fn build_all_enabled_uses_stable_order() {
    struct NamedProvider {
        name: &'static str,
    }

    #[async_trait]
    impl SearchProvider for NamedProvider {
        fn id(&self) -> String {
            self.name.to_string()
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities::default()
        }

        async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
            Ok(SearchResponse {
                query: query.text.clone(),
                provider: self.name.to_string(),
                results: vec![],
                total_estimated: None,
                next_page: None,
            })
        }
    }

    let mut registry = ProviderRegistry::empty();
    registry.register(
        ProviderId::Exa,
        Box::new(|| Box::new(NamedProvider { name: "exa" })),
    );
    registry.register(
        ProviderId::Brave,
        Box::new(|| Box::new(NamedProvider { name: "brave" })),
    );

    let service = registry.build_all_enabled().expect("build succeeds");
    let query = SearchQuery {
        text: "rust".to_string(),
        search_type: SearchType::Web,
        limit: None,
        offset: None,
        safe_search: None,
        country: None,
        language: None,
        time_range: None,
    };
    let batch = service.search_all(query).await;
    let providers: Vec<&str> = batch
        .responses
        .iter()
        .map(|r| r.provider.as_str())
        .collect();
    assert_eq!(providers, vec!["brave", "exa"]);
    assert!(batch.failures.is_empty());
}
