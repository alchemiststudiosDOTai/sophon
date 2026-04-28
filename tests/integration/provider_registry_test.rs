use async_trait::async_trait;
use sophon_cli::bootstrap::provider_registry::{
    BuildSearchServiceError, ProviderBuilder, ProviderId, ProviderRegistry,
};
use sophon_cli::domain::{
    ProviderCapabilities, SearchError, SearchProvider, SearchQuery, SearchResponse, SearchType,
};
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard, OnceLock};

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

struct EnvGuard {
    _lock: MutexGuard<'static, ()>,
    brave: Option<OsString>,
    exa: Option<OsString>,
}

impl EnvGuard {
    fn set(brave: Option<&str>, exa: Option<&str>) -> Self {
        let lock = env_lock().lock().unwrap();
        let guard = Self {
            _lock: lock,
            brave: std::env::var_os("BRAVE_API_KEY"),
            exa: std::env::var_os("EXA_API_KEY"),
        };

        set_env_var("BRAVE_API_KEY", brave);
        set_env_var("EXA_API_KEY", exa);
        guard
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        restore_saved_env_var("BRAVE_API_KEY", self.brave.take());
        restore_saved_env_var("EXA_API_KEY", self.exa.take());
    }
}

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn set_env_var(name: &str, value: Option<&str>) {
    match value {
        Some(value) => unsafe {
            std::env::set_var(name, value);
        },
        None => unsafe {
            std::env::remove_var(name);
        },
    }
}

fn restore_saved_env_var(name: &str, saved_value: Option<OsString>) {
    if let Some(saved_value) = saved_value {
        unsafe { std::env::set_var(name, saved_value) };
    } else {
        unsafe { std::env::remove_var(name) };
    }
}

fn provider_builder(name: &'static str) -> ProviderBuilder {
    Box::new(move || Box::new(NamedProvider { name }))
}

fn search_query(text: &str) -> SearchQuery {
    SearchQuery {
        text: text.to_string(),
        search_type: SearchType::Web,
        limit: None,
        offset: None,
        safe_search: None,
        country: None,
        language: None,
        time_range: None,
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
    registry.register(ProviderId::Brave, provider_builder("stub"));

    assert_eq!(registry.available_providers(), vec![ProviderId::Brave]);
}

#[test]
fn available_providers_returned_in_stable_order() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Exa, provider_builder("exa"));
    registry.register(ProviderId::Brave, provider_builder("brave"));

    assert_eq!(
        registry.available_providers(),
        vec![ProviderId::Brave, ProviderId::Exa]
    );
}

#[test]
fn production_registry_without_keys_has_no_providers() {
    let _env = EnvGuard::set(None, None);

    let registry = ProviderRegistry::production_from_env();

    assert!(registry.available_providers().is_empty());
}

#[test]
fn production_registry_treats_empty_keys_as_unconfigured() {
    let _env = EnvGuard::set(Some("   "), Some(""));

    let registry = ProviderRegistry::production_from_env();

    assert!(registry.available_providers().is_empty());
}

#[test]
fn production_registry_includes_brave_only_when_only_brave_key_is_set() {
    let _env = EnvGuard::set(Some("test-brave-key"), None);

    let registry = ProviderRegistry::production_from_env();

    assert_eq!(registry.available_providers(), vec![ProviderId::Brave]);
}

#[test]
fn production_registry_includes_exa_only_when_only_exa_key_is_set() {
    let _env = EnvGuard::set(None, Some("test-exa-key"));

    let registry = ProviderRegistry::production_from_env();

    assert_eq!(registry.available_providers(), vec![ProviderId::Exa]);
}

#[test]
fn production_registry_includes_both_env_configured_providers_in_stable_order() {
    let _env = EnvGuard::set(Some("test-brave-key"), Some("test-exa-key"));

    let registry = ProviderRegistry::production_from_env();

    assert_eq!(
        registry.available_providers(),
        vec![ProviderId::Brave, ProviderId::Exa]
    );
}

#[test]
fn production_registry_reports_explicit_provider_unavailable_when_only_other_provider_exists() {
    let _env = EnvGuard::set(None, Some("test-exa-key"));

    let registry = ProviderRegistry::production_from_env();
    let result = registry.build(ProviderId::Brave);

    match result {
        Err(BuildSearchServiceError::ProviderUnavailable {
            provider,
            available,
        }) => {
            assert_eq!(provider, ProviderId::Brave);
            assert_eq!(available, vec![ProviderId::Exa]);
        }
        other => panic!(
            "expected ProviderUnavailable error, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[tokio::test]
async fn registered_provider_builds_service_that_searches() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Brave, provider_builder("stub"));

    let service = registry.build(ProviderId::Brave).expect("build succeeds");
    let result = service
        .search(search_query("integration test"))
        .await
        .unwrap();

    assert_eq!(result.query, "integration test");
    assert_eq!(result.provider, "stub");
}

#[tokio::test]
async fn build_all_enabled_uses_stable_order() {
    let mut registry = ProviderRegistry::empty();
    registry.register(ProviderId::Exa, provider_builder("exa"));
    registry.register(ProviderId::Brave, provider_builder("brave"));

    let service = registry.build_all_enabled().expect("build succeeds");
    let batch = service.search_all(search_query("rust")).await;

    assert!(batch.failures.is_empty());
    assert_eq!(batch.responses.len(), 2);
    assert_eq!(batch.responses[0].provider, "brave");
    assert_eq!(batch.responses[1].provider, "exa");
}
