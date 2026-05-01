//! Post-search URL extraction and optional Spider/Chrome crawling (CLI layer only).

use std::collections::HashSet;
use std::time::{Duration, Instant};

use spider::features::chrome_common::RequestInterceptConfiguration;
use spider::website::Website;

use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

/// Default max pages per seed when `--scrape-limit` is omitted.
pub const DEFAULT_SCRAPE_PAGE_LIMIT: u32 = 10;

/// Default per-seed scrape timeout when `--scrape-timeout-seconds` is omitted.
pub const DEFAULT_SCRAPE_TIMEOUT_SECS: u64 = 120;

pub fn page_limit_for_scrape(cli: &crate::cli::args::CliArgs) -> u32 {
    cli.scrape_limit
        .and_then(|n| u32::try_from(n).ok())
        .unwrap_or(DEFAULT_SCRAPE_PAGE_LIMIT)
}

pub fn timeout_duration_for_scrape(cli: &crate::cli::args::CliArgs) -> Duration {
    Duration::from_secs(
        cli.scrape_timeout_seconds
            .unwrap_or(DEFAULT_SCRAPE_TIMEOUT_SECS),
    )
}

/// Extract result URLs in search order.
pub fn urls_from_results(results: &[SearchResult]) -> Vec<String> {
    results
        .iter()
        .map(|r| url_from_result(r).to_string())
        .collect()
}

fn url_from_result(r: &SearchResult) -> &str {
    match r {
        SearchResult::Web(w) => &w.url,
        SearchResult::News(n) => &n.url,
        SearchResult::Image(i) => &i.url,
        SearchResult::Video(v) => &v.url,
    }
}

/// Deduplicate URLs preserving first-seen order.
pub fn dedup_ordered(urls: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::<String>::new();
    let mut out = Vec::new();
    for u in urls {
        if seen.insert(u.clone()) {
            out.push(u);
        }
    }
    out
}

pub fn deduped_urls_from_response(response: &SearchResponse) -> Vec<String> {
    dedup_ordered(urls_from_results(&response.results))
}

pub fn deduped_urls_from_batch(batch: &SearchBatchResponse) -> Vec<String> {
    let mut urls = Vec::new();
    for response in &batch.responses {
        urls.extend(urls_from_results(&response.results));
    }
    dedup_ordered(urls)
}

/// One page captured by Spider while crawling a seed URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapedPage {
    pub url: String,
    pub status_code: u16,
    pub content: String,
}

/// One scraped seed URL: content pages plus crawl telemetry from Spider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrapedSite {
    pub seed_url: String,
    pub duration: Duration,
    pub page_limit: u32,
    pub pages: Vec<ScrapedPage>,
    pub visited_urls: Vec<String>,
    pub error: Option<String>,
}

struct CrawlResult {
    pages: Vec<ScrapedPage>,
    visited_urls: Vec<String>,
}

/// Scrape each seed sequentially with a per-seed timeout.
pub async fn scrape_seed_urls(
    seeds: &[String],
    page_limit_per_site: u32,
    timeout_per_seed: Duration,
) -> Vec<ScrapedSite> {
    let mut out = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let start = Instant::now();
        let crawl =
            tokio::time::timeout(timeout_per_seed, crawl_one_seed(seed, page_limit_per_site));
        match crawl.await {
            Err(_) => {
                out.push(ScrapedSite {
                    seed_url: seed.clone(),
                    duration: start.elapsed(),
                    page_limit: page_limit_per_site,
                    pages: vec![],
                    visited_urls: vec![],
                    error: Some(format!("timed out after {timeout_per_seed:?}")),
                });
            }
            Ok(Ok(crawl_result)) => {
                out.push(ScrapedSite {
                    seed_url: seed.clone(),
                    duration: start.elapsed(),
                    page_limit: page_limit_per_site,
                    pages: crawl_result.pages,
                    visited_urls: crawl_result.visited_urls,
                    error: None,
                });
            }
            Ok(Err(message)) => {
                out.push(ScrapedSite {
                    seed_url: seed.clone(),
                    duration: start.elapsed(),
                    page_limit: page_limit_per_site,
                    pages: vec![],
                    visited_urls: vec![],
                    error: Some(message),
                });
            }
        }
    }
    out
}

