use std::sync::Arc;

use axum::{http::Uri, routing::get, Router};
use tower_http::trace::TraceLayer;

use crate::state::AppState;

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", crate::api::routes())
        .nest("/view", crate::view::routes())
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn root() -> &'static str {
    "It works"
}

async fn not_found(uri: Uri) -> String {
    format!("Not found: {}", uri)
}
