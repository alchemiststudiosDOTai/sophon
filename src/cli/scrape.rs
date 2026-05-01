//! Optional HTTP fetch of result URLs for `--scrape` (CLI-only; not domain).

use std::time::{Duration, Instant};

use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE, HeaderMap};
use reqwest::{Client, Response, Url};
use tokio::task::JoinSet;
use tokio::time::timeout;

use crate::domain::{SearchBatchResponse, SearchResponse, SearchResult};

const MAX_CONCURRENT_SCRAPE_REQUESTS: usize = 4;
const SCRAPE_REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_SCRAPE_BODY_BYTES: usize = 512 * 1024;
const TRUNCATION_MARKER: &str = "\n<scrape body truncated>";

type ScrapedPage = (String, Option<u16>, String);
type IndexedScrapedPage = (usize, ScrapedPage);

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
    if page_limit == 0 {
        return (Vec::new(), 0, None);
    }

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
    if page_limit == 0 {
        return Vec::new();
    }

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
        if is_fetchable_url(&url) {
            urls.push(url);
        }
    }
    urls
}

async fn scrape_urls(
    client: &Client,
    urls: Vec<String>,
) -> (Vec<ScrapedPage>, u64, Option<String>) {
    let start = Instant::now();
    if urls.is_empty() {
        return (Vec::new(), 0, None);
    }

    let mut pending = urls.into_iter().enumerate();
    let mut tasks = JoinSet::new();
    let mut pages = Vec::new();

    for _ in 0..MAX_CONCURRENT_SCRAPE_REQUESTS {
        spawn_next_scrape(client, &mut pending, &mut tasks);
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(page) => pages.push(page),
            Err(error) => pages.push((
                usize::MAX,
                (
                    "(unknown url)".to_string(),
                    None,
                    format!("<fetch error: scrape task failed: {error}>"),
                ),
            )),
        };
        spawn_next_scrape(client, &mut pending, &mut tasks);
    }

    pages.sort_by_key(|(index, _)| *index);
    let pages = pages.into_iter().map(|(_, page)| page).collect();
    let duration_ms = start.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
    (pages, duration_ms, None)
}

fn spawn_next_scrape(
    client: &Client,
    pending: &mut impl Iterator<Item = (usize, String)>,
    tasks: &mut JoinSet<IndexedScrapedPage>,
) {
    if let Some((index, url)) = pending.next() {
        let client = client.clone();
        tasks.spawn(async move { (index, fetch_url_with_timeout(client, url).await) });
    }
}

async fn fetch_url_with_timeout(client: Client, url: String) -> ScrapedPage {
    match timeout(SCRAPE_REQUEST_TIMEOUT, fetch_url(client, url.clone())).await {
        Ok(page) => page,
        Err(_) => (
            url,
            None,
            format!(
                "<fetch error: request timed out after {}s>",
                SCRAPE_REQUEST_TIMEOUT.as_secs()
            ),
        ),
    }
}

async fn fetch_url(client: Client, url: String) -> ScrapedPage {
    match client.get(&url).send().await {
        Ok(resp) => response_to_page(url, resp).await,
        Err(error) => (url, None, format!("<fetch error: {error}>")),
    }
}

async fn response_to_page(url: String, mut resp: Response) -> ScrapedPage {
    let status = resp.status().as_u16();
    if !is_allowed_response_content(resp.headers()) {
        return (
            url,
            Some(status),
            format!(
                "<fetch skipped: unsupported content type {}>",
                content_type_label(resp.headers())
            ),
        );
    }

    if content_length_exceeds_cap(resp.headers()) {
        return (
            url,
            Some(status),
            format!("<fetch skipped: response exceeds {MAX_SCRAPE_BODY_BYTES} bytes>"),
        );
    }

    match read_bounded_text(&mut resp).await {
        Ok(body) => (url, Some(status), body),
        Err(error) => (url, Some(status), format!("<fetch error: {error}>")),
    }
}

async fn read_bounded_text(resp: &mut Response) -> Result<String, reqwest::Error> {
    let content_limit = MAX_SCRAPE_BODY_BYTES.saturating_sub(TRUNCATION_MARKER.len());
    let mut body = Vec::new();

    while let Some(chunk) = resp.chunk().await? {
        let remaining = content_limit.saturating_sub(body.len());
        if chunk.len() > remaining {
            body.extend_from_slice(&chunk[..remaining]);
            body.extend_from_slice(TRUNCATION_MARKER.as_bytes());
            return Ok(String::from_utf8_lossy(&body).into_owned());
        }
        body.extend_from_slice(&chunk);
    }

    Ok(String::from_utf8_lossy(&body).into_owned())
}

fn is_fetchable_url(url: &str) -> bool {
    Url::parse(url)
        .map(|url| matches!(url.scheme(), "http" | "https"))
        .unwrap_or(false)
}

fn is_allowed_response_content(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(is_allowed_content_type)
}

fn is_allowed_content_type(content_type: &str) -> bool {
    let media_type = content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();

    media_type.starts_with("text/")
        || matches!(
            media_type.as_str(),
            "application/json" | "application/xml" | "application/xhtml+xml"
        )
        || media_type.ends_with("+json")
        || media_type.ends_with("+xml")
}

fn content_length_exceeds_cap(headers: &HeaderMap) -> bool {
    headers
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .is_some_and(|length| length > MAX_SCRAPE_BODY_BYTES as u64)
}

fn content_type_label(headers: &HeaderMap) -> String {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("missing")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn content_filter_allows_text_and_structured_text() {
        assert!(is_allowed_content_type("text/html; charset=utf-8"));
        assert!(is_allowed_content_type("application/json"));
        assert!(is_allowed_content_type("application/activity+json"));
        assert!(is_allowed_content_type("application/rss+xml"));
    }

    #[test]
    fn content_filter_rejects_binary_and_missing_types() {
        assert!(!is_allowed_content_type("image/png"));
        assert!(!is_allowed_content_type("application/octet-stream"));
        assert!(!is_allowed_response_content(&HeaderMap::new()));
    }

    #[test]
    fn content_length_over_cap_is_rejected() {
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&(MAX_SCRAPE_BODY_BYTES as u64 + 1).to_string()).unwrap(),
        );
        assert!(content_length_exceeds_cap(&headers));
    }

    #[test]
    fn only_http_urls_are_fetchable() {
        assert!(is_fetchable_url("https://example.com"));
        assert!(is_fetchable_url("http://example.com"));
        assert!(!is_fetchable_url("file:///etc/passwd"));
        assert!(!is_fetchable_url("not a url"));
    }
}
