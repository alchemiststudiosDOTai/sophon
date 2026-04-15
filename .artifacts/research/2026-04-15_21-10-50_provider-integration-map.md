---
title: "provider integration map research findings"
link: "provider-integration-map-research"
type: research
ontological_relations:
  - relates_to: [[search-cli-plan]]
tags: [research, provider, rust, search-cli]
uuid: "d328abd9-9e8f-40a1-af87-65be2ac78dd3"
created_at: "2026-04-15T21:10:50Z"
---

## Structure
- `src/domain/` contains provider-agnostic query, result, error, and trait types.
- `src/transport/` contains the HTTP abstraction (`HttpClient`) and the reqwest adapter.
- `src/providers/brave/` contains the Brave adapter split into config, DTO, mapper, and client modules.
- `src/app/` contains `SearchService`, which stores `Box<dyn SearchProvider>`.
- `src/cli/` contains CLI argument parsing and text rendering.
- `src/main.rs` composes the concrete provider and converts CLI args into a `SearchQuery`.

## Key Files
- `src/domain/provider.rs:6` defines `ProviderCapabilities`.
- `src/domain/provider.rs:19` defines the `SearchProvider` trait with `id`, `capabilities`, and `search`.
- `src/domain/query.rs:4` defines `SearchQuery`.
- `src/domain/result.rs:5` defines `SearchResponse` with the provider identifier recorded as `String`.
- `src/domain/types.rs:2` defines supported `SearchType` variants: `Web`, `News`, `Images`, `Videos`.
- `src/transport/http.rs:5` defines the `HttpClient` trait used by provider clients.
- `src/transport/http.rs:18` defines `ReqwestHttpClient`.
- `src/providers/mod.rs:1` exports only the `brave` provider module.
- `src/providers/brave/config.rs:2` defines `BraveConfig`.
- `src/providers/brave/config.rs:8` loads configuration from `BRAVE_API_KEY`.
- `src/providers/brave/dto.rs:4` defines `BraveWebResponse`.
- `src/providers/brave/dto.rs:30` defines `BraveNewsResponse`.
- `src/providers/brave/dto.rs:51` defines `BraveImagesResponse`.
- `src/providers/brave/dto.rs:71` defines `BraveVideosResponse`.
- `src/providers/brave/mapper.rs:4` maps `BraveWebResponse` into `SearchResponse`.
- `src/providers/brave/mapper.rs:27` maps `BraveNewsResponse` into `SearchResponse`.
- `src/providers/brave/mapper.rs:50` maps `BraveImagesResponse` into `SearchResponse`.
- `src/providers/brave/mapper.rs:72` maps `BraveVideosResponse` into `SearchResponse`.
- `src/providers/brave/client.rs:12` defines `BraveProvider<C: HttpClient>`.
- `src/providers/brave/client.rs:24` implements `SearchProvider` for `BraveProvider<C>`.
- `src/app/search_service.rs:6` defines `SearchService`.
- `src/app/search_service.rs:11` constructs `SearchService` with `Box<dyn SearchProvider>`.
- `src/main.rs:12` imports `BraveProvider`.
- `src/main.rs:13` imports `BraveConfig`.
- `src/main.rs:42` loads Brave config.
- `src/main.rs:51` constructs the concrete `BraveProvider`.
- `src/main.rs:52` injects the provider into `SearchService`.
- `src/cli/args.rs:40` defines `CliArgs`.
- `tests/architecture_test.rs:25` enforces that provider modules do not import `cli` or `app`.

## Patterns Found
- Provider abstraction:
  - `src/domain/provider.rs:19`
  - `src/app/search_service.rs:7`
  - `src/app/search_service.rs:11`
  - `src/app/search_service.rs:32`
  - `src/providers/brave/client.rs:24`
- Provider-specific DTO to domain mapping:
  - `src/providers/brave/mapper.rs:4`
  - `src/providers/brave/mapper.rs:27`
  - `src/providers/brave/mapper.rs:50`
  - `src/providers/brave/mapper.rs:72`
- Provider-specific runtime composition in the binary:
  - `src/main.rs:12`
  - `src/main.rs:13`
  - `src/main.rs:42`
  - `src/main.rs:51`
  - `src/main.rs:52`
- Provider identifier stored in normalized output:
  - `src/domain/result.rs:7`
  - `src/providers/brave/mapper.rs:10`
  - `src/providers/brave/mapper.rs:32`
  - `src/providers/brave/mapper.rs:55`
  - `src/providers/brave/mapper.rs:77`
  - `src/cli/output.rs:5`

## Dependencies
- `src/main.rs` imports:
  - `src/app/search_service.rs`
  - `src/cli/args.rs`
  - `src/cli/output.rs`
  - `src/domain/query.rs`
  - `src/providers/brave/client.rs`
  - `src/providers/brave/config.rs`
  - `src/transport/http.rs`
- `src/app/search_service.rs` imports:
  - `src/domain/error.rs`
  - `src/domain/provider.rs`
  - `src/domain/query.rs`
  - `src/domain/result.rs`
- `src/providers/brave/client.rs` imports:
  - `src/domain/error.rs`
  - `src/domain/provider.rs`
  - `src/domain/query.rs`
  - `src/domain/result.rs`
  - `src/domain/types.rs`
  - `src/providers/brave/config.rs`
  - `src/providers/brave/dto.rs`
  - `src/providers/brave/mapper.rs`
  - `src/transport/http.rs`
- `src/providers/brave/mapper.rs` imports:
  - `src/domain/result.rs`
  - `src/providers/brave/dto.rs`
- `src/transport/http.rs` imports:
  - `src/domain/error.rs`

## Provider Addition Surface
- New provider modules belong under `src/providers/`, based on the existing `src/providers/brave/` layout and `src/providers/mod.rs:1`.
- The required trait implementation boundary is `src/domain/provider.rs:19`.
- The application layer already accepts any boxed provider at `src/app/search_service.rs:7` and `src/app/search_service.rs:11`.
- The current binary selects Brave directly in `src/main.rs:42`, `src/main.rs:51`, and `src/main.rs:52`.
- The current CLI arguments do not include a provider selector; `src/cli/args.rs:43-66` contains query, about flag, search type, limit, offset, safe search, country, and language.

## Tests And Enforcement
- `src/providers/brave/mapper.rs:95` contains mapper unit tests for all four search types.
- `src/providers/brave/client.rs:110` contains a provider test using a mocked `HttpClient`.
- `src/app/search_service.rs:20` contains a service test using a mocked `SearchProvider`.
- `tests/architecture_test.rs:25` forbids provider imports from `crate::cli::` and `crate::app::`.
- `justfile:2` defines `just check`, which runs format, clippy, tests, and mdBook build.

## Operator Artifacts
- `docs/architecture.md:151` documents provider addition as creating a new provider module, implementing `SearchProvider`, and mapping DTOs into `SearchResponse`.
- `docs/architecture.md:155` states that no changes to `domain`, `app`, or `cli` are required for a new provider.
- `src/main.rs:12-13` and `src/main.rs:42-52` show Brave-specific composition in the binary.
- `PRD.md` was not present in the repository root during this scan.