async fn crawl_one_seed(seed: &str, page_limit: u32) -> Result<CrawlResult, String> {
    let spider_page_limit = page_limit.max(2);
    let mut website: Website = Website::new(seed)
        .with_limit(spider_page_limit)
        .with_chrome_intercept(RequestInterceptConfiguration::new(true))
        .with_stealth(true)
        .build()
        .map_err(|e| e.to_string())?;

    website.scrape().await;

    let mut pages: Vec<ScrapedPage> = website
        .get_pages()
        .map(|pages| {
            pages
                .iter()
                .map(|page| ScrapedPage {
                    url: page.get_url().to_string(),
                    status_code: page.status_code.as_u16(),
                    content: page.get_content(),
                })
                .collect()
        })
        .unwrap_or_default();
    pages.truncate(page_limit as usize);

    let links = website.get_all_links_visited().await;
    let mut visited_urls: Vec<String> = links.into_iter().map(|u| u.to_string()).collect();
    visited_urls.truncate(page_limit as usize);
    Ok(CrawlResult {
        pages,
        visited_urls,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        ImageResult, NewsResult, ProviderSearchFailure, SearchBatchResponse, SearchError,
        SearchResponse, SearchResult, VideoResult, WebResult,
    };

    #[test]
    fn urls_from_results_collects_each_variant_url() {
        let results = vec![
            SearchResult::Web(WebResult {
                title: "w".into(),
                url: "https://a".into(),
                snippet: None,
                display_url: None,
            }),
            SearchResult::News(NewsResult {
                title: "n".into(),
                url: "https://b".into(),
                snippet: None,
                source: None,
                published_at: None,
            }),
            SearchResult::Image(ImageResult {
                title: "i".into(),
                url: "https://c.png".into(),
                thumbnail_url: None,
                source: None,
            }),
            SearchResult::Video(VideoResult {
                title: "v".into(),
                url: "https://d".into(),
                thumbnail_url: None,
                duration: None,
                published_at: None,
            }),
        ];
        assert_eq!(
            urls_from_results(&results),
            vec![
                "https://a".to_string(),
                "https://b".to_string(),
                "https://c.png".to_string(),
                "https://d".to_string(),
            ]
        );
    }

    #[test]
    fn dedup_ordered_keeps_first_occurrence_order() {
        let d = dedup_ordered(vec![
            "https://x".into(),
            "https://y".into(),
            "https://x".into(),
            "https://z".into(),
            "https://y".into(),
        ]);
        assert_eq!(
            d,
            vec![
                "https://x".to_string(),
                "https://y".to_string(),
                "https://z".to_string(),
            ]
        );
    }

    #[test]
    fn deduped_urls_from_response_preserves_order_and_dedupes() {
        let response = SearchResponse {
            query: "rust".to_string(),
            provider: "brave".to_string(),
            results: vec![
                SearchResult::Web(WebResult {
                    title: "a".into(),
                    url: "https://dup".into(),
                    snippet: None,
                    display_url: None,
                }),
                SearchResult::News(NewsResult {
                    title: "b".into(),
                    url: "https://dup".into(),
                    snippet: None,
                    source: None,
                    published_at: None,
                }),
                SearchResult::Web(WebResult {
                    title: "c".into(),
                    url: "https://only-once".into(),
                    snippet: None,
                    display_url: None,
                }),
            ],
            total_estimated: None,
            next_page: None,
        };
        assert_eq!(
            deduped_urls_from_response(&response),
            vec!["https://dup".to_string(), "https://only-once".to_string(),]
        );
    }

    #[test]
    fn deduped_urls_from_batch_flattens_providers_then_dedupes() {
        let batch = SearchBatchResponse {
            query: "q".into(),
            responses: vec![
                SearchResponse {
                    query: "q".into(),
                    provider: "brave".into(),
                    results: vec![SearchResult::Web(WebResult {
                        title: "1".into(),
                        url: "https://dup".into(),
                        snippet: None,
                        display_url: None,
                    })],
                    total_estimated: None,
                    next_page: None,
                },
                SearchResponse {
                    query: "q".into(),
                    provider: "exa".into(),
                    results: vec![
                        SearchResult::Web(WebResult {
                            title: "2".into(),
                            url: "https://dup".into(),
                            snippet: None,
                            display_url: None,
                        }),
                        SearchResult::Web(WebResult {
                            title: "3".into(),
                            url: "https://only-exa".into(),
                            snippet: None,
                            display_url: None,
                        }),
                    ],
                    total_estimated: None,
                    next_page: None,
                },
            ],
            failures: vec![ProviderSearchFailure {
                provider: "broken".into(),
                error: SearchError::Provider("err".into()),
            }],
        };
        assert_eq!(
            deduped_urls_from_batch(&batch),
            vec!["https://dup".to_string(), "https://only-exa".to_string(),]
        );
    }
}
