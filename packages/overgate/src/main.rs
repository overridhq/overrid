use overrid_overgate::{OvergateConfig, OvergateService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let config = OvergateConfig::from_env();
    let bind_addr = config.bind_addr.clone();
    let app = OvergateService::new(config).router();
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!(service = "service:overgate", %bind_addr, "starting Overgate Phase 2 service skeleton");
    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    let env_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "overgate=info,overrid_overgate=info,tower_http=info".to_owned());
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init();
}
