use crate::cli::scrape::ScrapedSite;
use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

const SCRAPED_CONTENT_PREVIEW_CHARS: usize = 2_000;

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
    lines.join("\n")
}

pub fn render_scraped_sites(sites: &[ScrapedSite]) -> String {
    let mut lines = vec!["== Scraped pages ==".to_string(), String::new()];
    for site in sites {
        lines.push(format!("Seed: {}", site.seed_url));
        lines.push(format!("Time: {:?}", site.duration));
        lines.push(format!("Page limit per seed: {}", site.page_limit));
        if let Some(err) = &site.error {
            lines.push(format!("Error: {err}"));
        } else {
            lines.push(format!("Extracted content pages: {}", site.pages.len()));
            for (i, page) in site.pages.iter().enumerate() {
                lines.push(format!("  Page {}: {}", i + 1, page.url));
                lines.push(format!("  Status: {}", page.status_code));
                lines.push("  Content excerpt:".to_string());
                for line in preview_content(&page.content).lines() {
                    lines.push(format!("    {}", line));
                }
            }

            lines.push(String::new());
            lines.push(format!(
                "Crawl telemetry - visited URLs: {}",
                site.visited_urls.len()
            ));
            for url in &site.visited_urls {
                lines.push(format!("  - {}", url));
            }
        }
        lines.push(String::new());
    }
    lines.join("\n")
}

fn preview_content(content: &str) -> String {
    let trimmed = content.trim();
    if trimmed.chars().count() <= SCRAPED_CONTENT_PREVIEW_CHARS {
        return trimmed.to_string();
    }

    let mut preview: String = trimmed
        .chars()
        .take(SCRAPED_CONTENT_PREVIEW_CHARS)
        .collect();
    preview.push_str("\n    ... [content truncated]");
    preview
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
    use crate::cli::scrape::ScrapedSite;
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
    fn test_render_scraped_sites_shows_content_separate_from_visited_urls() {
        let ok = ScrapedSite {
            seed_url: "https://example.com".into(),
            duration: std::time::Duration::from_millis(100),
            page_limit: 5,
            pages: vec![crate::cli::scrape::ScrapedPage {
                url: "https://example.com".into(),
                status_code: 200,
                content: "<html><body>Real scraped page body</body></html>".into(),
            }],
            visited_urls: vec!["https://example.com/a".into()],
            error: None,
        };
        let err = ScrapedSite {
            seed_url: "https://bad".into(),
            duration: std::time::Duration::from_secs(1),
            page_limit: 5,
            pages: vec![],
            visited_urls: vec![],
            error: Some("boom".into()),
        };
        let text = render_scraped_sites(&[ok, err]);
        assert!(text.contains("== Scraped pages =="));
        assert!(text.contains("Seed: https://example.com"));
        assert!(text.contains("Page limit per seed: 5"));
        assert!(text.contains("Extracted content pages: 1"));
        assert!(text.contains("Content excerpt:"));
        assert!(text.contains("Real scraped page body"));
        assert!(text.contains("Crawl telemetry - visited URLs: 1"));
        assert!(text.contains("https://example.com/a"));
        assert!(text.contains("Error: boom"));
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
