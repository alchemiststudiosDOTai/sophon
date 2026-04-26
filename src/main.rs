mod app;
mod bootstrap;
mod cli;
mod domain;
mod providers;
mod transport;

use bootstrap::provider_registry::{ProviderId, ProviderRegistry};
use clap::Parser;
use cli::args::{CliArgs, CliProvider};
use cli::output::render_text;
use domain::query::SearchQuery;

impl From<CliProvider> for ProviderId {
    fn from(provider: CliProvider) -> Self {
        match provider {
            CliProvider::Brave => ProviderId::Brave,
            CliProvider::Exa => ProviderId::Exa,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args = CliArgs::parse();

    if args.about {
        println!("sophon-cli — a provider-agnostic search CLI");
        println!();
        println!("Named after the Sophon from Cixin Liu's Three-Body Problem trilogy:");
        println!("a sentient proton supercomputer that performs near-infinite computation");
        println!("across vast distances. This tiny CLI delegates its heavy lifting to");
        println!("distant search APIs the same way.");
        println!();
        println!("Currently supports Brave Search (web, news, images, video) and Exa.");
        return;
    }

    let query_text = match args.query {
        Some(q) => q,
        None => {
            eprintln!("Error: missing query. Use --help for usage or --about for more info.");
            std::process::exit(1);
        }
    };

    let query = SearchQuery {
        text: query_text,
        search_type: args.search_type.into(),
        limit: args.limit,
        offset: args.offset,
        safe_search: args.safe_search.map(|s| s.into()),
        country: args.country,
        language: args.language,
        time_range: None,
    };

    let provider_id = ProviderId::from(args.provider);
    let registry = ProviderRegistry::production_from_env();
    let service = registry.build(provider_id).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });

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
