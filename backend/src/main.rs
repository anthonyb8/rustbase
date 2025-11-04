use app::config::CONFIG;
use app::logger::init_global_logger;
use app::router::router;
use app::state::AppState;
use app::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup Logging
    // init_global_logger()?;

    // Initialize the Axum routing service
    let state = AppState::new().await?;
    println!("made statea");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;
    // let listener = TcpListener::bind(&CONFIG.app_url).await?;
    tracing::info!("Listening on {}", &CONFIG.app_url);

    // Run the server
    let app = router(state);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
