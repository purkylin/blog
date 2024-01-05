use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod common;
mod routes;
mod state;
mod view;

use sqlx::postgres::PgPoolOptions;
use state::AppState;
use std::env;

#[tokio::main]
async fn main() {
    // load dot env config
    dotenvy::dotenv().ok();

    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DB_URL").expect("not found DB_URL");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("connect db failed");

    let state: Arc<AppState> = Arc::new(AppState { pool });

    let addr = "0.0.0.0:8000";
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = routes::create_app(state);
    info!("start app on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("start server failed");
}
