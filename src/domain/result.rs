use crate::domain::error::SearchError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageToken(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResponse {
    pub query: String,
    pub provider: String,
    pub results: Vec<SearchResult>,
    pub total_estimated: Option<u64>,
    pub next_page: Option<PageToken>,
}

#[derive(Debug)]
pub struct SearchBatchResponse {
    pub query: String,
    pub responses: Vec<SearchResponse>,
    pub failures: Vec<ProviderSearchFailure>,
}

#[derive(Debug)]
pub struct ProviderSearchFailure {
    pub provider: String,
    pub error: SearchError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Web(WebResult),
    News(NewsResult),
    Image(ImageResult),
    Video(VideoResult),
}

#[derive(Debug, Clone, PartialEq)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub display_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewsResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub source: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageResult {
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoResult {
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub published_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::SearchError;

    #[test]
    fn search_batch_response_can_hold_success_and_failure() {
        let response = SearchResponse {
            query: "rust".to_string(),
            provider: "brave".to_string(),
            results: vec![],
            total_estimated: None,
            next_page: None,
        };
        let failure = ProviderSearchFailure {
            provider: "exa".to_string(),
            error: SearchError::InvalidQuery("unsupported".to_string()),
        };
        let batch = SearchBatchResponse {
            query: "rust".to_string(),
            responses: vec![response],
            failures: vec![failure],
        };

        assert_eq!(batch.query, "rust");
        assert_eq!(batch.responses[0].provider, "brave");
        assert_eq!(batch.failures[0].provider, "exa");
        assert_eq!(
            batch.failures[0].error.to_string(),
            "invalid query: unsupported"
        );
    }
}
