//! Optional HTTP fetch of result URLs for `--scrape` (CLI-only; not domain).

use std::time::Instant;

use reqwest::Client;

use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

/// Fetch up to `page_limit` distinct result URLs; returns `(pages, duration_ms, fatal_error)`.
pub async fn scrape_result_urls(
    client: &Client,
    response: &SearchResponse,
    page_limit: usize,
) -> (Vec<(String, Option<u16>, String)>, u64, Option<String>) {
    let urls = collect_urls_from_response(response, page_limit);
    scrape_urls(client, urls).await
}

/// For `--provider all`, one shared scrape: URLs from successful responses in order, deduped.
pub async fn scrape_batch_urls(
    client: &Client,
    batch: &SearchBatchResponse,
    page_limit: usize,
) -> (Vec<(String, Option<u16>, String)>, u64, Option<String>) {
    let mut seen = std::collections::HashSet::new();
    let mut urls = Vec::new();
    for r in &batch.responses {
        for u in urls_from_response(r) {
            if seen.insert(u.clone()) {
                urls.push(u);
                if urls.len() >= page_limit {
                    break;
                }
            }
        }
        if urls.len() >= page_limit {
            break;
        }
    }
    scrape_urls(client, urls).await
}

fn collect_urls_from_response(response: &SearchResponse, page_limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    for u in urls_from_response(response) {
        out.push(u);
        if out.len() >= page_limit {
            break;
        }
    }
    out
}

fn urls_from_response(response: &SearchResponse) -> Vec<String> {
    let mut urls = Vec::new();
    for result in &response.results {
        let url = match result {
            SearchResult::Web(r) => r.url.clone(),
            SearchResult::News(r) => r.url.clone(),
            SearchResult::Image(r) => r.url.clone(),
            SearchResult::Video(r) => r.url.clone(),
        };
        urls.push(url);
    }
    urls
}

async fn scrape_urls(
    client: &Client,
    urls: Vec<String>,
) -> (Vec<(String, Option<u16>, String)>, u64, Option<String>) {
    let start = Instant::now();
    if urls.is_empty() {
        return (Vec::new(), 0, None);
    }

    let mut pages = Vec::with_capacity(urls.len());
    for url in urls {
        match client.get(&url).send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body = resp.text().await.unwrap_or_default();
                pages.push((url, Some(status), body));
            }
            Err(e) => {
                pages.push((url, None, format!("<fetch error: {e}>")));
            }
        }
    }

    let duration_ms = start.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
    (pages, duration_ms, None)
}
