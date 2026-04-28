pub mod error;
pub mod provider;
pub mod query;
pub mod result;
pub mod types;

pub use error::SearchError;
pub use provider::{ProviderCapabilities, SearchProvider};
pub use query::SearchQuery;
pub use result::{
    ImageResult, NewsResult, PageToken, ProviderSearchFailure, SearchBatchResponse, SearchResponse,
    SearchResult, VideoResult, WebResult,
};
pub use types::{SafeSearch, SearchType, TimeRange};
