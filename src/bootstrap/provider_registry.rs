use std::collections::HashMap;
use std::fmt;

use crate::app::search_service::SearchService;
use crate::domain::provider::SearchProvider;
use crate::providers::brave::client::BraveProvider;
use crate::providers::brave::config::BraveConfig;
use crate::providers::exa::client::ExaProvider;
use crate::providers::exa::config::ExaConfig;
use crate::transport::http::ReqwestHttpClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderId {
    Brave,
    Exa,
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderId::Brave => formatter.write_str("brave"),
            ProviderId::Exa => formatter.write_str("exa"),
        }
    }
}

pub type ProviderBuilder = Box<dyn Fn() -> Box<dyn SearchProvider> + Send + Sync>;

pub struct ProviderRegistry {
    builders: HashMap<ProviderId, ProviderBuilder>,
}

#[derive(Debug, thiserror::Error)]
pub enum BuildSearchServiceError {
    #[error("provider `{provider}` is unavailable; configured providers: {available:?}")]
    ProviderUnavailable {
        provider: ProviderId,
        available: Vec<ProviderId>,
    },
}

impl ProviderRegistry {
    pub fn empty() -> Self {
        Self {
            builders: HashMap::new(),
        }
    }

    pub fn production_from_env() -> Self {
        let mut registry = Self::empty();

        match BraveConfig::from_env() {
            Ok(config) => {
                registry.register(
                    ProviderId::Brave,
                    Box::new(move || {
                        Box::new(BraveProvider::new(ReqwestHttpClient::new(), config.clone()))
                    }),
                );
            }
            Err(std::env::VarError::NotPresent | std::env::VarError::NotUnicode(_)) => {}
        }

        match ExaConfig::from_env() {
            Ok(config) => {
                registry.register(
                    ProviderId::Exa,
                    Box::new(move || {
                        Box::new(ExaProvider::new(ReqwestHttpClient::new(), config.clone()))
                    }),
                );
            }
            Err(std::env::VarError::NotPresent | std::env::VarError::NotUnicode(_)) => {}
        }

        registry
    }

    pub fn register(&mut self, id: ProviderId, builder: ProviderBuilder) {
        self.builders.insert(id, builder);
    }

    pub fn available_providers(&self) -> Vec<ProviderId> {
        [ProviderId::Brave, ProviderId::Exa]
            .into_iter()
            .filter(|id| self.builders.contains_key(id))
            .collect()
    }

    pub fn build(&self, provider: ProviderId) -> Result<SearchService, BuildSearchServiceError> {
        let builder =
            self.builders
                .get(&provider)
                .ok_or_else(|| BuildSearchServiceError::ProviderUnavailable {
                    provider,
                    available: self.available_providers(),
                })?;

        Ok(SearchService::new(builder()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::SearchError;
    use crate::domain::provider::ProviderCapabilities;
    use crate::domain::query::SearchQuery;
    use crate::domain::result::SearchResponse;
    use async_trait::async_trait;
    use std::ffi::OsString;
    use std::sync::{Mutex, OnceLock};

    struct MockProvider;

    #[async_trait]
    impl SearchProvider for MockProvider {
        fn id(&self) -> String {
            "mock".to_string()
        }

        fn capabilities(&self) -> ProviderCapabilities {
            ProviderCapabilities {
                web: true,
                news: true,
                images: false,
                videos: false,
                pagination: false,
                safe_search: false,
                time_range_filter: false,
            }
        }

        async fn search(&self, _query: &SearchQuery) -> Result<SearchResponse, SearchError> {
            Ok(SearchResponse {
                query: "mock".to_string(),
                provider: "mock".to_string(),
                results: vec![],
                total_estimated: None,
                next_page: None,
            })
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn restore_env_var(name: &str, value: Option<OsString>) {
        match value {
            Some(value) => unsafe {
                std::env::set_var(name, value);
            },
            None => unsafe {
                std::env::remove_var(name);
            },
        }
    }

    #[test]
    fn empty_registry_reports_provider_unavailable() {
        let registry = ProviderRegistry::empty();

        match registry.build(ProviderId::Brave) {
            Err(BuildSearchServiceError::ProviderUnavailable {
                provider,
                available,
            }) => {
                assert_eq!(provider, ProviderId::Brave);
                assert!(available.is_empty());
            }
            Ok(_) => panic!("expected ProviderUnavailable error"),
        }
    }

    #[test]
    fn production_registry_only_includes_configured_providers() {
        let _guard = env_lock().lock().unwrap();
        let original_brave = std::env::var_os("BRAVE_API_KEY");
        let original_exa = std::env::var_os("EXA_API_KEY");

        unsafe {
            std::env::set_var("BRAVE_API_KEY", "test-brave-key");
            std::env::remove_var("EXA_API_KEY");
        }

        let registry = ProviderRegistry::production_from_env();

        restore_env_var("BRAVE_API_KEY", original_brave);
        restore_env_var("EXA_API_KEY", original_exa);

        assert_eq!(registry.available_providers(), vec![ProviderId::Brave]);
    }

    #[test]
    fn registered_provider_is_available_and_builds_service() {
        let mut registry = ProviderRegistry::empty();
        registry.register(ProviderId::Brave, Box::new(|| Box::new(MockProvider)));

        assert_eq!(registry.available_providers(), vec![ProviderId::Brave]);
        assert!(registry.build(ProviderId::Brave).is_ok());
    }

    #[test]
    fn available_providers_are_returned_in_stable_order() {
        let mut registry = ProviderRegistry::empty();
        registry.register(ProviderId::Exa, Box::new(|| Box::new(MockProvider)));
        registry.register(ProviderId::Brave, Box::new(|| Box::new(MockProvider)));

        assert_eq!(
            registry.available_providers(),
            vec![ProviderId::Brave, ProviderId::Exa]
        );
    }
}
