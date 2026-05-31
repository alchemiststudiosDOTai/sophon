---
title: "Research: Result URL surface"
when_to_read:
  - "When planning or implementing changes to URLs returned in search results."
  - "When checking how provider result URLs flow into CLI output."
summary: "Maps where search-result URLs are represented, mapped, preserved, rendered, and tested in sophon-cli."
research_type: "feature"
commitment_status: "decision-support"
research_goal: "Map the current code paths that carry provider-returned result URLs into normalized results and CLI text output."
uuid: "487FF3C3-E993-4D17-A9BA-7C6CA331773F"
created_at: "2026-05-31T11:15:23-05:00"
tags: [research, feature, urls, results, cli-output]
ontology_relations:
  - relation: "explores"
    target: "sophon-cli-result-url-surface"
    note: "Captures factual research about URL fields in normalized search results and output rendering."
  - relation: "relates_to"
    target: "docs/project/ARCHITECTURE.md"
    note: "Uses the documented provider-to-domain-to-CLI data flow as the research frame."
  - relation: "relates_to"
    target: "docs/artifacts/INDEX.md"
    note: "Stores the research in the tracked markdown artifact control plane."
---

# Exploration Memory

| Field | Value |
|---|---|
| Artifact Type | Exploration Memory |
| Status | Active |
| Date | 2026-05-31 |
| Owner | Codex |
| Research Type | `feature` |
| Commitment Status | `decision-support` |
| Related Artifacts | `docs/project/ARCHITECTURE.md`, `docs/project/PROJECT_CONTEXT.md`, `docs/artifacts/INDEX.md` |
| Related Files | `src/domain/result.rs`, `src/providers/brave/dto.rs`, `src/providers/brave/mapper.rs`, `src/providers/exa/dto.rs`, `src/providers/exa/mapper.rs`, `src/app/search_service.rs`, `src/app/fanout_search_service.rs`, `src/cli/output.rs`, `src/cli/runner.rs` |

## Research Intent

- Research type: `feature`
- Commitment status: `decision-support`
- Goal: map the current code paths that carry provider-returned result URLs into normalized domain results and CLI output.
- Out of scope: source-code changes, CLI UX changes, result-schema changes, provider API behavior changes, and deciding an implementation plan.

## Question

Where does sophon-cli currently represent, map, preserve, and print URLs returned from provider search results?

## Structure

- `src/domain/result.rs` defines normalized response and result structs for provider-agnostic results.
- `src/providers/brave/dto.rs` defines Brave response DTOs with optional URL fields.
- `src/providers/brave/mapper.rs` maps Brave DTO URL fields into normalized domain result URL fields.
- `src/providers/exa/dto.rs` defines Exa response DTOs with optional URL fields.
- `src/providers/exa/mapper.rs` maps Exa DTO URL fields into normalized domain result URL fields.
- `src/app/search_service.rs` delegates a single-provider search and returns the provider response.
- `src/app/fanout_search_service.rs` aggregates provider `SearchResponse` values into `SearchBatchResponse`.
- `src/cli/output.rs` renders single-provider and fan-out result text.
- `src/cli/runner.rs` prints rendered single-provider and fan-out output to stdout.

## Key Files

- `src/domain/result.rs:7` defines `SearchResponse`.
- `src/domain/result.rs:10` stores normalized results as `Vec<SearchResult>`.
- `src/domain/result.rs:29` defines `SearchResult::{Web, News, Image, Video}`.
- `src/domain/result.rs:37` defines `WebResult`.
- `src/domain/result.rs:39` defines `WebResult.url: String`.
- `src/domain/result.rs:45` defines `NewsResult`.
- `src/domain/result.rs:47` defines `NewsResult.url: String`.
- `src/domain/result.rs:54` defines `ImageResult`.
- `src/domain/result.rs:56` defines `ImageResult.url: String`.
- `src/domain/result.rs:62` defines `VideoResult`.
- `src/domain/result.rs:64` defines `VideoResult.url: String`.
- `src/domain/mod.rs:10` re-exports result structs from the domain module surface.
- `src/domain/provider.rs:18` defines the `SearchProvider` trait.
- `src/domain/provider.rs:23` returns `Result<SearchResponse, SearchError>` from `SearchProvider::search`.

## Provider URL Inputs

### Brave

