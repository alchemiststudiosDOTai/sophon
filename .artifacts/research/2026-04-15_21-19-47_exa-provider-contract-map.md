---
title: "exa provider contract map research findings"
link: "exa-provider-contract-map-research"
type: research
ontological_relations:
  - relates_to: [[search-cli-plan]]
tags: [research, exa, provider, rust, search-cli]
uuid: "8002473b-28c6-40a7-8ac3-3bf9b7c9480c"
created_at: "2026-04-15T21:19:47Z"
---

## Structure
- `src/domain/` contains the provider-agnostic query, result, type, and provider trait definitions.
- `src/transport/` contains the HTTP abstraction and reqwest adapter.
- `src/providers/brave/` contains the only current provider implementation.
- `src/main.rs` constructs `BraveProvider` directly and injects it into `SearchService`.
- `src/cli/args.rs` exposes query, search type, limit, offset, safe search, country, and language flags.
- Exa documentation referenced in this scan:
  - `https://exa.ai/docs/reference/search-api-guide`
  - `https://exa.ai/docs/reference/search`
  - `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`

## Key Files
- `src/domain/types.rs:2` defines `SearchType::{Web, News, Images, Videos}`.
- `src/domain/types.rs:10` defines `SafeSearch::{Off, Moderate, Strict}`.
- `src/domain/types.rs:18` defines `TimeRange::{Day, Week, Month, Year}`.
- `src/domain/query.rs:4` defines `SearchQuery { text, search_type, limit, offset, safe_search, country, language, time_range }`.
- `src/domain/result.rs:5` defines `SearchResponse { query, provider, results, total_estimated, next_page }`.
- `src/domain/result.rs:14` defines `SearchResult::{Web, News, Image, Video}`.
- `src/domain/provider.rs:8` defines `ProviderCapabilities { web, news, images, videos, pagination, safe_search, time_range_filter }`.
- `src/domain/provider.rs:19` defines the `SearchProvider` trait.
- `src/transport/http.rs:5` defines `HttpClient::get_json`.
- `src/transport/http.rs:18` defines `ReqwestHttpClient`.
- `src/providers/mod.rs:1` exports only `brave`.
- `src/providers/brave/config.rs:2` defines `BraveConfig`.
- `src/providers/brave/client.rs:12` defines `BraveProvider<C: HttpClient>`.
- `src/providers/brave/client.rs:24` implements `SearchProvider` for `BraveProvider<C>`.
- `src/providers/brave/mapper.rs:4` maps Brave web DTOs into `SearchResponse`.
- `src/providers/brave/mapper.rs:27` maps Brave news DTOs into `SearchResponse`.
- `src/providers/brave/mapper.rs:50` maps Brave image DTOs into `SearchResponse`.
- `src/providers/brave/mapper.rs:72` maps Brave video DTOs into `SearchResponse`.
- `src/cli/args.rs:43` defines `CliArgs`.
- `src/main.rs:12` imports `BraveProvider`.
- `src/main.rs:13` imports `BraveConfig`.
- `src/main.rs:42` loads Brave config from env.
- `src/main.rs:50` constructs `ReqwestHttpClient`.
- `src/main.rs:51` constructs `BraveProvider`.
- `src/main.rs:52` injects the provider into `SearchService`.

## Patterns Found
- Shared provider contract:
  - `src/domain/provider.rs:19`
  - `src/app/search_service.rs:7`
  - `src/app/search_service.rs:11`
  - `src/providers/brave/client.rs:24`
- Query-to-provider request translation:
  - `src/providers/brave/client.rs:41`
  - `src/providers/brave/client.rs:50`
  - `src/providers/brave/client.rs:57`
  - `src/providers/brave/client.rs:65`
  - `src/providers/brave/client.rs:68`
  - `src/providers/brave/client.rs:71`
- Provider DTO-to-domain result mapping:
  - `src/providers/brave/mapper.rs:4`
  - `src/providers/brave/mapper.rs:27`
  - `src/providers/brave/mapper.rs:50`
  - `src/providers/brave/mapper.rs:72`
- Provider selection in the binary:
  - `src/main.rs:42`
  - `src/main.rs:51`
  - `src/main.rs:52`

## Dependencies
- `src/main.rs` imports:
  - `src/app/search_service.rs`
  - `src/cli/args.rs`
  - `src/cli/output.rs`
  - `src/domain/query.rs`
  - `src/providers/brave/client.rs`
  - `src/providers/brave/config.rs`
  - `src/transport/http.rs`
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

