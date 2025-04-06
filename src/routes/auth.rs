// src/routes/auth.rs

use axum::{routing::{post, get}, Router};
use crate::auth::handler::{login, register_user, me};
use std::sync::Arc;
use crate::state::AppState;

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/auth/register", post(register_user))
        .route("/me", get(me))
}
