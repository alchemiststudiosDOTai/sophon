# Architecture

## Design principle

The codebase is split into four strictly ordered layers. **No layer may import from a layer above it.** This prevents the CLI from leaking into the domain and keeps providers interchangeable.

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
```

## Full request lifecycle

```
1. Terminal
   └── user runs: cargo run -- "query" --search-type news --limit 3

2. src/main.rs
   └── CliArgs::parse() produces CliArgs { query, search_type, limit, ... }
   └── maps CliArgs → SearchQuery

3. src/app/search_service.rs
   └── SearchService::search(SearchQuery) awaits
   └── delegates to dyn SearchProvider

4. src/providers/brave/client.rs
   └── BraveProvider::search(&SearchQuery)
   └── picks endpoint: news/search
   └── builds headers + query params
   └── calls HttpClient::get_json()

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
| Provider → Transport | `HttpClient::get_json` | `url, headers, params` | `Result<T, SearchError>` |
| Transport → Provider | (JSON response body) | bytes | `BraveWebResponse` / `BraveNewsResponse` / `BraveImagesResponse` / `BraveVideosResponse` |
| Provider → Domain | `map_*_response` | `Brave*Response` | `SearchResponse` |
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

Adding a new provider means:
1. Create a new module under `src/providers/`
2. Implement `SearchProvider` for your adapter
3. Map the provider's response DTOs into `SearchResponse`
4. No changes to `domain`, `app`, or `cli` are required.

## Architecture enforcement

The rules are verified by `tests/architecture_test.rs`. These tests scan source files and fail if a forbidden import pattern is found.

| Layer | Forbidden imports |
|-------|-------------------|
| `src/domain/` | `crate::providers::`, `crate::transport::`, `crate::cli::`, `crate::app::` |
| `src/transport/` | `crate::providers::`, `crate::cli::`, `crate::app::` |
| `src/providers/` | `crate::cli::`, `crate::app::` |
| `src/app/` | `crate::cli::` |
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
