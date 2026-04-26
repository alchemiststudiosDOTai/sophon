---
title: "Architecture"
when_to_read:
  - "When reading or editing the mdBook documentation surface."
  - "When checking how the CLI architecture, quickstart, and user-facing docs fit together."
summary: "mdBook documentation page for sophon-cli: Architecture. It contributes user and maintainer guidance that is built by the docs gate."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
---

# Architecture

## Design principle

The codebase is split into four strictly ordered runtime layers plus a narrow bootstrap composition layer. **No runtime layer may import from a layer above it.** This prevents the CLI from leaking into the domain and keeps providers interchangeable.

```
cli (top)
  ↑
app
  ↑
providers
  ↑
transport
  ↑
domain (bottom)

bootstrap composes app + providers + transport at startup
```

## Full request lifecycle

```
1. Terminal
   └── user runs: cargo run -- "query" --search-type news --limit 3

2. src/main.rs
   └── CliArgs::parse() produces CliArgs { query, provider, search_type, limit, ... }
   └── converts CliProvider to ProviderId
   └── asks the bootstrap ProviderRegistry to build SearchService
   └── maps CliArgs → SearchQuery

3. src/app/search_service.rs
   └── SearchService::search(SearchQuery) awaits
   └── delegates to dyn SearchProvider

4. src/providers/*/client.rs
   └── provider-specific SearchProvider::search(&SearchQuery)
   └── Brave builds GET endpoint + query params
   └── Exa builds POST /search JSON body
   └── calls HttpClient::get_json() or HttpClient::post_json()

5. src/transport/http.rs
   └── ReqwestHttpClient executes HTTP GET
   └── on success: deserializes JSON into BraveNewsResponse
   └── on failure: maps status code → SearchError

6. src/providers/brave/mapper.rs
   └── map_news_response(BraveNewsResponse) → SearchResponse
   └── transforms DTOs into domain SearchResult::News items

7. src/cli/output.rs
   └── render_text(&SearchResponse) → String

8. src/main.rs
   └── println!("{}", rendered_string)
```

## Type contracts by boundary

Every public function that crosses a module boundary uses a domain type.

| Boundary | Function | Input type | Output type |
|----------|----------|------------|-------------|
| CLI → App | `SearchService::search` | `SearchQuery` | `Result<SearchResponse, SearchError>` |
| App → Provider | `SearchProvider::search` | `&SearchQuery` | `Result<SearchResponse, SearchError>` |
| Provider → Transport | `HttpClient::{get_json, post_json}` | `url, headers, params/body` | `Result<T, SearchError>` |
| Transport → Provider | (JSON response body) | bytes | provider DTOs such as `Brave*Response` or `ExaSearchResponse` |
| Provider → Domain | `map_*_response` | provider DTOs | `SearchResponse` |
| App → CLI | `render_text` | `&SearchResponse` | `String` |

## Domain type reference

### Query

```rust
pub struct SearchQuery {
    pub text: String,
    pub search_type: SearchType,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub safe_search: Option<SafeSearch>,
    pub country: Option<String>,
    pub language: Option<String>,
    pub time_range: Option<TimeRange>,
}
```

### Result

```rust
pub struct SearchResponse {
    pub query: String,
    pub provider: String,
    pub results: Vec<SearchResult>,
    pub total_estimated: Option<usize>,
    pub next_page: Option<PageToken>,
}
```

```rust
pub enum SearchResult {
    Web(WebResult),
    News(NewsResult),
    Image(ImageResult),
    Video(VideoResult),
}
```

### Provider trait

```rust
#[async_trait]
pub trait SearchProvider: Send + Sync {
    fn id(&self) -> String;
    fn capabilities(&self) -> ProviderCapabilities;
    async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError>;
}
```

### Error

```rust
pub enum SearchError {
    InvalidQuery(String),
    Config(String),
    Provider { status: Option<u16>, message: String },
    Transport(String),
    Decode(String),
}
```

## Error flow

Errors are created at the layer where the failure occurs and bubble upward unchanged:

1. **Transport layer** — `reqwest` failures, non-2xx HTTP status, or JSON decode errors become `SearchError::Transport`, `SearchError::Provider`, or `SearchError::Decode`.
2. **Provider layer** — can surface `SearchError` directly; does not wrap in another error type.
3. **App layer** — `SearchService` returns the `SearchError` untouched.
4. **CLI layer** — `main.rs` matches on `SearchError` and prints a human-readable message to `stderr`, then exits with code `1`.

This keeps error handling simple: there is only one error type in the public API.

## Provider-specific dispatch

`BraveProvider` matches `SearchQuery.search_type` to decide which endpoint and mapper to use:

| `SearchType` | Brave endpoint | DTO | Mapper |
|--------------|----------------|-----|--------|
| `Web` | `web/search` | `BraveWebResponse` | `map_web_response` |
| `News` | `news/search` | `BraveNewsResponse` | `map_news_response` |
| `Images` | `images/search` | `BraveImagesResponse` | `map_images_response` |
| `Videos` | `videos/search` | `BraveVideosResponse` | `map_videos_response` |

`ExaProvider` reuses the same domain and app contracts but has a different transport shape:

| `SearchType` | Exa request | DTO | Mapper |
|--------------|-------------|-----|--------|
| `Web` | `POST /search` without `category` | `ExaSearchResponse` | `map_web_response` |
| `News` | `POST /search` with `category = "news"` | `ExaSearchResponse` | `map_news_response` |
| `Images` | rejected with `SearchError::InvalidQuery` | n/a | n/a |
| `Videos` | rejected with `SearchError::InvalidQuery` | n/a | n/a |

Unsupported Exa inputs are rejected at runtime instead of being ignored: `Images`, `Videos`, `offset`, and `language`.

## Runtime provider selection

`main.rs` remains the binary edge that chooses the requested provider ID. Concrete provider construction lives in `src/bootstrap/provider_registry.rs`, where typed provider config, HTTP transport, provider clients, and `SearchService` are composed.

- `--provider brave` maps to `ProviderId::Brave`; the registry includes it only when `BRAVE_API_KEY` is configured
- `--provider exa` maps to `ProviderId::Exa`; the registry includes it only when `EXA_API_KEY` is configured
- omitting `--provider` still selects Brave

## Architecture enforcement

The rules are verified by `tests/architecture_test.rs`. These tests scan source files and fail if a forbidden import pattern is found.

| Layer | Forbidden imports |
|-------|-------------------|
| `src/domain/` | `crate::providers::`, `crate::transport::`, `crate::cli::`, `crate::app::` |
| `src/transport/` | `crate::providers::`, `crate::cli::`, `crate::app::` |
| `src/providers/` | `crate::cli::`, `crate::app::` |
| `src/app/` | `crate::cli::` |
| `src/bootstrap/` | `crate::cli::` |
| Any layer except `src/cli/` | `render_text` |

Run them with the rest of the suite:

```bash
cargo test
```

## Testing boundaries

Each layer has tests that mock the layer directly beneath it:

- **`SearchService`** tests mock `SearchProvider` to prove delegation works without a real HTTP call.
- **`BraveProvider`** tests mock `HttpClient` to prove request building and response mapping work without hitting the Brave API.
- **Mapper** tests construct minimal DTOs and assert the mapped `SearchResponse` contains expected data.
- **Architecture** tests scan the filesystem to guarantee the import direction rules above.
