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
   └── initializes tracing and dotenv
   └── delegates runtime execution to src/cli/runner.rs

3. src/cli/runner.rs
   └── CliArgs::parse() produces CliArgs { query, provider, search_type, limit, ... }
   └── src/cli/request.rs maps CliArgs + query text → SearchQuery
   └── selects single-provider or all-provider mode
   └── requests services from ProviderRegistry
   └── renders stdout through src/cli/output.rs

4. src/app/search_service.rs or src/app/fanout_search_service.rs
   └── SearchService::search(SearchQuery) awaits one provider
   └── FanoutSearchService::search_all(SearchQuery) awaits enabled providers sequentially
   └── delegates to dyn SearchProvider trait objects

5. src/providers/*/client.rs
   └── provider-specific SearchProvider::search(&SearchQuery)
   └── Brave builds GET endpoint + query params
   └── Exa builds POST /search JSON body
   └── calls HttpClient::get_json() or HttpClient::post_json()

6. src/transport/http.rs
   └── ReqwestHttpClient executes HTTP GET
   └── on success: deserializes JSON into BraveNewsResponse
   └── on failure: maps status code → SearchError

7. src/providers/brave/mapper.rs
   └── map_news_response(BraveNewsResponse) → SearchResponse
   └── transforms DTOs into domain SearchResult::News items

8. src/cli/output.rs
   └── render_text(&SearchResponse) → String for single-provider output
   └── render_fanout_text(&SearchBatchResponse) → String for all-provider output

9. src/cli/runner.rs
   └── println!("{}", rendered_string)
```

## Type contracts by boundary

Every public function that crosses a module boundary uses a domain type.

| Boundary | Function | Input type | Output type |
|----------|----------|------------|-------------|
| CLI → App | `SearchService::search` | `SearchQuery` | `Result<SearchResponse, SearchError>` |
| CLI → App | `FanoutSearchService::search_all` | `SearchQuery` | `SearchBatchResponse` |
| App → Provider | `SearchProvider::search` | `&SearchQuery` | `Result<SearchResponse, SearchError>` |
| Provider → Transport | `HttpClient::{get_json, post_json}` | `url, headers, params/body` | `Result<T, SearchError>` |
| Transport → Provider | (JSON response body) | bytes | provider DTOs such as `Brave*Response` or `ExaSearchResponse` |
| Provider → Domain | `map_*_response` | provider DTOs | `SearchResponse` |
| CLI rendering | `render_text` | `&SearchResponse` | `String` |
| CLI rendering | `render_fanout_text` | `&SearchBatchResponse` | `String` |

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
    pub total_estimated: Option<u64>,
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

Fan-out results stay provider-agnostic in the domain layer:

```rust
pub struct SearchBatchResponse {
    pub query: String,
    pub responses: Vec<SearchResponse>,
    pub failures: Vec<ProviderSearchFailure>,
}

pub struct ProviderSearchFailure {
    pub provider: String,
    pub error: SearchError,
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
3. **App layer** — `SearchService` returns the `SearchError` untouched; `FanoutSearchService` records per-provider failures in `SearchBatchResponse` and continues to later providers.
4. **CLI layer** — `src/cli/runner.rs` matches on single-provider `SearchError` and prints a human-readable message to `stderr`, or renders fan-out successes and failures and returns exit code `1` when no provider succeeded.

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

`src/main.rs` remains a thin process entrypoint. It initializes process-level concerns and delegates to `src/cli/runner.rs`, which owns user-surface branching, query normalization, output rendering, and exit-code calculation.

Real provider identity and production construction live in `src/bootstrap/provider_catalog.rs`. The catalog declares each real provider's `ProviderId`, CLI token, display name, environment variable name, stable order, and production builder. `src/bootstrap/provider_registry.rs` consumes that catalog to register configured providers and compose `SearchService` or `FanoutSearchService`.

- `--provider brave` resolves the catalog token to `ProviderId::Brave`; the registry includes it only when `BRAVE_API_KEY` is configured
- `--provider exa` resolves the catalog token to `ProviderId::Exa`; the registry includes it only when `EXA_API_KEY` is configured
- `--provider all` is a CLI-only aggregate mode that uses `ProviderRegistry::build_all_enabled()` to build a `FanoutSearchService` from every configured real provider in catalog order
- omitting `--provider` still selects the first catalog provider, currently Brave

`FanoutSearchService` is application-layer orchestration over multiple domain `SearchProvider` trait objects. It does not render output; fan-out rendering remains in the CLI layer through `render_fanout_text`.

## Architecture enforcement

The rules are verified by `tests/architecture_test.rs`. These tests scan source files and fail if a forbidden import pattern is found.

| Layer | Forbidden imports |
|-------|-------------------|
| `src/domain/` | `crate::providers::`, `crate::transport::`, `crate::cli::`, `crate::app::` |
| `src/transport/` | `crate::providers::`, `crate::cli::`, `crate::app::` |
| `src/providers/` | `crate::cli::`, `crate::app::` |
| `src/app/` | `crate::cli::`, `crate::bootstrap::`, `crate::providers::`, `crate::transport::` |
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
