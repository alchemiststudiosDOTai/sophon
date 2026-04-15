---
title: "Exa provider implementation plan"
link: "exa-provider-implementation-plan"
type: implementation_plan
ontological_relations:
  - relates_to: [[exa-provider-contract-map-research]]
  - relates_to: [[provider-integration-map-research]]
tags: [plan, exa, provider, rust, coding]
uuid: "ac443d51-2b38-49bd-813b-debcb545248f"
created_at: "2026-04-15T21:23:23Z"
parent_research: ".artifacts/research/2026-04-15_21-19-47_exa-provider-contract-map.md"
git_commit_at_plan: "78610af"
---

## Goal

Add Exa as a selectable search provider in the Rust CLI while keeping the current domain and application contracts unchanged.

**Out of scope**: replacing Brave as the default provider, changing `SearchQuery` or `SearchResponse`, adding Exa-specific deep-search/output features, adding Exa image/video support, packaging or deployment work.

## Scope & Assumptions

**IN scope**:
- Extend the shared HTTP transport so providers can issue JSON `POST` requests.
- Add an `src/providers/exa/` adapter with config, DTOs, mapper, and provider client.
- Preserve the existing domain layer and `SearchService` trait boundary.
- Add CLI-level provider selection so Exa is reachable at runtime.
- Add focused regression tests for the new provider and the widened transport trait.

**OUT of scope**:
- Any edits to `src/domain/` types or provider trait shape.
- Exa support for `SearchType::Images` or `SearchType::Videos`.
- Silent fallback behavior for unsupported Exa query features.
- New output formats or rich structured rendering.
- User-facing docs beyond minimal developer architecture notes.

**Assumptions**:
- `EXA_API_KEY` will be supplied in the environment the same way `BRAVE_API_KEY` is today.
- Exa integration will use raw `reqwest` through the existing `HttpClient` abstraction rather than adding an Exa SDK dependency.
- Exa `moderation` is a boolean, so both `SafeSearch::Moderate` and `SafeSearch::Strict` will map to `true`.
- Exa does not expose request fields matching the current `offset` and `language` domain inputs in the referenced docs, so the adapter will reject those when `--provider exa` is selected instead of ignoring them.

## Deliverables

- `src/transport/http.rs` updated with shared JSON `POST` support.
- `src/providers/{mod.rs,exa/{mod.rs,config.rs,dto.rs,mapper.rs,client.rs}}`
- `src/cli/args.rs` updated with provider selection.
- `src/main.rs` updated to instantiate either Brave or Exa.
- `Cargo.toml` updated only if an ISO-8601 time helper crate is needed for Exa time-range translation.
- Targeted unit tests in the touched provider and transport modules.
- `docs/architecture.md` refreshed where the runtime provider-selection story changes.

## Readiness

- Repo is already bootstrapped and compiles before this work begins.
- Research artifacts exist at:
  - `.artifacts/research/2026-04-15_21-10-50_provider-integration-map.md`
  - `.artifacts/research/2026-04-15_21-19-47_exa-provider-contract-map.md`
- Current git plan baseline:
  - commit: `78610af`
  - working tree includes local doc/artifact changes (`AGENTS.md`, removed `PRD.md`, new `.artifacts/research/`)
- Execution requires environment access to both `BRAVE_API_KEY` and `EXA_API_KEY` for live manual smoke checks after implementation.

## Milestones

- **M1**: Shared transport and Exa scaffolding compile.
- **M2**: Exa request/response translation works for web and news searches.
- **M3**: CLI and binary composition can select Brave or Exa without touching domain/app layers.
- **M4**: Focused regression tests and architecture notes match the new provider surface.

## Ticket Index

<!-- TICKET_INDEX:START -->

| Task | Title | Ticket |
|---|---|---|
| T001 | Extend shared HTTP transport for JSON POST requests | [tickets/T001.md](tickets/T001.md) |
| T002 | Add Exa provider scaffolding, config, and serde DTOs | [tickets/T002.md](tickets/T002.md) |
| T003 | Map Exa responses into the existing domain result model | [tickets/T003.md](tickets/T003.md) |
| T004 | Implement the Exa provider client and query translation | [tickets/T004.md](tickets/T004.md) |
| T005 | Wire provider selection into the CLI and binary composition | [tickets/T005.md](tickets/T005.md) |
| T006 | Refresh regression coverage and architecture notes for multi-provider runtime | [tickets/T006.md](tickets/T006.md) |

<!-- TICKET_INDEX:END -->

## Work Breakdown (Tasks)

### T001: Extend shared HTTP transport for JSON POST requests

**Summary**: Add JSON `POST` support to the existing `HttpClient` trait and reqwest adapter so provider implementations can call Exa without bypassing the transport layer.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: <none>

**Target milestone**: M1

**Acceptance test**: `cargo test transport::http::tests::test_post_json_decodes_success_response`

**Files/modules touched**:
- `src/transport/http.rs`

