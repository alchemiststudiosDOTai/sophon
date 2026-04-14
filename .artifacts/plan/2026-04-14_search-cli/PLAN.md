---
title: "Search CLI implementation plan"
link: "search-cli-plan"
type: implementation_plan
ontological_relations:
  - relates_to: [[PRD]]
tags: [plan, search-cli, rust, brave, coding]
uuid: "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
created_at: "2026-04-14T12:45:00Z"
parent_research: "PRD.md"
git_commit_at_plan: "618c875"
---

## Goal

Build a Rust CLI application that performs web/news/images/video searches via the Brave Search API. The architecture must be provider-agnostic at the domain and application layers, with Brave implemented as the first provider behind a trait boundary.

**Out of scope**: CI/CD, packaging (deb/homebrew), adding additional providers (Exa/Tavily), user documentation, deployment.

## Scope & Assumptions

**IN scope**:
- Rust project bootstrapping with Cargo
- Domain types (`SearchQuery`, `SearchResponse`, `SearchResult`, errors)
- `SearchProvider` trait with capabilities
- `HttpClient` transport abstraction
- Brave provider implementation (DTOs, mapper, `BraveProvider`)
- Application orchestration (`SearchService`)
- CLI argument parsing with `clap`
- Text output rendering
- Basic boundary tests

**OUT of scope**:
- Additional search providers
- JSON/table output modes (text only for v0)
- Pagination execution (token modeled but not CLI-exposed)
- Configuration files (API key read from env only)

**Assumptions**:
- Stable Rust toolchain available
- `reqwest` + `tokio` for async HTTP
- `serde` + `serde_json` for serialization
- `thiserror` for error types
- `async-trait` for trait async methods
- `.env` file present with `BRAVE_API_KEY` (existing)

## Deliverables

- `Cargo.toml` and `src/main.rs`
- `src/domain/{mod.rs,query.rs,result.rs,provider.rs,error.rs,types.rs}`
- `src/providers/{mod.rs,brave/{mod.rs,client.rs,config.rs,dto.rs,mapper.rs}}`
- `src/transport/{mod.rs,http.rs}`
- `src/app/{mod.rs,search_service.rs}`
- `src/cli/{mod.rs,args.rs,output.rs}`
- Unit tests for mapper and boundary tests for service

## Readiness

- Repository cloned and `.env` present with `BSAqJrJYLQsHK0YGQR82odpW20MuDel brave`
- Rust toolchain installed (`cargo --version` works)
- Internet access for fetching crates

## Milestones

- **M1**: Skeleton & domain types — project compiles with domain layer complete
- **M2**: Provider layer — Brave DTOs, mapper, and `BraveProvider` compile and can call Brave API
- **M3**: App & CLI — `SearchService`, CLI args, output rendering wired together
- **M4**: Tests & integration — basic tests pass and CLI runs end-to-end against Brave

## Ticket Index

<!-- TICKET_INDEX:START -->

| Task | Title | Ticket |
|---|---|---|
| T001 | Bootstrap Rust project and dependencies | [tickets/T001.md](tickets/T001.md) |
| T002 | Implement domain core types | [tickets/T002.md](tickets/T002.md) |
| T003 | Implement transport layer (HttpClient trait + reqwest adapter) | [tickets/T003.md](tickets/T003.md) |
| T004 | Implement SearchProvider trait and capabilities | [tickets/T004.md](tickets/T004.md) |
| T005 | Implement Brave DTOs | [tickets/T005.md](tickets/T005.md) |
| T006 | Implement Brave mapper | [tickets/T006.md](tickets/T006.md) |
| T007 | Implement BraveProvider | [tickets/T007.md](tickets/T007.md) |
| T008 | Implement SearchService | [tickets/T008.md](tickets/T008.md) |
| T009 | Implement CLI argument parsing | [tickets/T009.md](tickets/T009.md) |
| T010 | Implement output rendering | [tickets/T010.md](tickets/T010.md) |
| T011 | Wire main.rs and run end-to-end | [tickets/T011.md](tickets/T011.md) |
| T012 | Add boundary tests and CI-ready verification | [tickets/T012.md](tickets/T012.md) |

