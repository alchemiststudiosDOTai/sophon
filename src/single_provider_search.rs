use sophon_cli::bootstrap::provider_registry::{ProviderId, ProviderRegistry};
use sophon_cli::cli::output::render_text;
use sophon_cli::domain::SearchQuery;

pub async fn run_single_provider(
    registry: &ProviderRegistry,
    provider_id: ProviderId,
    query: SearchQuery,
) {
    tracing::info!(provider = %provider_id, query = %query.text, "initializing search service");

    let service = registry.build(provider_id).unwrap_or_else(|error| {
        tracing::error!(%error, "failed to build provider");
        eprintln!("{error}");
        std::process::exit(1);
    });

    match service.search(query).await {
        Ok(response) => {
            tracing::info!(result_count = response.results.len(), total_estimated = ?response.total_estimated, "search completed");
            println!("{}", render_text(&response));
        }
        Err(e) => {
            tracing::error!(error = %e, "search failed");
            eprintln!("Search failed: {}", e);
            std::process::exit(1);
        }
    }
}
