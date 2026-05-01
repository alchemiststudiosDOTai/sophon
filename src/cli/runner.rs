use clap::Parser;

use crate::bootstrap::provider_registry::{ProviderId, ProviderRegistry};
use crate::cli::args::{CliArgs, CliProvider};
use crate::cli::db::SearchDbWriter;
use crate::cli::output::{render_fanout_text, render_text};
use crate::cli::request::build_search_query;
use crate::cli::scrape::{scrape_batch_urls, scrape_result_urls};
use crate::domain::{SearchBatchResponse, SearchQuery, SearchResponse, SearchResult};

pub async fn run_from_env() -> i32 {
    let args = CliArgs::parse();
    run(args).await
}

pub async fn run(args: CliArgs) -> i32 {
    if args.about {
        print_about();
        return 0;
    }

    let query_text = match args.query.clone() {
        Some(query) => query,
        None => {
            eprintln!("Error: missing query. Use --help for usage or --about for more info.");
            return 1;
        }
    };

    let db_path = args.db.clone();
    let scrape_enabled = args.scrape;
    let scrape_limit = args.scrape_limit;

    let query = build_search_query(query_text, &args);
    let registry = ProviderRegistry::production_from_env();

    match args.provider {
        CliProvider::Brave => {
            run_single_provider(
                &registry,
                ProviderId::Brave,
                query,
                db_path,
                scrape_enabled,
                scrape_limit,
            )
            .await
        }
        CliProvider::Exa => {
            run_single_provider(
                &registry,
                ProviderId::Exa,
                query,
                db_path,
                scrape_enabled,
                scrape_limit,
            )
            .await
        }
        CliProvider::All => {
            run_all_enabled(&registry, query, db_path, scrape_enabled, scrape_limit).await
        }
    }
}

fn print_about() {
    println!("sophon-cli — a provider-agnostic search CLI");
    println!();
    println!("Named after the Sophon from Cixin Liu's Three-Body Problem trilogy:");
    println!("a sentient proton supercomputer that performs near-infinite computation");
    println!("across vast distances. This tiny CLI delegates its heavy lifting to");
    println!("distant search APIs the same way.");
    println!();
    println!("Currently supports Brave Search (web, news, images, video) and Exa.");
}

fn first_result_url(response: &SearchResponse) -> String {
    response
        .results
        .first()
        .map(|r| match r {
            SearchResult::Web(w) => w.url.clone(),
            SearchResult::News(n) => n.url.clone(),
            SearchResult::Image(i) => i.url.clone(),
            SearchResult::Video(v) => v.url.clone(),
        })
        .unwrap_or_else(|| "(no result urls)".to_string())
}

fn first_url_in_batch(batch: &SearchBatchResponse) -> String {
    for r in &batch.responses {
        if let Some(u) = r.results.first().map(|res| match res {
            SearchResult::Web(w) => w.url.clone(),
            SearchResult::News(n) => n.url.clone(),
            SearchResult::Image(i) => i.url.clone(),
            SearchResult::Video(v) => v.url.clone(),
        }) {
            return u;
        }
    }
    "(no result urls)".to_string()
}

async fn persist_and_optional_scrape_single(
    db_path: &std::path::Path,
    response: &SearchResponse,
    scrape: bool,
    page_limit: usize,
) -> Result<(), String> {
    if scrape {
        let client = reqwest::Client::new();
        let (pages, duration_ms, fatal) = scrape_result_urls(&client, response, page_limit).await;
        let seed_url = first_result_url(response);
        let response_clone = response.clone();
        let path = db_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut writer = SearchDbWriter::open(&path).map_err(|e| e.to_string())?;
            writer
                .persist_response_with_scrape(
                    &response_clone,
                    &seed_url,
                    duration_ms,
                    page_limit,
                    fatal.as_deref(),
                    &pages,
                )
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())??;
    } else {
        let response_clone = response.clone();
        let path = db_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut writer = SearchDbWriter::open(&path).map_err(|e| e.to_string())?;
            writer
                .persist_response(&response_clone)
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())??;
    }
    Ok(())
}

async fn persist_and_optional_scrape_batch(
    db_path: &std::path::Path,
    batch: &SearchBatchResponse,
    scrape: bool,
    page_limit: usize,
) -> Result<(), String> {
    if scrape && !batch.responses.is_empty() {
        let client = reqwest::Client::new();
        let (pages, duration_ms, fatal) = scrape_batch_urls(&client, batch, page_limit).await;
        let seed_url = first_url_in_batch(batch);
        let batch_clone = batch.clone();
        let path = db_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut writer = SearchDbWriter::open(&path).map_err(|e| e.to_string())?;
            writer
                .persist_batch_responses_with_scrape(
                    &batch_clone,
                    &seed_url,
                    duration_ms,
                    page_limit,
                    fatal.as_deref(),
                    &pages,
                )
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())??;
    } else {
        let batch_clone = batch.clone();
        let path = db_path.to_path_buf();
        tokio::task::spawn_blocking(move || {
            let mut writer = SearchDbWriter::open(&path).map_err(|e| e.to_string())?;
            writer
                .persist_batch_responses(&batch_clone)
                .map_err(|e| e.to_string())
        })
        .await
        .map_err(|e| e.to_string())??;
    }
    Ok(())
}

async fn run_single_provider(
    registry: &ProviderRegistry,
    provider_id: ProviderId,
    query: SearchQuery,
    db_path: Option<std::path::PathBuf>,
    scrape: bool,
    scrape_limit: usize,
) -> i32 {
    tracing::info!(provider = %provider_id, query = %query.text, "initializing search service");

    let service = match registry.build(provider_id) {
        Ok(service) => service,
        Err(error) => {
            tracing::error!(%error, "failed to build provider");
            eprintln!("{error}");
            return 1;
        }
    };

    match service.search(query).await {
        Ok(response) => {
            tracing::info!(result_count = response.results.len(), total_estimated = ?response.total_estimated, "search completed");
            println!("{}", render_text(&response));

            if let Some(ref path) = db_path
                && let Err(e) =
                    persist_and_optional_scrape_single(path, &response, scrape, scrape_limit).await
            {
                tracing::error!(%e, "database persist failed");
                eprintln!("Database error: {e}");
                return 1;
            }
            0
        }
        Err(error) => {
            tracing::error!(error = %error, "search failed");
            eprintln!("Search failed: {}", error);
            1
        }
    }
}

async fn run_all_enabled(
    registry: &ProviderRegistry,
    query: SearchQuery,
    db_path: Option<std::path::PathBuf>,
    scrape: bool,
    scrape_limit: usize,
) -> i32 {
    tracing::info!(query = %query.text, "initializing all-enabled provider fan-out service");
    let service = match registry.build_all_enabled() {
        Ok(service) => service,
        Err(error) => {
            tracing::error!(%error, "failed to build fan-out providers");
            eprintln!("{error}");
            return 1;
        }
    };

    let response = service.search_all(query).await;
    tracing::info!(
        successful_providers = response.responses.len(),
        failed_providers = response.failures.len(),
        "fan-out search completed"
    );
    println!("{}", render_fanout_text(&response));

    let exit = if response.responses.is_empty() { 1 } else { 0 };

    if let Some(ref path) = db_path
        && exit == 0
        && let Err(e) =
            persist_and_optional_scrape_batch(path, &response, scrape, scrape_limit).await
    {
        tracing::error!(%e, "database persist failed");
        eprintln!("Database error: {e}");
        return 1;
    }

    exit
}
