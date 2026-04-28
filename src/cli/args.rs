use clap::{Parser, ValueEnum};

use crate::domain::{SafeSearch, SearchType};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CliProvider {
    Brave,
    Exa,
    All,
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

    #[arg(short = 'p', long, value_enum, default_value = "brave")]
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
    use clap::Parser;

    #[test]
    fn test_cli_provider_parses_exa_and_defaults_to_brave() {
        let exa_args = CliArgs::parse_from(["sophon-cli", "--provider", "exa", "rust"]);
        assert_eq!(exa_args.provider, CliProvider::Exa);
        assert_eq!(exa_args.search_type, CliSearchType::Web);

        let default_args = CliArgs::parse_from(["sophon-cli", "rust"]);
        assert_eq!(default_args.provider, CliProvider::Brave);
    }

    #[test]
    fn test_cli_provider_parses_all_and_defaults_to_brave() {
        let all_args = CliArgs::try_parse_from(["sophon-cli", "rust", "--provider", "all"])
            .expect("all provider parses");
        assert_eq!(all_args.provider, CliProvider::All);

        let default_args =
            CliArgs::try_parse_from(["sophon-cli", "rust"]).expect("default provider parses");
        assert_eq!(default_args.provider, CliProvider::Brave);
    }
}
