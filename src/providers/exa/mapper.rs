use crate::domain::result::{NewsResult, SearchResponse, SearchResult, WebResult};
use crate::providers::exa::dto::{ExaResult, ExaSearchResponse};

/// Maximum characters for `snippet` shown in the CLI (after join / trim).
const SNIPPET_DISPLAY_MAX_CHARS: usize = 500;

const HIGHLIGHT_JOIN: &str = " … ";

pub fn map_web_response(query: &str, dto: ExaSearchResponse) -> SearchResponse {
    map_response(query, dto, |result, snippet| {
        SearchResult::Web(WebResult {
            title: result.title.unwrap_or_default(),
            url: result.url.unwrap_or_default(),
            snippet,
            display_url: None,
        })
    })
}

pub fn map_news_response(query: &str, dto: ExaSearchResponse) -> SearchResponse {
    map_response(query, dto, |result, snippet| {
        SearchResult::News(NewsResult {
            title: result.title.unwrap_or_default(),
            url: result.url.unwrap_or_default(),
            snippet,
            source: result.author,
            published_at: result.published_date,
        })
    })
}

fn map_response<F>(query: &str, dto: ExaSearchResponse, map_result: F) -> SearchResponse
where
    F: Fn(ExaResult, Option<String>) -> SearchResult,
{
    SearchResponse {
        query: query.to_string(),
        provider: "exa".to_string(),
        total_estimated: None,
        next_page: None,
        results: dto
            .results
            .into_iter()
            .map(|result| {
                let snippet = preferred_snippet(&result);
                map_result(result, snippet)
            })
            .collect(),
    }
}

fn preferred_snippet(result: &ExaResult) -> Option<String> {
    if let Some(s) = result
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Some(cap_snippet_chars(s, SNIPPET_DISPLAY_MAX_CHARS));
    }

    let joined = join_highlights(&result.highlights);
    if joined.is_empty() {
        return None;
    }

    Some(cap_snippet_chars(&joined, SNIPPET_DISPLAY_MAX_CHARS))
}

fn join_highlights(highlights: &[String]) -> String {
    highlights
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(HIGHLIGHT_JOIN)
}

fn cap_snippet_chars(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        return s.to_string();
    }
    let mut out: String = s.chars().take(max_chars.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::map_news_response;
    use crate::domain::result::SearchResult;
    use crate::providers::exa::dto::{ExaResult, ExaSearchResponse};

    fn sample_result() -> ExaResult {
        ExaResult {
            title: Some("Example headline".to_string()),
            url: Some("https://example.com/news".to_string()),
            published_date: Some("2026-04-15T00:00:00.000Z".to_string()),
            author: Some("Example Reporter".to_string()),
            highlights: vec![],
            text: None,
            summary: Some("Short summary".to_string()),
        }
    }

    #[test]
    fn test_map_news_response_prefers_summary_and_preserves_author() {
        let dto = ExaSearchResponse {
            request_id: Some("req_123".to_string()),
            search_type: Some("auto".to_string()),
            results: vec![ExaResult {
                highlights: vec!["Longer body text".to_string()],
                text: Some("IGNORED FULL TEXT".to_string()),
                ..sample_result()
            }],
        };

        let response = map_news_response("ai news", dto);

        assert_eq!(response.query, "ai news");
        assert_eq!(response.provider, "exa");
        assert_eq!(response.total_estimated, None);
        assert_eq!(response.next_page, None);
        assert_eq!(response.results.len(), 1);
        match &response.results[0] {
            SearchResult::News(result) => {
                assert_eq!(result.title, "Example headline");
                assert_eq!(result.url, "https://example.com/news");
                assert_eq!(result.snippet.as_deref(), Some("Short summary"));
                assert_eq!(result.source.as_deref(), Some("Example Reporter"));
                assert_eq!(
                    result.published_at.as_deref(),
                    Some("2026-04-15T00:00:00.000Z")
                );
            }
            other => panic!("expected news result, got {other:?}"),
        }
    }

    #[test]
    fn test_map_prefers_summary_over_highlights() {
        let dto = ExaSearchResponse {
            request_id: None,
            search_type: None,
            results: vec![ExaResult {
                summary: Some("Summary wins".to_string()),
                highlights: vec!["Highlight A".to_string(), "Highlight B".to_string()],
                text: Some("x".repeat(50_000)),
                ..sample_result()
            }],
        };
        let response = map_news_response("q", dto);
        match &response.results[0] {
            SearchResult::News(r) => assert_eq!(r.snippet.as_deref(), Some("Summary wins")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn test_map_whitespace_summary_falls_through_to_highlights() {
        let dto = ExaSearchResponse {
            request_id: None,
            search_type: None,
            results: vec![ExaResult {
                summary: Some("   \t  ".to_string()),
                highlights: vec!["Only this".to_string()],
                text: Some("x".repeat(10_000)),
                ..sample_result()
            }],
        };
        let response = map_news_response("q", dto);
        match &response.results[0] {
            SearchResult::News(r) => assert_eq!(r.snippet.as_deref(), Some("Only this")),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn test_map_highlights_joined_and_capped() {
        let a = "a".repeat(350);
        let b = "b".repeat(350);
        let dto = ExaSearchResponse {
            request_id: None,
            search_type: None,
            results: vec![ExaResult {
                summary: None,
                highlights: vec![a, b],
                text: Some("SHOULD NOT APPEAR".to_string()),
                ..sample_result()
            }],
        };
        let response = map_news_response("q", dto);
        match &response.results[0] {
            SearchResult::News(r) => {
                let s = r.snippet.as_ref().unwrap();
                assert!(!s.contains("SHOULD NOT APPEAR"));
                assert!(s.chars().count() <= 500);
                assert!(s.ends_with('…'));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn test_map_no_concise_fields_returns_none_despite_huge_text() {
        let dto = ExaSearchResponse {
            request_id: None,
            search_type: None,
            results: vec![ExaResult {
                summary: None,
                highlights: vec![],
                text: Some("x".repeat(100_000)),
                ..sample_result()
            }],
        };
        let response = map_news_response("q", dto);
        match &response.results[0] {
            SearchResult::News(r) => assert!(r.snippet.is_none()),
            other => panic!("{other:?}"),
        }
    }
}
