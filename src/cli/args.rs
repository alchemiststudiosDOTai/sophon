use std::fmt;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{Parser, ValueEnum};

use crate::bootstrap::provider_catalog::{self, ProviderId};
use crate::domain::{SafeSearch, SearchType};

const ALL_PROVIDER_TOKEN: &str = "all";

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliSearchType {
    Web,
    News,
    Images,
    Videos,
}

impl From<CliSearchType> for SearchType {
    fn from(val: CliSearchType) -> Self {
        match val {
            CliSearchType::Web => SearchType::Web,
            CliSearchType::News => SearchType::News,
            CliSearchType::Images => SearchType::Images,
            CliSearchType::Videos => SearchType::Videos,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliSafeSearch {
    Off,
    Moderate,
    Strict,
}

impl From<CliSafeSearch> for SafeSearch {
    fn from(val: CliSafeSearch) -> Self {
        match val {
            CliSafeSearch::Off => SafeSearch::Off,
            CliSafeSearch::Moderate => SafeSearch::Moderate,
            CliSafeSearch::Strict => SafeSearch::Strict,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliProvider {
    Single(ProviderId),
    All,
}

impl Default for CliProvider {
    fn default() -> Self {
        Self::Single(provider_catalog::default_provider_id())
    }
}

impl fmt::Display for CliProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliProvider::Single(provider_id) => provider_id.fmt(formatter),
            CliProvider::All => formatter.write_str(ALL_PROVIDER_TOKEN),
        }
    }
}

fn cli_provider_value_parser() -> impl TypedValueParser<Value = CliProvider> {
    PossibleValuesParser::new(valid_provider_tokens()).map(|value| parse_cli_provider(&value))
}

fn valid_provider_tokens() -> Vec<&'static str> {
    let mut tokens = provider_catalog::provider_cli_tokens().collect::<Vec<_>>();
    tokens.push(ALL_PROVIDER_TOKEN);
    tokens
}

fn parse_cli_provider(value: &str) -> CliProvider {
    if value == ALL_PROVIDER_TOKEN {
        return CliProvider::All;
    }

    CliProvider::Single(
        provider_catalog::find_provider_by_cli_token(value)
            .expect("possible provider token must resolve through provider_catalog"),
    )
}

#[derive(Parser, Debug)]
#[command(name = "sophon-cli")]
#[command(about = "Provider-agnostic search CLI")]
pub struct CliArgs {
    #[arg(help = "Search query text")]
    pub query: Option<String>,

    #[arg(long, help = "Show what sophon-cli is")]
    pub about: bool,

    #[arg(short, long, value_enum, default_value = "web")]
    pub search_type: CliSearchType,

    #[arg(short = 'p', long, value_parser = cli_provider_value_parser(), default_value_t = CliProvider::default())]
    pub provider: CliProvider,

    #[arg(short, long)]
    pub limit: Option<usize>,

    #[arg(long)]
    pub offset: Option<usize>,

    #[arg(long, value_enum)]
    pub safe_search: Option<CliSafeSearch>,

    #[arg(long)]
    pub country: Option<String>,

    #[arg(long)]
    pub language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{CliArgs, CliProvider, CliSearchType};
    use crate::bootstrap::provider_catalog::ProviderId;
    use clap::Parser;

    #[test]
    fn test_cli_provider_parses_exa_and_defaults_to_brave() {
        let exa_args = CliArgs::parse_from(["sophon-cli", "--provider", "exa", "rust"]);
        assert_eq!(exa_args.provider, CliProvider::Single(ProviderId::Exa));
        assert_eq!(exa_args.search_type, CliSearchType::Web);

        let default_args = CliArgs::parse_from(["sophon-cli", "rust"]);
        assert_eq!(
            default_args.provider,
            CliProvider::Single(ProviderId::Brave)
        );
    }

    #[test]
    fn test_cli_provider_parses_all_and_defaults_to_brave() {
        let all_args = CliArgs::try_parse_from(["sophon-cli", "rust", "--provider", "all"])
            .expect("all provider parses");
        assert_eq!(all_args.provider, CliProvider::All);

        let default_args =
            CliArgs::try_parse_from(["sophon-cli", "rust"]).expect("default provider parses");
        assert_eq!(
            default_args.provider,
            CliProvider::Single(ProviderId::Brave)
        );
    }

    #[test]
    fn test_cli_provider_rejects_unknown_provider_with_catalog_tokens() {
        let error = CliArgs::try_parse_from(["sophon-cli", "--provider", "unknown", "rust"])
            .expect_err("unknown provider should fail")
            .to_string();

        assert!(error.contains("invalid value 'unknown'"));
        assert!(error.contains("brave"));
        assert!(error.contains("exa"));
        assert!(error.contains("all"));
    }
}
