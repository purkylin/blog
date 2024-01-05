use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub type SharedState = Arc<AppState>;
pub struct AppState {
    pub pool: Pool<Postgres>,
}