- `src/providers/brave/dto.rs:21` defines `BraveWebResult`.
- `src/providers/brave/dto.rs:23` defines `BraveWebResult.url: Option<String>`.
- `src/providers/brave/dto.rs:41` defines `BraveNewsResult`.
- `src/providers/brave/dto.rs:43` defines `BraveNewsResult.url: Option<String>`.
- `src/providers/brave/dto.rs:57` defines `BraveImageResult`.
- `src/providers/brave/dto.rs:59` defines `BraveImageResult.url: Option<String>`.
- `src/providers/brave/dto.rs:82` defines `BraveVideoResult`.
- `src/providers/brave/dto.rs:84` defines `BraveVideoResult.url: Option<String>`.
- `src/providers/brave/client.rs:93` dispatches Brave web DTO responses to `map_web_response`.
- `src/providers/brave/client.rs:98` dispatches Brave news DTO responses to `map_news_response`.
- `src/providers/brave/client.rs:102` dispatches Brave image DTO responses to `map_images_response`.
- `src/providers/brave/client.rs:106` dispatches Brave video DTO responses to `map_videos_response`.

### Exa

- `src/providers/exa/dto.rs:52` defines `ExaSearchResponse`.
- `src/providers/exa/dto.rs:56` stores Exa response results as `Vec<ExaResult>`.
- `src/providers/exa/dto.rs:61` defines `ExaResult`.
- `src/providers/exa/dto.rs:63` defines `ExaResult.url: Option<String>`.
- `src/providers/exa/client.rs:121` dispatches Exa web responses to `map_web_response`.
- `src/providers/exa/client.rs:123` dispatches Exa news responses to `map_news_response`.
- `src/providers/exa/client.rs:97` reports Exa capabilities.
- `src/providers/exa/client.rs:101` sets Exa image support to `false`.
- `src/providers/exa/client.rs:102` sets Exa video support to `false`.

## Mapping Paths

### Brave DTO To Domain

- `src/providers/brave/mapper.rs:7` defines `map_web_response`.
- `src/providers/brave/mapper.rs:19` creates `SearchResult::Web`.
- `src/providers/brave/mapper.rs:21` maps `BraveWebResult.url` into `WebResult.url` with `unwrap_or_default()`.
- `src/providers/brave/mapper.rs:30` defines `map_news_response`.
- `src/providers/brave/mapper.rs:41` creates `SearchResult::News`.
- `src/providers/brave/mapper.rs:43` maps `BraveNewsResult.url` into `NewsResult.url` with `unwrap_or_default()`.
- `src/providers/brave/mapper.rs:53` defines `map_images_response`.
- `src/providers/brave/mapper.rs:64` creates `SearchResult::Image`.
- `src/providers/brave/mapper.rs:66` maps `BraveImageResult.url` into `ImageResult.url` with `unwrap_or_default()`.
- `src/providers/brave/mapper.rs:75` defines `map_videos_response`.
- `src/providers/brave/mapper.rs:86` creates `SearchResult::Video`.
- `src/providers/brave/mapper.rs:88` maps `BraveVideoResult.url` into `VideoResult.url` with `unwrap_or_default()`.

### Exa DTO To Domain

- `src/providers/exa/mapper.rs:10` defines `map_web_response`.
- `src/providers/exa/mapper.rs:12` creates `SearchResult::Web`.
- `src/providers/exa/mapper.rs:14` maps `ExaResult.url` into `WebResult.url` with `unwrap_or_default()`.
- `src/providers/exa/mapper.rs:21` defines `map_news_response`.
- `src/providers/exa/mapper.rs:23` creates `SearchResult::News`.
- `src/providers/exa/mapper.rs:25` maps `ExaResult.url` into `NewsResult.url` with `unwrap_or_default()`.
- `src/providers/exa/mapper.rs:33` defines the shared `map_response` helper.
- `src/providers/exa/mapper.rs:42` maps `dto.results` into normalized `SearchResult` values.

## Application Preservation Path

- `src/app/search_service.rs:13` defines `SearchService::search`.
- `src/app/search_service.rs:15` calls `self.provider.search(&query).await`.
- `src/app/search_service.rs:19` returns the provider result without rebuilding `SearchResponse`.
- `src/app/fanout_search_service.rs:14` defines `FanoutSearchService::search_all`.
- `src/app/fanout_search_service.rs:20` calls each provider search.
- `src/app/fanout_search_service.rs:21` pushes successful provider `SearchResponse` values into `responses`.
- `src/app/fanout_search_service.rs:29` returns `SearchBatchResponse` with the collected `responses`.

## CLI Rendering Path

