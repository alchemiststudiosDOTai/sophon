use super::{SafeSearch, SearchType, TimeRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    pub text: String,
    pub search_type: SearchType,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub safe_search: Option<SafeSearch>,
    pub country: Option<String>,
    pub language: Option<String>,
    pub time_range: Option<TimeRange>,
}
