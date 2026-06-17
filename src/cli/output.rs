use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

pub fn render_text(response: &SearchResponse) -> String {
    let mut lines = vec![
        format!("Provider: {}", response.provider),
        format!("Query: {}", response.query),
        format!("Results: {}", response.results.len()),
        String::new(),
    ];
    for (i, result) in response.results.iter().enumerate() {
        match result {
            SearchResult::Web(r) => {
                lines.push(format!("{}. [{}]", i + 1, r.title));
                lines.push(format!("   URL: {}", r.url));
                if let Some(s) = &r.snippet {
                    let t = s.trim();
                    if !t.is_empty() {
                        lines.push(format!("   {}", t));
                    }
                }
            }
            SearchResult::News(r) => {
                lines.push(format!("{}. [NEWS] {}", i + 1, r.title));
                lines.push(format!("   URL: {}", r.url));
                if let Some(s) = &r.source {
                    lines.push(format!("   Source: {}", s));
                }
                if let Some(s) = &r.snippet {
                    let t = s.trim();
                    if !t.is_empty() {
                        lines.push(format!("   {}", t));
                    }
                }
            }
            SearchResult::Image(r) => {
                lines.push(format!("{}. [IMAGE] {}", i + 1, r.title));
                lines.push(format!("   URL: {}", r.url));
            }
            SearchResult::Video(r) => {
                lines.push(format!("{}. [VIDEO] {}", i + 1, r.title));
                lines.push(format!("   URL: {}", r.url));
            }
        }
        lines.push(String::new());
    }

    let urls: Vec<&str> = response
        .results
        .iter()
        .map(result_url)
        .filter_map(|url| {
            let trimmed = url.trim();
            (!trimmed.is_empty()).then_some(trimmed)
        })
        .collect();

    if !urls.is_empty() {
        lines.push("URLs:".to_string());
        for url in urls {
            lines.push(format!("- {}", url));
        }
    }

    lines.join("\n")
}

fn result_url(result: &SearchResult) -> &str {
    match result {
        SearchResult::Web(r) => &r.url,
        SearchResult::News(r) => &r.url,
        SearchResult::Image(r) => &r.url,
        SearchResult::Video(r) => &r.url,
    }
}

pub fn render_fanout_text(response: &SearchBatchResponse) -> String {
    let mut lines = vec![
        format!("Query: {}", response.query),
        format!("Providers succeeded: {}", response.responses.len()),
        format!("Providers failed: {}", response.failures.len()),
        String::new(),
    ];

    for provider_response in &response.responses {
        lines.push(format!("== {} ==", provider_response.provider));
        lines.push(render_text(provider_response));
        lines.push(String::new());
    }

    if !response.failures.is_empty() {
        lines.push("== Failures ==".to_string());
        for failure in &response.failures {
            lines.push(format!("- {}: {}", failure.provider, failure.error));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        ImageResult, NewsResult, ProviderSearchFailure, SearchError, VideoResult, WebResult,
    };

    #[test]
    fn test_render_text_mixed_results() {
        let response = SearchResponse {
            query: "rust".to_string(),
            provider: "brave".to_string(),
            results: vec![
                SearchResult::Web(WebResult {
                    title: "Rust Lang".to_string(),
                    url: "https://rust-lang.org".to_string(),
                    snippet: Some("Safe systems".to_string()),
                    display_url: None,
                }),
                SearchResult::News(NewsResult {
                    title: "Rust News".to_string(),
                    url: "https://example.com/news".to_string(),
                    snippet: Some("Breaking update".to_string()),
                    source: Some("Example".to_string()),
                    published_at: None,
                }),
                SearchResult::Image(ImageResult {
                    title: "Rust Logo".to_string(),
                    url: "https://example.com/img.png".to_string(),
                    thumbnail_url: None,
                    source: None,
                }),
                SearchResult::Video(VideoResult {
                    title: "Rust Tutorial".to_string(),
                    url: "https://example.com/video".to_string(),
                    thumbnail_url: None,
                    duration: None,
                    published_at: None,
                }),
            ],
            total_estimated: None,
            next_page: None,
        };
        let text = render_text(&response);
        assert!(text.contains("Provider: brave"));
        assert!(text.contains("Query: rust"));
        assert!(text.contains("Rust Lang"));
        assert!(text.contains("https://rust-lang.org"));
        assert!(text.contains("[NEWS] Rust News"));
        assert!(text.contains("Breaking update"));
        assert!(text.contains("[IMAGE] Rust Logo"));
        assert!(text.contains("[VIDEO] Rust Tutorial"));
    }

    #[test]
    fn render_text_appends_url_list_at_end() {
        let response = SearchResponse {
            query: "rust".to_string(),
            provider: "brave".to_string(),
            results: vec![
                SearchResult::Web(WebResult {
                    title: "Rust Lang".to_string(),
                    url: "https://rust-lang.org".to_string(),
                    snippet: None,
                    display_url: None,
                }),
                SearchResult::News(NewsResult {
                    title: "Rust News".to_string(),
                    url: " https://example.com/news ".to_string(),
                    snippet: None,
                    source: None,
                    published_at: None,
                }),
                SearchResult::Image(ImageResult {
                    title: "Missing URL".to_string(),
                    url: "   ".to_string(),
                    thumbnail_url: None,
                    source: None,
                }),
                SearchResult::Video(VideoResult {
                    title: "Rust Tutorial".to_string(),
                    url: "https://example.com/video".to_string(),
                    thumbnail_url: None,
                    duration: None,
                    published_at: None,
                }),
            ],
            total_estimated: None,
            next_page: None,
        };

        let text = render_text(&response);

        assert!(text.ends_with(
            "URLs:\n- https://rust-lang.org\n- https://example.com/news\n- https://example.com/video"
        ));
        assert!(!text.contains("-    "));
    }

    #[test]
    fn test_render_fanout_text_includes_successes_and_failures() {
        let response = SearchBatchResponse {
            query: "rust".to_string(),
            responses: vec![SearchResponse {
                query: "rust".to_string(),
                provider: "brave".to_string(),
                results: vec![SearchResult::Web(WebResult {
                    title: "Rust Lang".to_string(),
                    url: "https://rust-lang.org".to_string(),
                    snippet: Some("Safe systems".to_string()),
                    display_url: None,
                })],
                total_estimated: None,
                next_page: None,
            }],
            failures: vec![ProviderSearchFailure {
                provider: "exa".to_string(),
                error: SearchError::InvalidQuery("unsupported".to_string()),
            }],
        };

        let text = render_fanout_text(&response);

        assert!(text.contains("Query: rust"));
        assert!(text.contains("Providers succeeded: 1"));
        assert!(text.contains("Providers failed: 1"));
        assert!(text.contains("== brave =="));
        assert!(text.contains("Rust Lang"));
        assert!(text.contains("- exa: invalid query: unsupported"));
    }
}
