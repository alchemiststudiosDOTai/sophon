mod app;
mod cli;
mod domain;
mod providers;
mod transport;

use app::search_service::SearchService;
use clap::Parser;
use cli::args::{CliArgs, CliProvider};
use cli::output::render_text;
use domain::query::SearchQuery;
use providers::brave::client::BraveProvider;
use providers::brave::config::BraveConfig;
use providers::exa::client::ExaProvider;
use providers::exa::config::ExaConfig;
use transport::http::ReqwestHttpClient;

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
        println!("Currently supports Brave Search (web, news, images, video).");
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

    let service = match args.provider {
        CliProvider::Brave => {
            let config = match BraveConfig::from_env() {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("Failed to load Brave config: {}", error);
                    std::process::exit(1);
                }
            };
            let provider = BraveProvider::new(ReqwestHttpClient::new(), config);
            SearchService::new(Box::new(provider))
        }
        CliProvider::Exa => {
            let config = match ExaConfig::from_env() {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("Failed to load Exa config: {}", error);
                    std::process::exit(1);
                }
            };
            let provider = ExaProvider::new(ReqwestHttpClient::new(), config);
            SearchService::new(Box::new(provider))
        }
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
