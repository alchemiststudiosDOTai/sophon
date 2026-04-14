Yes. That is the right first move.

Do **not** start from the Brave response shape. Start from your **domain contract**, then adapt Brave into it. That is what keeps the CLI from becoming a pile of provider-specific conditionals once you add Exa/Tavily later.

Brave already exposes multiple search surfaces under one API family, including web, news, images, videos, and answer-style capabilities, so provider variance is guaranteed to grow over time. Designing around a provider adapter boundary now is the correct call. ([Brave][1])

## Core design rule

Split the system into four layers:

1. **Domain**
   Pure types and traits. No HTTP. No CLI parsing.
2. **Providers**
   Brave-specific request/response models plus adapter code into domain types.
3. **Application**
   Orchestration: take user input, resolve provider, execute search, return normalized results.
4. **CLI**
   Parse args, invoke app layer, render output.

That keeps the contracts clean.

---

# 1. Domain types

These should be provider-agnostic.

```rust
// src/domain/query.rs
#[derive(Debug, Clone, PartialEq, Eq)]
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

```rust
// src/domain/types.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Web,
    News,
    Images,
    Videos,
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafeSearch {
    Off,
    Moderate,
    Strict,
}
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}
```

## Result model

Use a normalized result enum, not one mega struct with a bunch of `Option`s.

```rust
// src/domain/result.rs
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResponse {
    pub query: String,
    pub provider: ProviderId,
    pub results: Vec<SearchResult>,
    pub total_estimated: Option<u64>,
    pub next_page: Option<PageToken>,
}
```

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Web(WebResult),
    News(NewsResult),
    Image(ImageResult),
    Video(VideoResult),
}
```

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub display_url: Option<String>,
}
```

Do the same for `NewsResult`, `ImageResult`, `VideoResult`.

## Provider identity

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderId {
    Brave,
}
```

## Pagination token

Do not expose raw provider pagination structures directly.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageToken(pub String);
```

---

# 2. Interfaces / contracts

This is the most important seam.

## Search provider trait

```rust
// src/domain/provider.rs
use async_trait::async_trait;

#[async_trait]
pub trait SearchProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    async fn search(
        &self,
        query: &SearchQuery,
    ) -> Result<SearchResponse, SearchError>;

    fn capabilities(&self) -> ProviderCapabilities;
}
```

## Capabilities

This prevents hardcoding assumptions in the CLI and app layer.

```rust
#[derive(Debug, Clone)]
pub struct ProviderCapabilities {
    pub web: bool,
    pub news: bool,
    pub images: bool,
    pub videos: bool,
    pub pagination: bool,
    pub safe_search: bool,
    pub time_range_filter: bool,
}
```

When you add Tavily or Exa later, the CLI can fail cleanly if a feature is unsupported.

## Error contract

Keep one top-level error type, but preserve category.

```rust
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("invalid query: {0}")]
    InvalidQuery(String),

    #[error("provider configuration error: {0}")]
    Config(String),

    #[error("authentication failed")]
    Auth,

    #[error("rate limited")]
    RateLimited,

    #[error("transport error: {0}")]
    Transport(String),

    #[error("provider returned invalid data: {0}")]
    Decode(String),

    #[error("provider error: {0}")]
    Provider(String),
}
```

That error boundary matters because Brave documents authentication and rate-limiting as first-class concerns, and those should not leak as random low-level HTTP failures everywhere. ([Brave][1])

---

# 3. Module layout

Keep it small, but shaped correctly.

```text
src/
  main.rs

  cli/
    mod.rs
    args.rs
    output.rs

  app/
    mod.rs
    search_service.rs

  domain/
    mod.rs
    query.rs
    result.rs
    provider.rs
    error.rs
    types.rs

  providers/
    mod.rs
    brave/
      mod.rs
      client.rs
      config.rs
      dto.rs
      mapper.rs

  transport/
    mod.rs
    http.rs

  config/
    mod.rs
    settings.rs