- `src/cli/output.rs:3` defines `render_text(&SearchResponse) -> String`.
- `src/cli/output.rs:12` matches `SearchResult::Web`.
- `src/cli/output.rs:14` renders web result URLs as `URL: {r.url}`.
- `src/cli/output.rs:22` matches `SearchResult::News`.
- `src/cli/output.rs:24` renders news result URLs as `URL: {r.url}`.
- `src/cli/output.rs:35` matches `SearchResult::Image`.
- `src/cli/output.rs:37` renders image result URLs as `URL: {r.url}`.
- `src/cli/output.rs:39` matches `SearchResult::Video`.
- `src/cli/output.rs:41` renders video result URLs as `URL: {r.url}`.
- `src/cli/output.rs:49` defines `render_fanout_text(&SearchBatchResponse) -> String`.
- `src/cli/output.rs:57` iterates successful provider responses.
- `src/cli/output.rs:59` renders each provider response by calling `render_text(provider_response)`.
- `src/cli/runner.rs:72` handles a successful single-provider search response.
- `src/cli/runner.rs:75` prints `render_text(&response)` to stdout.
- `src/cli/runner.rs:97` obtains a fan-out `SearchBatchResponse`.
- `src/cli/runner.rs:103` prints `render_fanout_text(&response)` to stdout.

## Patterns Found

- Domain URL field pattern: all normalized result structs carry `url: String` at `src/domain/result.rs:39`, `src/domain/result.rs:47`, `src/domain/result.rs:56`, and `src/domain/result.rs:64`.
- Provider DTO URL field pattern: Brave DTOs carry `Option<String>` URL fields at `src/providers/brave/dto.rs:23`, `src/providers/brave/dto.rs:43`, `src/providers/brave/dto.rs:59`, and `src/providers/brave/dto.rs:84`; Exa carries `Option<String>` at `src/providers/exa/dto.rs:63`.
- Mapper defaulting pattern: provider optional URL fields are converted with `unwrap_or_default()` at `src/providers/brave/mapper.rs:21`, `src/providers/brave/mapper.rs:43`, `src/providers/brave/mapper.rs:66`, `src/providers/brave/mapper.rs:88`, `src/providers/exa/mapper.rs:14`, and `src/providers/exa/mapper.rs:25`.
- Text renderer URL-line pattern: CLI output prints an explicit `URL:` line for web, news, image, and video results at `src/cli/output.rs:14`, `src/cli/output.rs:24`, `src/cli/output.rs:37`, and `src/cli/output.rs:41`.
- Fan-out reuse pattern: fan-out output delegates each successful provider response to the same single-provider renderer at `src/cli/output.rs:59`.
- App pass-through pattern: the app layer does not inspect individual `SearchResult` fields in `src/app/search_service.rs:15` or `src/app/fanout_search_service.rs:21`.

## Dependencies

- CLI single-provider path: `src/cli/runner.rs:72` -> `SearchService::search` at `src/app/search_service.rs:13` -> `SearchProvider::search` at `src/domain/provider.rs:23` -> provider mapper -> `SearchResponse` -> `render_text` at `src/cli/output.rs:3`.
- CLI fan-out path: `src/cli/runner.rs:97` -> `FanoutSearchService::search_all` at `src/app/fanout_search_service.rs:14` -> provider `SearchResponse` values in `SearchBatchResponse.responses` -> `render_fanout_text` at `src/cli/output.rs:49` -> `render_text` at `src/cli/output.rs:59`.
- Brave URL path: `src/providers/brave/client.rs:93` through `src/providers/brave/client.rs:108` -> `src/providers/brave/mapper.rs:7`, `src/providers/brave/mapper.rs:30`, `src/providers/brave/mapper.rs:53`, or `src/providers/brave/mapper.rs:75` -> domain result URL fields in `src/domain/result.rs`.
- Exa URL path: `src/providers/exa/client.rs:121` through `src/providers/exa/client.rs:123` -> `src/providers/exa/mapper.rs:10` or `src/providers/exa/mapper.rs:21` -> domain web/news URL fields in `src/domain/result.rs`.

## Tests Found

