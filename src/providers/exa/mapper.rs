use crate::domain::result::{NewsResult, SearchResponse, SearchResult, WebResult};
use crate::providers::exa::dto::{ExaResult, ExaSearchResponse};

pub fn map_web_response(query: &str, dto: ExaSearchResponse) -> SearchResponse {
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
                SearchResult::Web(WebResult {
                    title: result.title.unwrap_or_default(),
                    url: result.url.unwrap_or_default(),
                    snippet,
                    display_url: None,
                })
            })
            .collect(),
    }
}

pub fn map_news_response(query: &str, dto: ExaSearchResponse) -> SearchResponse {
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
                SearchResult::News(NewsResult {
                    title: result.title.unwrap_or_default(),
                    url: result.url.unwrap_or_default(),
                    snippet,
                    source: result.author,
                    published_at: result.published_date,
                })
            })
            .collect(),
    }
}

fn preferred_snippet(result: &ExaResult) -> Option<String> {
    result.summary.clone().or_else(|| result.text.clone())
}

#[cfg(test)]
mod tests {
    use super::map_news_response;
    use crate::domain::result::SearchResult;
    use crate::providers::exa::dto::{ExaResult, ExaSearchResponse};

    #[test]
    fn test_map_news_response_prefers_summary_and_preserves_author() {
        let dto = ExaSearchResponse {
            request_id: Some("req_123".to_string()),
            search_type: Some("auto".to_string()),
            results: vec![ExaResult {
                title: Some("Example headline".to_string()),
                url: Some("https://example.com/news".to_string()),
                published_date: Some("2026-04-15T00:00:00.000Z".to_string()),
                author: Some("Example Reporter".to_string()),
                text: Some("Longer body text".to_string()),
                summary: Some("Short summary".to_string()),
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
}