```

### Why this layout works

* `domain/`: stable core
* `providers/brave/dto.rs`: ugly Brave JSON shapes stay isolated
* `providers/brave/mapper.rs`: converts Brave DTOs into domain results
* `transport/http.rs`: reusable HTTP wrapper, retries, headers, timeout
* `app/search_service.rs`: orchestration only

---

# 4. Contract-based connections

You said “contract-based connections,” which is exactly right.

These are the actual seams:

## CLI -> App

The CLI should pass a parsed command object, not random strings.

```rust
pub struct SearchCommand {
    pub query: String,
    pub search_type: SearchType,
    pub limit: Option<usize>,
    pub output: OutputFormat,
}
```

## App -> Domain provider trait

`SearchService` depends on `dyn SearchProvider`, not Brave directly.

```rust
pub struct SearchService {
    provider: Box<dyn SearchProvider>,
}
```

## Provider -> Transport

Brave provider should depend on an HTTP abstraction, not directly on `reqwest::Client`.

```rust
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get_json<T>(
        &self,
        url: &str,
        headers: Vec<(String, String)>,
        query: Vec<(String, String)>,
    ) -> Result<T, SearchError>
    where
        T: serde::de::DeserializeOwned + Send;
}
```

This makes later testing easier without inline test clutter in implementation files, which matches your preference.

## Provider DTO -> Domain mapper

Never deserialize Brave JSON straight into your domain model.

Use:

* `dto.rs` for Brave wire structs
* `mapper.rs` for `BraveDto -> SearchResponse`

That prevents provider schema drift from contaminating the domain.

---

# 5. Brave-specific shape

For v0, Brave only, but still behind the provider trait.

Brave’s docs show separate search APIs for web, news, image, and video, so model your Brave provider as one provider with internal dispatch by `SearchType`, not four separate provider implementations yet. ([Brave][1])

Example:

```rust
pub struct BraveProvider<C: HttpClient> {
    client: C,
    config: BraveConfig,
}
```

```rust
pub struct BraveConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_secs: u64,
}
```

Then in `search()`:

* inspect `query.search_type`
* choose Brave endpoint
* build provider request params
* call HTTP client
* map DTO to normalized `SearchResponse`

---

# 6. Interfaces that matter now

You do **not** need many interfaces yet.

You need exactly these:

* `SearchProvider`
* `HttpClient`
* maybe `OutputRenderer` if you want JSON/table/text modes cleanly separated

Example:

```rust
pub trait OutputRenderer {
    fn render(&self, response: &SearchResponse) -> Result<String, SearchError>;
}
```

That is enough. Do not overdo abstraction.

---

# 7. Types to avoid

Do **not** do these:

* `HashMap<String, Value>` as result payload
* one mega `SearchResult` with 20 `Option<T>` fields
* provider-specific structs in app layer
* direct `reqwest` calls inside CLI handlers
* traits for everything

Keep the polymorphism at the provider seam only.

---

# 8. Initial build sequence

This is the order I would implement it:

1. `domain/` types
2. `SearchProvider` trait
3. `HttpClient` trait
4. `BraveConfig`
5. Brave DTO structs
6. Brave mapper
7. `BraveProvider`
8. `SearchService`
9. CLI args
10. output rendering

No tests inline. Build first, then test around the boundaries.

---

# 9. Recommended first-pass contract

This is the minimum useful architecture:

```rust
SearchQuery -> SearchService -> dyn SearchProvider -> BraveProvider
                                            |
                                            v
                                      HttpClient trait
                                            |
                                            v
                                      reqwest adapter

Brave JSON DTO -> mapper -> SearchResponse -> renderer
```

That is clean, extensible, and still small.

---

# 10. What I would lock in right now

Use these as non-negotiables:

* domain model is provider-agnostic
* provider JSON never leaks outside `providers/brave`
* provider chosen through trait boundary
* HTTP isolated behind a transport interface
* result model normalized by content type
* testing happens after implementation, at seam boundaries

That gives you a real foundation instead of fake “future-proofing.”

Next step should be the actual Rust skeleton: exact structs, traits, and file contents for `domain/`, `providers/brave/`, and `app/`.

[1]: https://api-dashboard.search.brave.com/documentation "Brave Search - API"