**Steps**:
1. Add a new async trait method to `HttpClient`, alongside `get_json`, for posting a JSON body and decoding the JSON response.
2. Keep the trait generic over the response type and request body type so provider adapters can reuse it without casting through `serde_json::Value`.
3. Refactor `ReqwestHttpClient` so GET and POST share a single private response-status handling path for auth, rate limit, non-2xx provider errors, and JSON decode errors.
4. Implement the new `post_json` method with reqwest `.post(url).json(&body)`, preserving caller-supplied headers.
5. Add a focused test in `src/transport/http.rs` that spins up a local `tokio::net::TcpListener`, returns a fixed JSON body, and verifies the new POST method decodes it successfully.

### T002: Add Exa provider scaffolding, config, and serde DTOs

**Summary**: Create the Exa provider module layout and define the request/response structs needed for the subset of the Exa API this CLI will consume.

**Owner**: backend

**Estimate**: 1.5h

**Dependencies**: T001

**Target milestone**: M1

**Acceptance test**: `cargo test providers::exa::dto::tests::test_exa_search_response_deserializes_minimal_payload`

**Files/modules touched**:
- `src/providers/mod.rs`
- `src/providers/exa/mod.rs`
- `src/providers/exa/config.rs`
- `src/providers/exa/dto.rs`

**Steps**:
1. Export `pub mod exa;` from `src/providers/mod.rs` and create `src/providers/exa/mod.rs` with `pub mod client;`, `pub mod config;`, `pub mod dto;`, and `pub mod mapper;`.
2. Add `ExaConfig` in `src/providers/exa/config.rs` with `api_key` and `base_url`, and implement `from_env()` to read `EXA_API_KEY` and default `base_url` to `https://api.exa.ai`.
3. In `src/providers/exa/dto.rs`, define the request structs needed by the adapter using `#[serde(rename_all = "camelCase")]`:
   - `ExaSearchRequest`
   - `ExaContentsRequest`
4. In the same file, define only the response structs/fields the adapter will map:
   - `ExaSearchResponse`
   - `ExaResult`
   - any small nested types required for fields consumed by the mapper
5. Keep unknown Exa fields ignored by omission rather than attempting to model the full API surface.
6. Add a DTO unit test that deserializes a minimal Exa search payload containing one result and asserts `request_id`, `search_type`, `published_date`, and `summary` are populated correctly.

### T003: Map Exa responses into the existing domain result model

**Summary**: Implement mapper functions that convert Exa search results into the current `SearchResponse` and `SearchResult` variants without changing shared domain types.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: T002

**Target milestone**: M2

**Acceptance test**: `cargo test providers::exa::mapper::tests::test_map_news_response_prefers_summary_and_preserves_author`

**Files/modules touched**:
- `src/providers/exa/mapper.rs`

**Steps**:
1. Add `map_web_response` and `map_news_response` in `src/providers/exa/mapper.rs`.
2. Set `SearchResponse.provider` to `"exa"` and `SearchResponse.query` from the original query string passed into the mapper, because the referenced Exa response contract does not expose the original query text.
3. For web results, map Exa `title`, `url`, and `summary`/`text` into `WebResult`, preferring `summary` and falling back to `text`.
4. For news results, map Exa `title`, `url`, `summary`/`text`, `author`, and `publishedDate` into `NewsResult`, using `author` as `source`.
5. Set `total_estimated` and `next_page` to `None`, matching the current normalized model when the provider does not expose those values in the referenced contract.
6. Add a mapper test that constructs a minimal Exa news DTO and proves the mapper uses `summary` before `text` and carries `author` into `source`.

### T004: Implement the Exa provider client and query translation

**Summary**: Add `ExaProvider<C>` that translates the existing `SearchQuery` into Exa request bodies, advertises accurate capabilities, and fails explicitly on unsupported inputs.

**Owner**: backend

**Estimate**: 2h

**Dependencies**: T001, T002, T003

**Target milestone**: M2

**Acceptance test**: `cargo test providers::exa::client::tests::test_exa_provider_news_search_posts_expected_payload`

**Files/modules touched**:
- `Cargo.toml`
- `src/providers/exa/client.rs`

**Steps**:
1. Add a narrow time-handling dependency in `Cargo.toml` only if needed to produce ISO-8601 UTC timestamps for Exa `startPublishedDate` and `endPublishedDate`.
2. Implement `ExaProvider<C: HttpClient>` with `new(client, config)` and a `SearchProvider` implementation whose `id()` returns `"exa"`.
3. Set Exa capabilities to:
   - `web = true`
   - `news = true`
   - `images = false`
   - `videos = false`
   - `pagination = false`
   - `safe_search = true`
   - `time_range_filter = true`
4. Translate the shared `SearchQuery` into Exa request fields as follows:
   - `text` -> `query`
   - `limit` -> `numResults`
   - `search_type = Web` -> omit `category`
   - `search_type = News` -> `category = "news"`
   - `safe_search = Off` -> `moderation = false`
   - `safe_search = Moderate | Strict` -> `moderation = true`
   - `country` -> `userLocation`
   - `time_range` -> `startPublishedDate` and `endPublishedDate` in ISO-8601 UTC
   - request `contents.text = true`
   - set request `type = "auto"`
