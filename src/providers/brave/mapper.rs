use crate::domain::{
    ImageResult, NewsResult, SearchResponse, SearchResult, VideoResult, WebResult,
};

use super::dto::{BraveImagesResponse, BraveNewsResponse, BraveVideosResponse, BraveWebResponse};

pub fn map_web_response(dto: BraveWebResponse) -> SearchResponse {
    let query_text = dto.query.and_then(|q| q.original).unwrap_or_default();
    let total_estimated = dto.web.as_ref().and_then(|w| w.total);
    let results = dto.web.and_then(|w| w.results).unwrap_or_default();
    SearchResponse {
        query: query_text,
        provider: "brave".to_string(),
        total_estimated,
        next_page: None,
        results: results
            .into_iter()
            .map(|r| {
                SearchResult::Web(WebResult {
                    title: r.title.unwrap_or_default(),
                    url: r.url.unwrap_or_default(),
                    snippet: r.description,
                    display_url: r.display_url,
                })
            })
            .collect(),
    }
}

pub fn map_news_response(dto: BraveNewsResponse) -> SearchResponse {
    let query_text = dto.query.and_then(|q| q.original).unwrap_or_default();
    let results = dto.news.and_then(|n| n.results).unwrap_or_default();
    SearchResponse {
        query: query_text,
        provider: "brave".to_string(),
        total_estimated: None,
        next_page: None,
        results: results
            .into_iter()
            .map(|r| {
                SearchResult::News(NewsResult {
                    title: r.title.unwrap_or_default(),
                    url: r.url.unwrap_or_default(),
                    snippet: r.description,
                    source: r.source,
                    published_at: r.age,
                })
            })
            .collect(),
    }
}

pub fn map_images_response(dto: BraveImagesResponse) -> SearchResponse {
    let query_text = dto.query.and_then(|q| q.original).unwrap_or_default();
    let results = dto.image_results.unwrap_or_default();
    SearchResponse {
        query: query_text,
        provider: "brave".to_string(),
        total_estimated: None,
        next_page: None,
        results: results
            .into_iter()
            .map(|r| {
                SearchResult::Image(ImageResult {
                    title: r.title.unwrap_or_default(),
                    url: r.url.unwrap_or_default(),
                    thumbnail_url: r.thumbnail.and_then(|t| t.src),
                    source: r.source,
                })
            })
            .collect(),
    }
}

pub fn map_videos_response(dto: BraveVideosResponse) -> SearchResponse {
    let query_text = dto.query.and_then(|q| q.original).unwrap_or_default();
    let results = dto.videos.and_then(|v| v.results).unwrap_or_default();
    SearchResponse {
        query: query_text,
        provider: "brave".to_string(),
        total_estimated: None,
        next_page: None,
        results: results
            .into_iter()
            .map(|r| {
                SearchResult::Video(VideoResult {
                    title: r.title.unwrap_or_default(),
                    url: r.url.unwrap_or_default(),
                    thumbnail_url: r.thumbnail.and_then(|t| t.src),
                    duration: r.duration,
                    published_at: r.age,
                })
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::dto::{
        BraveImageResult, BraveImagesResponse, BraveNewsResponse, BraveNewsResult,
        BraveNewsResults, BraveQuery, BraveThumbnail, BraveVideoResult, BraveVideosResponse,
        BraveVideosResults, BraveWebResponse, BraveWebResult, BraveWebResults,
    };
    use super::*;

    #[test]
    fn test_map_web_response() {
        let dto = BraveWebResponse {
            query: Some(BraveQuery {
                original: Some("rust".to_string()),
            }),
            web: Some(BraveWebResults {
                results: Some(vec![BraveWebResult {
                    title: Some("Rust Lang".to_string()),
                    url: Some("https://rust-lang.org".to_string()),
                    description: Some("The Rust language".to_string()),
                    display_url: Some("rust-lang.org".to_string()),
                }]),
                total: Some(42),
            }),
        };
        let resp = map_web_response(dto);
        assert_eq!(resp.query, "rust");
        assert_eq!(resp.provider, "brave");
        assert_eq!(resp.total_estimated, Some(42));
        assert_eq!(resp.results.len(), 1);
        match &resp.results[0] {
            SearchResult::Web(r) => {
                assert_eq!(r.title, "Rust Lang");
                assert_eq!(r.url, "https://rust-lang.org");
                assert_eq!(r.snippet, Some("The Rust language".to_string()));
                assert_eq!(r.display_url, Some("rust-lang.org".to_string()));
            }
            _ => panic!("expected Web result"),
        }
    }

    #[test]
    fn test_map_news_response() {
        let dto = BraveNewsResponse {
            query: Some(BraveQuery {
                original: Some("rust".to_string()),
            }),
            news: Some(BraveNewsResults {
                results: Some(vec![BraveNewsResult {
                    title: Some("Rust News".to_string()),
                    url: Some("https://example.com/news".to_string()),
                    description: Some("Latest in Rust".to_string()),
                    source: Some("Example News".to_string()),
                    age: Some("2h".to_string()),
                }]),
            }),
        };
        let resp = map_news_response(dto);
        assert_eq!(resp.query, "rust");
        assert_eq!(resp.results.len(), 1);
        match &resp.results[0] {
            SearchResult::News(r) => {
                assert_eq!(r.title, "Rust News");
                assert_eq!(r.source, Some("Example News".to_string()));
                assert_eq!(r.published_at, Some("2h".to_string()));
            }
            _ => panic!("expected News result"),
        }
    }

    #[test]
    fn test_map_images_response() {
        let dto = BraveImagesResponse {
            query: Some(BraveQuery {
                original: Some("rust logo".to_string()),
            }),
            image_results: Some(vec![BraveImageResult {
                title: Some("Rust Logo".to_string()),
                url: Some("https://example.com/img.png".to_string()),
                thumbnail: Some(BraveThumbnail {
                    src: Some("https://example.com/thumb.png".to_string()),
                }),
                source: Some("Example".to_string()),
            }]),
        };
        let resp = map_images_response(dto);
        assert_eq!(resp.query, "rust logo");
        assert_eq!(resp.results.len(), 1);
        match &resp.results[0] {
            SearchResult::Image(r) => {
                assert_eq!(r.title, "Rust Logo");
                assert_eq!(
                    r.thumbnail_url,
                    Some("https://example.com/thumb.png".to_string())
                );
            }
            _ => panic!("expected Image result"),
        }
    }

    #[test]
    fn test_map_videos_response() {
        let dto = BraveVideosResponse {
            query: Some(BraveQuery {
                original: Some("rust tutorial".to_string()),
            }),
            videos: Some(BraveVideosResults {
                results: Some(vec![BraveVideoResult {
                    title: Some("Learn Rust".to_string()),
                    url: Some("https://example.com/video".to_string()),
                    thumbnail: Some(BraveThumbnail {
                        src: Some("https://example.com/vthumb.png".to_string()),
                    }),
                    duration: Some("10:00".to_string()),
                    age: Some("1d".to_string()),
                }]),
            }),
        };
        let resp = map_videos_response(dto);
        assert_eq!(resp.query, "rust tutorial");
        assert_eq!(resp.results.len(), 1);
        match &resp.results[0] {
            SearchResult::Video(r) => {
                assert_eq!(r.title, "Learn Rust");
                assert_eq!(r.duration, Some("10:00".to_string()));
                assert_eq!(r.published_at, Some("1d".to_string()));
            }
            _ => panic!("expected Video result"),
        }
    }
}
