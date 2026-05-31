use std::fmt;

use crate::domain::SearchProvider;
use crate::providers::{
    brave::{client::BraveProvider, config::BraveConfig},
    exa::{client::ExaProvider, config::ExaConfig},
};
use crate::transport::http::ReqwestHttpClient;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderId {
    Brave,
    Exa,
}

impl ProviderId {
    pub fn cli_token(self) -> &'static str {
        provider_entry(self).cli_token()
    }

    pub fn display_name(self) -> &'static str {
        provider_entry(self).display_name()
    }

    pub fn env_var_name(self) -> &'static str {
        provider_entry(self).env_var_name()
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.cli_token())
    }
}

pub type ProviderBuilder = Box<dyn Fn() -> Box<dyn SearchProvider> + Send + Sync>;

#[derive(Clone, Copy)]
pub struct ProviderCatalogEntry {
    id: ProviderId,
    cli_token: &'static str,
    display_name: &'static str,
    env_var_name: &'static str,
    production_builder: fn() -> Result<ProviderBuilder, std::env::VarError>,
}

impl ProviderCatalogEntry {
    pub fn id(self) -> ProviderId {
        self.id
    }

    pub fn cli_token(self) -> &'static str {
        self.cli_token
    }

    pub fn display_name(self) -> &'static str {
        self.display_name
    }

    pub fn env_var_name(self) -> &'static str {
        self.env_var_name
    }

    pub fn production_builder(self) -> Result<ProviderBuilder, std::env::VarError> {
        (self.production_builder)()
    }
}

pub static PROVIDER_CATALOG: &[ProviderCatalogEntry] = &[
    ProviderCatalogEntry {
        id: ProviderId::Brave,
        cli_token: "brave",
        display_name: "Brave Search",
        env_var_name: "BRAVE_API_KEY",
        production_builder: build_brave_from_env,
    },
    ProviderCatalogEntry {
        id: ProviderId::Exa,
        cli_token: "exa",
        display_name: "Exa",
        env_var_name: "EXA_API_KEY",
        production_builder: build_exa_from_env,
    },
];

pub fn provider_catalog() -> &'static [ProviderCatalogEntry] {
    PROVIDER_CATALOG
}

pub fn default_provider_id() -> ProviderId {
    provider_catalog()[0].id()
}

pub fn find_provider_by_cli_token(token: &str) -> Option<ProviderId> {
    provider_catalog()
        .iter()
        .find(|entry| entry.cli_token() == token)
        .map(|entry| entry.id())
}

pub fn provider_cli_tokens() -> impl Iterator<Item = &'static str> {
    provider_catalog().iter().map(|entry| entry.cli_token())
}

pub fn provider_display_names() -> impl Iterator<Item = &'static str> {
    provider_catalog().iter().map(|entry| entry.display_name())
}

pub fn provider_env_var_hint() -> String {
    let env_vars = provider_catalog()
        .iter()
        .map(|entry| entry.env_var_name())
        .collect::<Vec<_>>();

    match env_vars.as_slice() {
        [] => "a provider API key".to_string(),
        [only] => (*only).to_string(),
        [first, second] => format!("{first} and/or {second}"),
        [head @ .., last] => format!("{}, and/or {last}", head.join(", ")),
    }
}

fn provider_entry(id: ProviderId) -> &'static ProviderCatalogEntry {
    provider_catalog()
        .iter()
        .find(|entry| entry.id() == id)
        .expect("ProviderId must have a provider catalog entry")
}

fn build_brave_from_env() -> Result<ProviderBuilder, std::env::VarError> {
    let config = BraveConfig::from_env()?;
    let builder: ProviderBuilder = Box::new(move || {
        Box::new(BraveProvider::new(ReqwestHttpClient::new(), config.clone()))
            as Box<dyn SearchProvider>
    });
    Ok(builder)
}

fn build_exa_from_env() -> Result<ProviderBuilder, std::env::VarError> {
    let config = ExaConfig::from_env()?;
    let builder: ProviderBuilder = Box::new(move || {
        Box::new(ExaProvider::new(ReqwestHttpClient::new(), config.clone()))
            as Box<dyn SearchProvider>
    });
    Ok(builder)
}