5. Reject unsupported Exa query inputs with `SearchError::InvalidQuery` rather than silently ignoring them:
   - `SearchType::Images`
   - `SearchType::Videos`
   - any non-`None` `offset`
   - any non-`None` `language`
6. Send `POST {base_url}/search` with the `x-api-key` header through the new transport method and dispatch the decoded response through the Exa mapper selected by the original `SearchType`.
7. Add a mocked-provider test that asserts the request body contains `category: "news"`, `numResults`, `moderation`, and published-date window fields, then verifies the mapped response comes back with provider `"exa"`.

### T005: Wire provider selection into the CLI and binary composition

**Summary**: Make the provider selectable at runtime while preserving Brave as the default and leaving the domain/application layers untouched.

**Owner**: backend

**Estimate**: 1.5h

**Dependencies**: T004

**Target milestone**: M3

**Acceptance test**: `cargo test cli::args::tests::test_cli_provider_parses_exa_and_defaults_to_brave`

**Files/modules touched**:
- `src/cli/args.rs`
- `src/main.rs`

**Steps**:
1. Add a `CliProvider` `ValueEnum` to `src/cli/args.rs` with `Brave` and `Exa`.
2. Add a `--provider` flag (short `-p`) to `CliArgs` with default value `brave`.
3. Keep the existing `SearchQuery` construction unchanged so the provider adapter remains responsible for validating unsupported inputs.
4. Update `src/main.rs` to import `ExaProvider` and `ExaConfig`, then match on `args.provider` to build the concrete provider instance:
   - Brave branch loads `BraveConfig` and constructs `BraveProvider`
   - Exa branch loads `ExaConfig` and constructs `ExaProvider`
5. Keep Brave as the default branch so existing invocations still behave the same when `--provider` is omitted.
6. Add a CLI parsing unit test proving `--provider exa` parses and that the default remains `Brave`.

### T006: Refresh regression coverage and architecture notes for multi-provider runtime

**Summary**: Update affected tests and architecture notes so the widened transport trait and multi-provider binary wiring are documented and enforced by focused regressions.

**Owner**: backend

**Estimate**: 1h

**Dependencies**: T001, T004, T005

**Target milestone**: M4

**Acceptance test**: `cargo test providers::exa::client::tests::test_exa_provider_rejects_unsupported_query_fields`

**Files/modules touched**:
- `src/providers/brave/client.rs`
- `src/providers/exa/client.rs`
- `docs/architecture.md`

**Steps**:
1. Update the existing Brave provider test mock in `src/providers/brave/client.rs` so it implements the expanded `HttpClient` trait without changing Brave behavior.
2. Add a focused Exa provider regression test that proves unsupported query fields (`Images`, `Videos`, `offset`, or `language`) return `SearchError::InvalidQuery`.
3. Refresh `docs/architecture.md` so the provider-addition section distinguishes:
   - unchanged domain/app boundaries
   - provider-specific transport needs
   - current binary wiring via CLI provider selection
4. Leave `tests/architecture_test.rs` unchanged unless the implementation introduces a real boundary violation; the current boundary rules should still hold.

## Risks & Mitigations

- Exa request contract drift: mitigate by implementing only the fields cited in the current Exa docs and keeping DTOs narrow.
- Safe-search fidelity loss: Exa exposes boolean moderation only, so `Moderate` and `Strict` collapse to the same provider value; document this in code comments near the mapping.
- Unsupported query-field confusion: reject unsupported Exa inputs explicitly instead of silently dropping them.
- Time-range serialization complexity: contain any new date/time dependency to the Exa client request translation path.
- Architecture doc drift: refresh the provider-selection section in `docs/architecture.md` as part of the same change set.

## Test Strategy

- Add one focused test per task only, centered on the newly introduced behavior.
- Prefer unit tests with mock transport over live API calls.
- Keep existing Brave tests compiling after the `HttpClient` trait expands.
- Defer live `cargo run -- --provider exa ...` smoke checks to execution/verification, not plan work.

## References

- `.artifacts/research/2026-04-15_21-19-47_exa-provider-contract-map.md`
- `.artifacts/research/2026-04-15_21-10-50_provider-integration-map.md`
- `src/domain/provider.rs:8`
- `src/domain/provider.rs:19`
- `src/transport/http.rs:5`
- `src/transport/http.rs:31`
- `src/providers/brave/client.rs:24`
- `src/providers/brave/client.rs:41`
- `src/main.rs:12`
- `src/main.rs:42`
- `src/main.rs:51`
- `src/cli/args.rs:36`
- `docs/architecture.md:151`
- `https://exa.ai/docs/reference/search`
- `https://exa.ai/docs/reference/search-api-guide`
- `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`

## Final Gate

- **Output summary**: plan dir path `.artifacts/plan/2026-04-15_16-22-21_exa-provider/`, milestone count `4`, ticket count `6`
- **Next step**: review this plan, then proceed to `grill-me` or `execute-phase` using `.artifacts/plan/2026-04-15_16-22-21_exa-provider/PLAN.md`
