---
title: "Response Shape"
when_to_read:
  - "When integrating with sophon-cli output or consuming SearchResponse/SearchBatchResponse."
  - "When mapping provider DTOs into domain results or adding new result types."
summary: "Reference for every field in the domain search response, including single-provider, fan-out, error, and CLI text representations."
ontology_relations:
  - relation: "part_of"
    target: "docs/SUMMARY.md"
    note: "Belongs to the mdBook documentation set."
---

# Response Shape

When you run a search, the domain layer returns provider-agnostic types. The CLI renders them as plain text; if you are using the library surface directly, you work with these structs and enums.

## Single provider

```rust
pub struct SearchResponse {
    pub query: String,
    pub provider: String,
    pub results: Vec<SearchResult>,
    pub total_estimated: Option<u64>,
    pub next_page: Option<PageToken>,
}
```

| Field | Meaning |
|-------|---------|
| `query` | Normalized query text that was sent to the provider. |
| `provider` | Provider identifier, e.g. `"brave"` or `"exa"`. |
| `results` | Ordered list of hits. Each item is one of four variants. |
| `total_estimated` | Provider-reported total hit count, if available. Not rendered in CLI text output. |
| `next_page` | Opaque token for fetching the next page, if the provider supports pagination. Not rendered in CLI text output. |

---

## `SearchResult` variants

### `SearchResult::Web`

```rust
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub display_url: Option<String>,
}
```

**CLI text rendering**

```text
1. [Rust Programming Language]
   URL: https://www.rust-lang.org
   A language empowering everyone to build reliable software.
```

- `display_url` is not shown in text output.

---

### `SearchResult::News`

```rust
pub struct NewsResult {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub source: Option<String>,
    pub published_at: Option<String>,
}
```

**CLI text rendering**

```text
2. [NEWS] Rust 1.85 Released
   URL: https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html
   Source: Rust Blog
   Major compiler improvements and new language features.
```

- `published_at` is not shown in text output.

---

### `SearchResult::Image`

```rust
pub struct ImageResult {
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub source: Option<String>,
}
```

**CLI text rendering**

```text
3. [IMAGE] Rust Logo
   URL: https://www.rust-lang.org/static/images/rust-logo-blk.svg
```

- `thumbnail_url` and `source` are not shown in text output.

---

### `SearchResult::Video`

```rust
pub struct VideoResult {
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub published_at: Option<String>,
}
```

**CLI text rendering**

```text
4. [VIDEO] Rust Tutorial
   URL: https://example.com/rust-tutorial
```

- `thumbnail_url`, `duration`, and `published_at` are not shown in text output.

---

## Multi-provider fanout (`--provider all`)

When multiple providers are queried, the result is a `SearchBatchResponse`:

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

| Field | Meaning |
|-------|---------|
| `query` | The shared query text. |
| `responses` | One `SearchResponse` per provider that succeeded. |
| `failures` | One entry per provider that failed; the error is preserved so callers know why. |

**CLI text rendering**

```text
Query: rust programming
Providers succeeded: 2
Providers failed: 0

== brave ==
Provider: brave
Query: rust programming
Results: 3
1. [Rust Programming Language]
   URL: https://www.rust-lang.org
   ...

== exa ==
Provider: exa
Query: rust programming
Results: 2
1. [Systems Programming with Rust]
   URL: https://example.com/intro
   ...
```

If a provider fails:

```text
== Failures ==
- exa: invalid query: unsupported
```

---

## Error shape

The single public error type across all layers is `SearchError`:

```rust
pub enum SearchError {
    InvalidQuery(String),
    Unauthorized,
    RateLimited,
    ProviderError(String),
    NetworkError(String),
    SerializationError(String),
    Unexpected(String),
}
```

In text output, errors are formatted as lower-case messages prefixed by the variant:

- `InvalidQuery("foo")` → `invalid query: foo`
- `Unauthorized` → `unauthorized`
- `RateLimited` → `rate limited`
- `ProviderError("down")` → `provider error: down`
- `NetworkError("timeout")` → `network error: timeout`
- `SerializationError("bad json")` → `serialization error: bad json`
- `Unexpected("oops")` → `unexpected error: oops`

---

## Summary

- **One provider** → `SearchResponse` → `render_text`
- **All providers** → `SearchBatchResponse` → `render_fanout_text`
- **Each hit** → `SearchResult::{Web,News,Image,Video}` with typed fields
- **CLI text** shows `title`, `url`, and the most common human-readable fields; optional metadata (`display_url`, `published_at`, `thumbnail_url`, `duration`, `total_estimated`, `next_page`) is present in the domain type but omitted from text output.
