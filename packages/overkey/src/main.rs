use overrid_overkey::{OverkeyConfig, OverkeyService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "overrid_overkey=info,tower_http=info".into()),
        )
        .init();

    let config = OverkeyConfig::from_env();
    let bind_addr = config.bind_addr.clone();
    let service = OverkeyService::new(config);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    tracing::info!(
        service_id = %service.state().config.service_id,
        public_base_path = %service.state().config.public_base_path,
        bind_addr = %bind_addr,
        "starting Overkey service"
    );

    axum::serve(listener, service.router())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install terminate handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
