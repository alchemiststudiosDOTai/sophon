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
    pub text: bool,
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
                text: None,
                summary: Some("Example summary".to_string()),
            }]
        );
    }
}
