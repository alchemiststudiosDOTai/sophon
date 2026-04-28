use crate::cli::args::CliArgs;
use crate::domain::SearchQuery;

pub fn build_search_query(query_text: String, args: &CliArgs) -> SearchQuery {
    SearchQuery {
        text: query_text,
        search_type: args.search_type.into(),
        limit: args.limit,
        offset: args.offset,
        safe_search: args.safe_search.map(|safe_search| safe_search.into()),
        country: args.country.clone(),
        language: args.language.clone(),
        time_range: None,
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::args::{CliArgs, CliProvider, CliSafeSearch, CliSearchType};
    use crate::cli::request::build_search_query;
    use crate::domain::{SafeSearch, SearchType};

    #[test]
    fn build_search_query_maps_all_supported_fields() {
        let args = CliArgs {
            query: Some("ignored parsed query".to_string()),
            about: false,
            search_type: CliSearchType::News,
            provider: CliProvider::All,
            limit: Some(7),
            offset: Some(14),
            safe_search: Some(CliSafeSearch::Strict),
            country: Some("US".to_string()),
            language: Some("en".to_string()),
        };

        let query = build_search_query("rust search".to_string(), &args);

        assert_eq!(query.text, "rust search");
        assert_eq!(query.search_type, SearchType::News);
        assert_eq!(query.limit, Some(7));
        assert_eq!(query.offset, Some(14));
        assert_eq!(query.safe_search, Some(SafeSearch::Strict));
        assert_eq!(query.country, Some("US".to_string()));
        assert_eq!(query.language, Some("en".to_string()));
        assert_eq!(query.time_range, None);
        assert_eq!(args.country, Some("US".to_string()));
        assert_eq!(args.language, Some("en".to_string()));
    }
}
