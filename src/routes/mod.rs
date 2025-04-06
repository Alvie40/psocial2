use axum::Router;
use std::sync::Arc;
use crate::state::AppState;

mod auth;

pub fn public_routes() -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth::auth_routes())
}
