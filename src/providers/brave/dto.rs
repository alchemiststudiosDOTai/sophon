use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BraveWebResponse {
    pub query: Option<BraveQuery>,
    pub web: Option<BraveWebResults>,
}

#[derive(Debug, Deserialize)]
pub struct BraveQuery {
    pub original: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BraveWebResults {
    pub results: Option<Vec<BraveWebResult>>,
    pub total: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct BraveWebResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub display_url: Option<String>,
}

// News
#[derive(Debug, Deserialize)]
pub struct BraveNewsResponse {
    pub query: Option<BraveQuery>,
    pub news: Option<BraveNewsResults>,
}

#[derive(Debug, Deserialize)]
pub struct BraveNewsResults {
    pub results: Option<Vec<BraveNewsResult>>,
}

#[derive(Debug, Deserialize)]
pub struct BraveNewsResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub source: Option<String>,
    pub age: Option<String>,
}

// Images
#[derive(Debug, Deserialize)]
pub struct BraveImagesResponse {
    pub query: Option<BraveQuery>,
    pub image_results: Option<Vec<BraveImageResult>>,
}

#[derive(Debug, Deserialize)]
pub struct BraveImageResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub thumbnail: Option<BraveThumbnail>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BraveThumbnail {
    pub src: Option<String>,
}

// Videos
#[derive(Debug, Deserialize)]
pub struct BraveVideosResponse {
    pub query: Option<BraveQuery>,
    pub videos: Option<BraveVideosResults>,
}

#[derive(Debug, Deserialize)]
pub struct BraveVideosResults {
    pub results: Option<Vec<BraveVideoResult>>,
}

#[derive(Debug, Deserialize)]
pub struct BraveVideoResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub thumbnail: Option<BraveThumbnail>,
    pub duration: Option<String>,
    pub age: Option<String>,
}
