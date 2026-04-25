use std::collections::HashMap;
use std::fmt;

use crate::app::search_service::SearchService;
use crate::domain::provider::SearchProvider;

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
}
