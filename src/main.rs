use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    dotenvy::dotenv().ok();

    let exit_code = sophon_cli::cli::runner::run_from_env().await;
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
