use crate::domain::types::SearchType;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
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

#[derive(Debug, Clone, ValueEnum)]
pub enum CliSafeSearch {
    Off,
    Moderate,
    Strict,
}

impl From<CliSafeSearch> for crate::domain::types::SafeSearch {
    fn from(val: CliSafeSearch) -> Self {
        match val {
            CliSafeSearch::Off => crate::domain::types::SafeSearch::Off,
            CliSafeSearch::Moderate => crate::domain::types::SafeSearch::Moderate,
            CliSafeSearch::Strict => crate::domain::types::SafeSearch::Strict,
        }
    }
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
