use crate::domain::error::SearchError;
use crate::domain::query::SearchQuery;
use crate::domain::result::SearchResponse;
use async_trait::async_trait;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProviderCapabilities {
    pub web: bool,
    pub news: bool,
    pub images: bool,
    pub videos: bool,
    pub pagination: bool,
    pub safe_search: bool,
    pub time_range_filter: bool,
}

#[async_trait]
pub trait SearchProvider: Send + Sync {
    #[allow(dead_code)]
    fn id(&self) -> String;
    #[allow(dead_code)]
    fn capabilities(&self) -> ProviderCapabilities;
    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError>;
}