- `src/cli/output.rs:80` defines `test_render_text_mixed_results`.
- `src/cli/output.rs:86` constructs a web result with `https://rust-lang.org`.
- `src/cli/output.rs:92` constructs a news result with `https://example.com/news`.
- `src/cli/output.rs:99` constructs an image result with `https://example.com/img.png`.
- `src/cli/output.rs:105` constructs a video result with `https://example.com/video`.
- `src/cli/output.rs:120` asserts the rendered text contains `https://rust-lang.org`.
- `src/cli/output.rs:128` defines `test_render_fanout_text_includes_successes_and_failures`.
- `src/cli/output.rs:134` constructs a fan-out web result with `https://rust-lang.org`.
- `src/providers/brave/mapper.rs:108` defines `test_map_web_response`.
- `src/providers/brave/mapper.rs:116` sets a Brave web DTO URL to `https://rust-lang.org`.
- `src/providers/brave/mapper.rs:131` asserts the mapped web result URL is `https://rust-lang.org`.
- `src/providers/brave/mapper.rs:140` defines `test_map_news_response`.
- `src/providers/brave/mapper.rs:148` sets a Brave news DTO URL to `https://example.com/news`.
- `src/providers/brave/mapper.rs:169` defines `test_map_images_response`.
- `src/providers/brave/mapper.rs:176` sets a Brave image DTO URL to `https://example.com/img.png`.
- `src/providers/brave/mapper.rs:199` defines `test_map_videos_response`.
- `src/providers/brave/mapper.rs:207` sets a Brave video DTO URL to `https://example.com/video`.
- `src/providers/brave/client.rs:160` defines `test_brave_provider_web_search`.
- `src/providers/brave/client.rs:165` includes `https://rust-lang.org` in the mocked Brave JSON response.
- `src/providers/brave/client.rs:196` asserts the provider response web result URL is `https://rust-lang.org`.
- `src/providers/exa/dto.rs:77` defines `test_exa_search_response_deserializes_minimal_payload`.
- `src/providers/exa/dto.rs:84` includes `https://example.com` in mocked Exa JSON.
- `src/providers/exa/dto.rs:99` asserts Exa DTO deserialization preserves the URL field.
- `src/providers/exa/mapper.rs:97` defines `sample_result`.
- `src/providers/exa/mapper.rs:100` sets the sample Exa result URL to `https://example.com/news`.
- `src/providers/exa/mapper.rs:110` defines `test_map_news_response_prefers_summary_and_preserves_author`.
- `src/providers/exa/mapper.rs:131` asserts the mapped Exa news result URL is `https://example.com/news`.
- `tests/integration/search_service_test.rs:83` defines `web_result(title, url, snippet)`.
- `tests/integration/search_service_test.rs:86` maps the helper URL argument into `WebResult.url`.
- `tests/integration/search_service_test.rs:131` defines `search_service_preserves_result_count`.
- `tests/integration/search_service_test.rs:136` constructs a web result with `https://example.com/1`.
- `tests/integration/search_service_test.rs:137` constructs a web result with `https://example.com/2`.

## Symbol Index

- `src/domain/result.rs:7` -> `SearchResponse`
- `src/domain/result.rs:16` -> `SearchBatchResponse`
- `src/domain/result.rs:23` -> `ProviderSearchFailure`
- `src/domain/result.rs:29` -> `SearchResult`
- `src/domain/result.rs:37` -> `WebResult`
- `src/domain/result.rs:45` -> `NewsResult`
- `src/domain/result.rs:54` -> `ImageResult`
- `src/domain/result.rs:62` -> `VideoResult`
- `src/domain/provider.rs:18` -> `SearchProvider`
- `src/providers/brave/mapper.rs:7` -> `map_web_response`
- `src/providers/brave/mapper.rs:30` -> `map_news_response`
- `src/providers/brave/mapper.rs:53` -> `map_images_response`
- `src/providers/brave/mapper.rs:75` -> `map_videos_response`
- `src/providers/exa/mapper.rs:10` -> `map_web_response`
- `src/providers/exa/mapper.rs:21` -> `map_news_response`
- `src/cli/output.rs:3` -> `render_text`
- `src/cli/output.rs:49` -> `render_fanout_text`

## Structural Scan Notes

- Ran `/Users/tuna/.codex/skills/research-phase/scripts/structure-map.sh ./ --with-stats`; the script reported 108 Rust files and 118 total code files.
- Ran `/Users/tuna/.codex/skills/research-phase/scripts/ast-scan.sh all src/`; the script listed functions and test helper structs across `src/`, including `src/cli/output.rs`, `src/domain/result.rs`, provider mappers, provider clients, and app services.
- Ran `/Users/tuna/.codex/skills/research-phase/scripts/symbol-index.sh src/`; the script returned no exported functions/classes/types/constants for this Rust tree, so the symbol index above comes from numbered source reads.
- Ran `rg -n "SearchResult|SearchResults|result|results|url|link|render_text|print|stdout|provider all|Document|SearchContent|source|href" src tests docs README.md Cargo.toml` to locate URL/result/output references.

## Observed Current Behavior Surface

- Normalized domain result structs already contain a `url: String` field for every current result variant.
- Brave DTO URL fields are optional and are mapped into domain URL strings for web, news, image, and video results.
- Exa DTO URL fields are optional and are mapped into domain URL strings for web and news results.
- Mapper code converts missing provider URL fields to empty strings.
- Single-provider CLI text output includes a `URL:` line for every current result variant.
- Fan-out CLI text output includes URLs through its call to the single-provider renderer.
- The app layer preserves provider `SearchResponse` values without per-result URL handling.
