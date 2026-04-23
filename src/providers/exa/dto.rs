use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaSearchRequest {
    pub query: String,
    #[serde(rename = "type")]
    pub search_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_results: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_published_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_published_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<bool>,
    pub contents: ExaContentsRequest,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaContentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<ExaHighlightsRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ExaSummaryRequest>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaHighlightsRequest {
    pub max_characters: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Request-time `contents.summary` object with query (see Exa search API).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaSummaryRequest {
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaSearchResponse {
    pub request_id: Option<String>,
    pub search_type: Option<String>,
    #[serde(default)]
    pub results: Vec<ExaResult>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExaResult {
    pub title: Option<String>,
    pub url: Option<String>,
    pub published_date: Option<String>,
    pub author: Option<String>,
    #[serde(default)]
    pub highlights: Vec<String>,
    pub text: Option<String>,
    pub summary: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{ExaResult, ExaSearchResponse};

    #[test]
    fn test_exa_search_response_deserializes_minimal_payload() {
        let json = r#"{
            "requestId": "req_123",
            "searchType": "auto",
            "results": [
                {
                    "title": "Example result",
                    "url": "https://example.com",
                    "publishedDate": "2026-04-15T00:00:00.000Z",
                    "summary": "Example summary"
                }
            ]
        }"#;

        let response: ExaSearchResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.request_id.as_deref(), Some("req_123"));
        assert_eq!(response.search_type.as_deref(), Some("auto"));
        assert_eq!(
            response.results,
            vec![ExaResult {
                title: Some("Example result".to_string()),
                url: Some("https://example.com".to_string()),
                published_date: Some("2026-04-15T00:00:00.000Z".to_string()),
                author: None,
                highlights: vec![],
                text: None,
                summary: Some("Example summary".to_string()),
            }]
        );
    }

    #[test]
    fn test_exa_search_response_deserializes_highlights_and_summary() {
        let json = r#"{
            "requestId": "req_456",
            "searchType": "auto",
            "results": [
                {
                    "title": "Article",
                    "url": "https://example.com/a",
                    "highlights": ["First excerpt.", "Second excerpt."],
                    "summary": "One-line overview",
                    "text": "FULL PAGE MARKDOWN WOULD BE HERE"
                }
            ]
        }"#;

        let response: ExaSearchResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.results.len(), 1);
        let r = &response.results[0];
        assert_eq!(
            r.highlights,
            vec!["First excerpt.".to_string(), "Second excerpt.".to_string()]
        );
        assert_eq!(r.summary.as_deref(), Some("One-line overview"));
        assert_eq!(r.text.as_deref(), Some("FULL PAGE MARKDOWN WOULD BE HERE"));
    }
}
