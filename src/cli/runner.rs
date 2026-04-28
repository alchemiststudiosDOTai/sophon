use clap::Parser;

use crate::bootstrap::provider_registry::{ProviderId, ProviderRegistry};
use crate::cli::args::{CliArgs, CliProvider};
use crate::cli::output::{render_fanout_text, render_text};
use crate::cli::request::build_search_query;
use crate::domain::SearchQuery;

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

    let query = build_search_query(query_text, &args);
    let registry = ProviderRegistry::production_from_env();

    match args.provider {
        CliProvider::Brave => run_single_provider(&registry, ProviderId::Brave, query).await,
        CliProvider::Exa => run_single_provider(&registry, ProviderId::Exa, query).await,
        CliProvider::All => run_all_enabled(&registry, query).await,
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

async fn run_single_provider(
    registry: &ProviderRegistry,
    provider_id: ProviderId,
    query: SearchQuery,
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
            0
        }
        Err(error) => {
            tracing::error!(error = %error, "search failed");
            eprintln!("Search failed: {}", error);
            1
        }
    }
}

async fn run_all_enabled(registry: &ProviderRegistry, query: SearchQuery) -> i32 {
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
    if response.responses.is_empty() { 1 } else { 0 }
}
