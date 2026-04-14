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