<!-- TICKET_INDEX:END -->

## Work Breakdown (Tasks)

### T001: Bootstrap Rust project and dependencies

**Summary**: Initialize Cargo project and add all required dependencies to `Cargo.toml`.

**Owner**: backend

**Estimate**: 15m

**Dependencies**: <none>

**Target milestone**: M1

**Acceptance test**: `cargo check` runs without errors on a bare `main.rs`.

**Files/modules touched**:
- `Cargo.toml`
- `src/main.rs`

**Steps**:
1. Run `cargo init --name search-cli` in the repo root.
2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   tokio = { version = "1", features = ["full"] }
   reqwest = { version = "0.12", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   thiserror = "1.0"
   async-trait = "0.1"
   clap = { version = "4", features = ["derive"] }
   dotenvy = "0.15"
   ```
3. Replace `src/main.rs` with a minimal `async fn main() {}`.
4. Run `cargo check` and confirm success.

---

### T002: Implement domain core types

**Summary**: Create all provider-agnostic domain types, enums, and the top-level error type.

**Owner**: backend

**Estimate**: 30m

**Dependencies**: T001

**Target milestone**: M1

**Acceptance test**: `cargo check` passes with all domain modules compiling.

**Files/modules touched**:
- `src/domain/mod.rs`
- `src/domain/types.rs`
- `src/domain/query.rs`
- `src/domain/result.rs`
- `src/domain/error.rs`

**Steps**:
1. Create `src/domain/mod.rs` that re-exports all submodules:
   ```rust
   pub mod error;
   pub mod query;
   pub mod result;
   pub mod types;
   ```
2. In `src/domain/types.rs`, define:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum SearchType { Web, News, Images, Videos }

   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum SafeSearch { Off, Moderate, Strict }

   #[derive(Debug, Clone, Copy, PartialEq, Eq)]
   pub enum TimeRange { Day, Week, Month, Year }
   ```
3. In `src/domain/query.rs`, define:
   ```rust
   use crate::domain::types::*;

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
4. In `src/domain/result.rs`, define:
   ```rust
   #[derive(Debug, Clone, PartialEq, Eq)]
   pub struct PageToken(pub String);

   #[derive(Debug, Clone, PartialEq)]
   pub struct SearchResponse {
       pub query: String,
       pub provider: String,
       pub results: Vec<SearchResult>,
       pub total_estimated: Option<u64>,
       pub next_page: Option<PageToken>,
   }

   #[derive(Debug, Clone, PartialEq)]
   pub enum SearchResult {
       Web(WebResult),
       News(NewsResult),
       Image(ImageResult),
       Video(VideoResult),
   }

   #[derive(Debug, Clone, PartialEq)]
   pub struct WebResult {
       pub title: String,
       pub url: String,
       pub snippet: Option<String>,
       pub display_url: Option<String>,
   }

   #[derive(Debug, Clone, PartialEq)]
   pub struct NewsResult {
       pub title: String,
       pub url: String,
       pub snippet: Option<String>,
       pub source: Option<String>,
       pub published_at: Option<String>,
   }

   #[derive(Debug, Clone, PartialEq)]
   pub struct ImageResult {
       pub title: String,
       pub url: String,
       pub thumbnail_url: Option<String>,
       pub source: Option<String>,
   }

   #[derive(Debug, Clone, PartialEq)]
   pub struct VideoResult {
       pub title: String,
       pub url: String,
       pub thumbnail_url: Option<String>,
       pub duration: Option<String>,
       pub published_at: Option<String>,
   }
   ```
   Note: use `String` for `provider` in `SearchResponse` to avoid circular dependency issues; we will use `"brave".to_string()`.
5. In `src/domain/error.rs`, define:
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
6. Run `cargo check` and fix any compilation errors.

---

### T003: Implement transport layer (HttpClient trait + reqwest adapter)

**Summary**: Define the `HttpClient` trait and a `reqwest`-based adapter so providers are decoupled from the HTTP library.

**Owner**: backend

**Estimate**: 25m

**Dependencies**: T001, T002

**Target milestone**: M2

**Acceptance test**: `cargo check` compiles `transport/` and a simple `ReqwestHttpClient` struct exists.

**Files/modules touched**:
- `src/transport/mod.rs`
- `src/transport/http.rs`

**Steps**:
1. Create `src/transport/mod.rs` that declares `pub mod http;`.
2. In `src/transport/http.rs`, add:
   ```rust
   use async_trait::async_trait;
   use crate::domain::error::SearchError;

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
3. Implement `ReqwestHttpClient`:
   ```rust
   use reqwest::Client;

   pub struct ReqwestHttpClient {
       client: Client,
   }

   impl ReqwestHttpClient {
       pub fn new() -> Self {
           Self { client: Client::new() }
       }
   }

   #[async_trait]
   impl HttpClient for ReqwestHttpClient {
       async fn get_json<T>(
           &self,
           url: &str,
           headers: Vec<(String, String)>,
           query: Vec<(String, String)>,
       ) -> Result<T, SearchError>
       where
           T: serde::de::DeserializeOwned + Send,
       {
           let mut req = self.client.get(url);
           for (k, v) in headers {
               req = req.header(k, v);
           }
           req = req.query(&query);
           let resp = req.send().await.map_err(|e| SearchError::Transport(e.to_string()))?;

           if resp.status() == 401 || resp.status() == 403 {
               return Err(SearchError::Auth);
           }
           if resp.status() == 429 {
               return Err(SearchError::RateLimited);
           }
           if !resp.status().is_success() {
               let text = resp.text().await.unwrap_or_default();
               return Err(SearchError::Provider(format!("HTTP {}: {}", resp.status(), text)));
           }

           resp.json::<T>().await.map_err(|e| SearchError::Decode(e.to_string()))
       }
   }
   ```
4. Run `cargo check`.

---

### T004: Implement SearchProvider trait and capabilities

**Summary**: Define the provider trait and capabilities struct in the domain layer so the app layer depends on abstractions.

**Owner**: backend

**Estimate**: 20m

**Dependencies**: T002

**Target milestone**: M2

**Acceptance test**: `cargo check` passes with `SearchProvider` trait and `ProviderCapabilities` defined.

**Files/modules touched**:
- `src/domain/mod.rs`
- `src/domain/provider.rs`

**Steps**:
1. Add `pub mod provider;` to `src/domain/mod.rs`.
2. In `src/domain/provider.rs`, define:
   ```rust
   use async_trait::async_trait;
   use crate::domain::query::SearchQuery;
   use crate::domain::result::SearchResponse;
   use crate::domain::error::SearchError;

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

   #[async_trait]
   pub trait SearchProvider: Send + Sync {
       fn id(&self) -> String;
       fn capabilities(&self) -> ProviderCapabilities;
       async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError>;
   }
   ```
3. Run `cargo check`.

---

### T005: Implement Brave DTOs

**Summary**: Create Brave-specific request/response structs to isolate provider JSON shapes from the domain.

**Owner**: backend

**Estimate**: 25m

**Dependencies**: T001, T002

**Target milestone**: M2

**Acceptance test**: `cargo check` compiles `providers/brave/dto.rs` with all structs deriving `Deserialize`.

**Files/modules touched**:
- `src/providers/mod.rs`
- `src/providers/brave/mod.rs`
- `src/providers/brave/dto.rs`

**Steps**:
1. Create `src/providers/mod.rs` with `pub mod brave;`.
2. Create `src/providers/brave/mod.rs` that declares:
   ```rust
   pub mod client;
   pub mod config;
   pub mod dto;
   pub mod mapper;
   ```
3. In `src/providers/brave/dto.rs`, define the following `Deserialize` structs based on Brave API v1 response shapes:
   ```rust
   use serde::Deserialize;

   #[derive(Debug, Deserialize)]
   pub struct BraveWebResponse {
       pub query: Option<BraveQuery>,
       pub web: Option<BraveWebResults>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveQuery {
       pub original: Option<String>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveWebResults {
       pub results: Option<Vec<BraveWebResult>>,
       pub total: Option<u64>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveWebResult {
       pub title: Option<String>,
       pub url: Option<String>,
       pub description: Option<String>,
       pub display_url: Option<String>,
   }

   // News
   #[derive(Debug, Deserialize)]
   pub struct BraveNewsResponse {
       pub query: Option<BraveQuery>,
       pub news: Option<BraveNewsResults>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveNewsResults {
       pub results: Option<Vec<BraveNewsResult>>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveNewsResult {
       pub title: Option<String>,
       pub url: Option<String>,
       pub description: Option<String>,
       pub source: Option<String>,
       pub age: Option<String>,
   }

   // Images
   #[derive(Debug, Deserialize)]
   pub struct BraveImagesResponse {
       pub query: Option<BraveQuery>,
       pub image_results: Option<Vec<BraveImageResult>>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveImageResult {
       pub title: Option<String>,
       pub url: Option<String>,
       pub thumbnail: Option<BraveThumbnail>,
       pub source: Option<String>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveThumbnail {
       pub src: Option<String>,
   }

   // Videos
   #[derive(Debug, Deserialize)]
   pub struct BraveVideosResponse {
       pub query: Option<BraveQuery>,
       pub videos: Option<BraveVideosResults>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveVideosResults {
       pub results: Option<Vec<BraveVideoResult>>,
   }

   #[derive(Debug, Deserialize)]
   pub struct BraveVideoResult {
       pub title: Option<String>,
       pub url: Option<String>,
       pub thumbnail: Option<BraveThumbnail>,
       pub duration: Option<String>,
       pub age: Option<String>,
   }
   ```
4. Run `cargo check`.

---

### T006: Implement Brave mapper

**Summary**: Write mapper functions that convert Brave DTOs into domain `SearchResponse` and `SearchResult` enums.

**Owner**: backend

**Estimate**: 30m

**Dependencies**: T004, T005

**Target milestone**: M2

**Acceptance test**: Unit tests in `mapper.rs` verify that sample Brave DTOs map to correct domain results.

**Files/modules touched**:
- `src/providers/brave/mapper.rs`

**Steps**:
1. In `src/providers/brave/mapper.rs`, implement four mapper functions:
   ```rust
   use crate::domain::result::*;
   use crate::providers::brave::dto::*;

   pub fn map_web_response(dto: BraveWebResponse) -> SearchResponse {
       let query_text = dto.query.and_then(|q| q.original).unwrap_or_default();
       let results = dto.web.and_then(|w| w.results).unwrap_or_default();
       SearchResponse {
           query: query_text,
           provider: "brave".to_string(),
           total_estimated: dto.web.as_ref().and_then(|w| w.total),
           next_page: None,
           results: results.into_iter().map(|r| {
               SearchResult::Web(WebResult {
                   title: r.title.unwrap_or_default(),
                   url: r.url.unwrap_or_default(),
                   snippet: r.description,
                   display_url: r.display_url,
               })
           }).collect(),
       }
   }
   ```
2. Implement `map_news_response`, `map_images_response`, `map_videos_response` following the same pattern, mapping to `SearchResult::News`, `SearchResult::Image`, and `SearchResult::Video` respectively.
3. Add `#[cfg(test)]` module with basic unit tests that construct minimal DTOs and assert the mapped domain output.
4. Run `cargo test --lib` for the mapper tests.

---

### T007: Implement BraveProvider

**Summary**: Build the concrete `BraveProvider` that implements `SearchProvider`, dispatches by `SearchType`, and calls the Brave API.

**Owner**: backend

**Estimate**: 45m

**Dependencies**: T003, T004, T005, T006

**Target milestone**: M2

**Acceptance test**: A simple integration test (or manual `cargo run`) performs a web search via Brave and prints results. For automated acceptance, mock the `HttpClient` trait to return a sample JSON string and verify `BraveProvider` returns a `SearchResponse`.

**Files/modules touched**:
- `src/providers/brave/client.rs`
- `src/providers/brave/config.rs`

**Steps**:
1. In `src/providers/brave/config.rs`, define:
   ```rust
   #[derive(Debug, Clone)]
   pub struct BraveConfig {
       pub api_key: String,
       pub base_url: String,
   }

   impl BraveConfig {
       pub fn from_env() -> Result<Self, std::env::VarError> {
           let api_key = std::env::var("BRAVE_API_KEY")?;
           Ok(Self {
               api_key,
               base_url: "https://api.search.brave.com/res/v1".to_string(),
           })
       }
   }
   ```
2. In `src/providers/brave/client.rs`, define:
   ```rust
   use async_trait::async_trait;
   use crate::domain::error::SearchError;
   use crate::domain::provider::{ProviderCapabilities, SearchProvider};
   use crate::domain::query::SearchQuery;
   use crate::domain::result::SearchResponse;
   use crate::domain::types::SearchType;
   use crate::transport::http::HttpClient;
   use crate::providers::brave::config::BraveConfig;
   use crate::providers::brave::dto::*;
   use crate::providers::brave::mapper::*;

   pub struct BraveProvider<C: HttpClient> {
       client: C,
       config: BraveConfig,
   }

   impl<C: HttpClient> BraveProvider<C> {
       pub fn new(client: C, config: BraveConfig) -> Self {
           Self { client, config }
       }
   }

   #[async_trait]
   impl<C: HttpClient> SearchProvider for BraveProvider<C> {
       fn id(&self) -> String {
           "brave".to_string()
       }

       fn capabilities(&self) -> ProviderCapabilities {
           ProviderCapabilities {
               web: true,
               news: true,
               images: true,
               videos: true,
               pagination: false,
               safe_search: true,
               time_range_filter: true,
           }
       }

       async fn search(&self, query: &SearchQuery) -> Result<SearchResponse, SearchError> {
           let endpoint = match query.search_type {
               SearchType::Web => "web/search",
               SearchType::News => "news/search",
               SearchType::Images => "images/search",
               SearchType::Videos => "videos/search",
           };
           let url = format!("{}/{}", self.config.base_url, endpoint);

           let mut params: Vec<(String, String)> = vec![
               ("q".to_string(), query.text.clone()),
           ];
           if let Some(limit) = query.limit {
               params.push(("count".to_string(), limit.to_string()));
           }
           if let Some(offset) = query.offset {
               params.push(("offset".to_string(), offset.to_string()));
           }
           if let Some(ss) = query.safe_search {
               let val = match ss {
                   crate::domain::types::SafeSearch::Off => "off",
                   crate::domain::types::SafeSearch::Moderate => "moderate",
                   crate::domain::types::SafeSearch::Strict => "strict",
               };
               params.push(("safesearch".to_string(), val.to_string()));
           }
           if let Some(ref country) = query.country {
               params.push(("country".to_string(), country.clone()));
           }
           if let Some(ref lang) = query.language {
               params.push(("search_lang".to_string(), lang.clone()));
           }
           if let Some(ref tr) = query.time_range {
               let val = match tr {
                   crate::domain::types::TimeRange::Day => "day",
                   crate::domain::types::TimeRange::Week => "week",
                   crate::domain::types::TimeRange::Month => "month",
                   crate::domain::types::TimeRange::Year => "year",
               };
               params.push(("freshness".to_string(), val.to_string()));
           }

           let headers = vec![
               ("Accept".to_string(), "application/json".to_string()),
               ("X-Subscription-Token".to_string(), self.config.api_key.clone()),
           ];

           match query.search_type {
               SearchType::Web => {
                   let dto: BraveWebResponse = self.client.get_json(&url, headers, params).await?;
                   Ok(map_web_response(dto))
               }
               SearchType::News => {
                   let dto: BraveNewsResponse = self.client.get_json(&url, headers, params).await?;
                   Ok(map_news_response(dto))
               }
               SearchType::Images => {
                   let dto: BraveImagesResponse = self.client.get_json(&url, headers, params).await?;
                   Ok(map_images_response(dto))
               }
               SearchType::Videos => {
                   let dto: BraveVideosResponse = self.client.get_json(&url, headers, params).await?;
                   Ok(map_videos_response(dto))
               }
           }
       }
   }
   ```
3. Add a `#[cfg(test)]` mock `HttpClient` that returns a hardcoded JSON string and assert `BraveProvider::search` produces the expected `SearchResponse`.
4. Run `cargo test --lib`.

---

### T008: Implement SearchService

**Summary**: Build the application-layer `SearchService` that accepts a `dyn SearchProvider` and orchestrates searches.

**Owner**: backend

**Estimate**: 20m

**Dependencies**: T004, T007

**Target milestone**: M3

**Acceptance test**: `cargo check` compiles `SearchService` and a test proves it delegates to a mocked `SearchProvider`.

**Files/modules touched**:
- `src/app/mod.rs`
- `src/app/search_service.rs`

**Steps**:
1. Create `src/app/mod.rs` with `pub mod search_service;`.
2. In `src/app/search_service.rs`, define:
   ```rust
   use crate::domain::error::SearchError;
   use crate::domain::provider::SearchProvider;
   use crate::domain::query::SearchQuery;
   use crate::domain::result::SearchResponse;

   pub struct SearchService {
       provider: Box<dyn SearchProvider>,
   }

   impl SearchService {
       pub fn new(provider: Box<dyn SearchProvider>) -> Self {
           Self { provider }
       }

       pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse, SearchError> {
           self.provider.search(&query).await
       }
   }
   ```
3. Add a test that creates a mock `SearchProvider` (implement the trait on a simple struct) and verify `SearchService::search` returns the mock response.
4. Run `cargo test --lib`.

---

### T009: Implement CLI argument parsing

**Summary**: Use `clap` to parse user input into a `SearchCommand` struct with query, search type, limit, and output format.

**Owner**: backend

**Estimate**: 25m

**Dependencies**: T001, T002

**Target milestone**: M3

**Acceptance test**: `cargo run -- --help` displays all options and `cargo run -- "rust"` defaults to web search.

**Files/modules touched**:
- `src/cli/mod.rs`
- `src/cli/args.rs`

**Steps**:
1. Create `src/cli/mod.rs` with:
   ```rust
   pub mod args;
   pub mod output;
   ```
2. In `src/cli/args.rs`, define:
   ```rust
   use clap::{Parser, ValueEnum};
   use crate::domain::types::SearchType;

   #[derive(Debug, Clone, ValueEnum)]
   pub enum CliSearchType {
       Web,
       News,
       Images,
       Videos,
   }

   impl From<CliSearchType> for SearchType {
       fn from(val: CliSearchType) -> Self {
           match val {
               CliSearchType::Web => SearchType::Web,
               CliSearchType::News => SearchType::News,
               CliSearchType::Images => SearchType::Images,
               CliSearchType::Videos => SearchType::Videos,
           }
       }
   }

   #[derive(Parser, Debug)]
   #[command(name = "search-cli")]
   #[command(about = "Provider-agnostic search CLI")]
   pub struct CliArgs {
       #[arg(help = "Search query text")]
       pub query: String,

       #[arg(short, long, value_enum, default_value = "web")]
       pub search_type: CliSearchType,

       #[arg(short, long)]
       pub limit: Option<usize>,

       #[arg(long)]
       pub offset: Option<usize>,

       #[arg(long, value_enum)]
       pub safe_search: Option<crate::domain::types::SafeSearch>,

       #[arg(long)]
       pub country: Option<String>,

       #[arg(long)]
       pub language: Option<String>,
   }
   ```
   Note: `SafeSearch` already derives the necessary traits, but `clap::ValueEnum` requires additional derive. If compilation fails, add a manual mapping instead of using `value_enum` on `SafeSearch`. Create a `CliSafeSearch` enum and map it if needed.
3. Update `src/main.rs` to parse args:
   ```rust
   use clap::Parser;
   use search_cli::cli::args::CliArgs;

   #[tokio::main]
   async fn main() {
       let _args = CliArgs::parse();
       println!("{:?}", _args);
   }
   ```
   (Temporarily expose `cli` module from a lib or keep everything in `main.rs` if the project is bin-only.)
   
   **Important**: Since this is a binary crate, either make `main.rs` contain the modules directly, or create `src/lib.rs`. To keep it simple, declare modules in `main.rs`:
   ```rust
   mod app;
   mod cli;
   mod domain;
   mod providers;
   mod transport;
   ```
   Update `main.rs` to include these mod declarations and parse args.
4. Run `cargo run -- --help` and confirm output.
5. Run `cargo run -- "rust"` and confirm it prints the parsed args.

---

### T010: Implement output rendering

**Summary**: Create a simple text renderer that prints `SearchResponse` results in a human-readable format.

**Owner**: backend

**Estimate**: 20m

**Dependencies**: T002, T009

**Target milestone**: M3

**Acceptance test**: A test provides a `SearchResponse` and the renderer returns a string containing all result titles and URLs.

**Files/modules touched**:
- `src/cli/output.rs`

**Steps**:
1. In `src/cli/output.rs`, define:
   ```rust
   use crate::domain::result::{SearchResponse, SearchResult};

   pub fn render_text(response: &SearchResponse) -> String {
       let mut lines = vec![
           format!("Provider: {}", response.provider),
           format!("Query: {}", response.query),
           format!("Results: {}", response.results.len()),
           String::new(),
       ];
       for (i, result) in response.results.iter().enumerate() {
           match result {
               SearchResult::Web(r) => {
                   lines.push(format!("{}. [{}]", i + 1, r.title));
                   lines.push(format!("   URL: {}", r.url));
                   if let Some(s) = &r.snippet {
                       lines.push(format!("   {}", s));
                   }
               }
               SearchResult::News(r) => {
                   lines.push(format!("{}. [NEWS] {}", i + 1, r.title));
                   lines.push(format!("   URL: {}", r.url));
                   if let Some(s) = &r.source {
                       lines.push(format!("   Source: {}", s));
                   }
               }
               SearchResult::Image(r) => {
                   lines.push(format!("{}. [IMAGE] {}", i + 1, r.title));
                   lines.push(format!("   URL: {}", r.url));
               }
               SearchResult::Video(r) => {
                   lines.push(format!("{}. [VIDEO] {}", i + 1, r.title));
                   lines.push(format!("   URL: {}", r.url));
               }
           }
           lines.push(String::new());
       }
       lines.join("\n")
   }
   ```
2. Add a unit test that constructs a `SearchResponse` with mixed results and asserts the rendered string contains expected substrings.
3. Run `cargo test --lib`.

---

### T011: Wire main.rs and run end-to-end

**Summary**: Connect CLI parsing, `SearchService`, `BraveProvider`, and output rendering in `main.rs` so the CLI performs a live search.

**Owner**: backend

**Estimate**: 25m

**Dependencies**: T007, T008, T009, T010

**Target milestone**: M4

**Acceptance test**: `cargo run -- "rust programming"` successfully queries Brave and prints at least one search result. If API key is missing, it prints a clean error message.

**Files/modules touched**:
- `src/main.rs`

**Steps**:
1. In `src/main.rs`, replace contents with:
   ```rust
   mod app;
   mod cli;
   mod domain;
   mod providers;
   mod transport;

   use clap::Parser;
   use cli::args::CliArgs;
   use cli::output::render_text;
   use app::search_service::SearchService;
   use domain::query::SearchQuery;
   use domain::types::SafeSearch;
   use providers::brave::client::BraveProvider;
   use providers::brave::config::BraveConfig;
   use transport::http::ReqwestHttpClient;

   #[tokio::main]
   async fn main() {
       dotenvy::dotenv().ok();

       let args = CliArgs::parse();

       let config = match BraveConfig::from_env() {
           Ok(c) => c,
           Err(e) => {
               eprintln!("Failed to load Brave config: {}", e);
               std::process::exit(1);
           }
       };

       let client = ReqwestHttpClient::new();
       let provider = BraveProvider::new(client, config);
       let service = SearchService::new(Box::new(provider));

       let query = SearchQuery {
           text: args.query,
           search_type: args.search_type.into(),
           limit: args.limit,
           offset: args.offset,
           safe_search: args.safe_search,
           country: args.country,
           language: args.language,
           time_range: None,
       };

       match service.search(query).await {
           Ok(response) => {
               println!("{}", render_text(&response));
           }
           Err(e) => {
               eprintln!("Search failed: {}", e);
               std::process::exit(1);
           }
       }
   }
   ```
   Note: If `SafeSearch` does not implement `clap::ValueEnum`, adjust `CliArgs` to use a local `CliSafeSearch` enum and map it in the `SearchQuery` construction.
2. Ensure `cargo check` passes.
3. Run `cargo run -- "rust programming"` and verify it returns search results.
4. Run `cargo run -- "rust" --search-type news` and verify news results.
5. Run `cargo run -- "cats" --search-type images --limit 3` and verify image results.

---

### T012: Add boundary tests and CI-ready verification

**Summary**: Add integration-style tests at the app and provider boundaries, and ensure `cargo test` passes cleanly.

**Owner**: backend

**Estimate**: 30m

**Dependencies**: T011

**Target milestone**: M4

**Acceptance test**: `cargo test` passes with at least one test for mapper, one for `BraveProvider` with mock HTTP, and one for `SearchService` with mock provider.

**Files/modules touched**:
- `src/providers/brave/mapper.rs`
- `src/providers/brave/client.rs`
- `src/app/search_service.rs`

**Steps**:
1. In `src/providers/brave/mapper.rs`, ensure the existing `#[cfg(test)]` module covers all four search types.
2. In `src/providers/brave/client.rs`, ensure the mock HTTP test validates that headers include `X-Subscription-Token` and that query params include `q`, `count`, and `safesearch`.
3. In `src/app/search_service.rs`, ensure the mock provider test validates delegation.
4. Run `cargo test` and fix any failures.
5. Run `cargo clippy` (if available) and fix warnings.

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Brave API schema differs from documented DTOs | Start with minimal fields; mapper defaults `Option` values to empty strings rather than failing |
| `clap` `ValueEnum` incompatibility with domain enums | Use separate CLI enums and map them explicitly to domain types |
| Rate limiting during manual testing | Keep query counts low; use `limit` flag; mock HTTP for automated tests |
| Missing env var in CI/test environments | `BraveConfig::from_env()` returns clear error; tests use mock config |

## Test Strategy

- **Mapper tests**: Construct minimal DTOs, assert domain output (T006).
- **Provider tests**: Mock `HttpClient` to return JSON, assert `BraveProvider` produces `SearchResponse` (T007).
- **Service tests**: Mock `SearchProvider`, assert `SearchService` delegates correctly (T008).
- **Renderer tests**: Provide `SearchResponse`, assert string output contains expected data (T010).
- **E2E manual**: Run CLI with live Brave API for web, news, images, and video queries (T011).

## References

- PRD.md sections 1–10 (domain design, provider trait, Brave mapping, module layout)
- `Cargo.toml` dependency versions aligned with PRD async/HTTP stack
- Brave Search API docs: https://api-dashboard.search.brave.com/documentation

## Final Gate

- **Output summary**: plan dir `.artifacts/plan/2026-04-14_search-cli/`, 4 milestones, 12 tickets
- **Next step**: proceed to execute-phase with `.artifacts/plan/2026-04-14_search-cli/PLAN.md`