## Current Shared Contract Surface
- `SearchQuery.search_type` uses the shared enum in `src/domain/types.rs:2`.
- `SearchQuery.limit` and `SearchQuery.offset` are optional numeric fields in `src/domain/query.rs:7-8`.
- `SearchQuery.safe_search`, `country`, `language`, and `time_range` are optional fields in `src/domain/query.rs:9-12`.
- `SearchResponse.results` is a vector of `SearchResult` variants in `src/domain/result.rs:8` and `src/domain/result.rs:14-18`.
- `SearchResponse.total_estimated` is optional in `src/domain/result.rs:9`.
- `SearchResponse.next_page` is optional in `src/domain/result.rs:10`.

## Exa Request Fields Observed
- Exa search endpoint: `POST https://api.exa.ai/search`.
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Exa authentication header: `x-api-key`.
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Request parameters listed in the Exa coding-agent reference:
  - `query`
  - `type`
  - `stream`
  - `numResults`
  - `category`
  - `userLocation`
  - `includeDomains`
  - `excludeDomains`
  - `startPublishedDate`
  - `endPublishedDate`
  - `startCrawlDate`
  - `endCrawlDate`
  - `moderation`
  - `additionalQueries`
  - `systemPrompt`
  - `outputSchema`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Exa contents parameters listed in the Exa coding-agent reference:
  - `contents.text`
  - `contents.highlights`
  - `contents.summary`
  - `contents.livecrawlTimeout`
  - `contents.maxAgeHours`
  - `contents.subpages`
  - `contents.subpageTarget`
  - `contents.extras.links`
  - `contents.extras.imageLinks`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Exa search types listed in the Exa coding-agent reference:
  - `auto`
  - `fast`
  - `instant`
  - `deep-lite`
  - `deep`
  - `deep-reasoning`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Exa categories listed in the Exa coding-agent reference:
  - `company`
  - `people`
  - `research paper`
  - `news`
  - `personal site`
  - `financial report`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`

## Exa Response Fields Observed
- Exa response top-level fields listed in the Exa coding-agent reference:
  - `requestId`
  - `searchType`
  - `results`
  - `output`
  - `costDollars`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`
- Exa result object fields listed in the Exa coding-agent reference:
  - `title`
  - `url`
  - `id`
  - `publishedDate`
  - `author`
  - `image`
  - `favicon`
  - `text`
  - `highlights`
  - `highlightScores`
  - `summary`
  - `subpages`
  - `extras.links`
  - Source: `https://exa.ai/docs/reference/search-api-guide-for-coding-agents`

## Repo-to-Exa Field Presence Map
- `src/domain/query.rs:5` defines `text`; Exa request docs list `query`.
- `src/domain/query.rs:6` defines `search_type`; Exa request docs list `type` and `category`.
- `src/domain/query.rs:7` defines `limit`; Exa request docs list `numResults`.
- `src/domain/query.rs:8` defines `offset`; no Exa request field named `offset` appears in the referenced Exa docs.
- `src/domain/query.rs:9` defines `safe_search`; Exa request docs list `moderation`.
- `src/domain/query.rs:10` defines `country`; Exa request docs list `userLocation`.
- `src/domain/query.rs:11` defines `language`; no Exa request field named `language` appears in the referenced Exa docs.
- `src/domain/query.rs:12` defines `time_range`; Exa request docs list `startPublishedDate`, `endPublishedDate`, `startCrawlDate`, and `endCrawlDate`.
- `src/domain/result.rs:22-27` defines `WebResult`.
- `src/domain/result.rs:30-36` defines `NewsResult`.
- `src/domain/result.rs:39-44` defines `ImageResult`.
- `src/domain/result.rs:47-53` defines `VideoResult`.
- Exa result docs list `title`, `url`, `publishedDate`, `author`, `image`, `favicon`, `text`, `highlights`, `summary`, `subpages`, and `extras.links`.

## Current Transport Surface Compared To Exa Docs
- `src/transport/http.rs:6-10` defines `HttpClient::get_json` with `url`, `headers`, and query parameters.
- `src/transport/http.rs:41` builds a reqwest `GET` request.
- Exa docs referenced in this scan describe `POST /search` with a JSON request body.

## Existing Provider Composition Surface
- `src/providers/mod.rs:1` exposes only the `brave` module.
- `src/main.rs:12-13` imports Brave-specific provider types.
- `src/main.rs:42-52` constructs Brave-specific runtime configuration and provider instances.
- `src/cli/args.rs:50-66` defines CLI flags for `search_type`, `limit`, `offset`, `safe_search`, `country`, and `language`.
