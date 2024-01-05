use axum::Router;

use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
}
