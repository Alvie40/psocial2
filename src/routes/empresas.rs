use axum::{
    routing::{post, get, put, delete},
    Router,
};
use std::sync::Arc;
use crate::state::AppState;
use crate::empresas::handler::{create_empresa, list_empresas, update_empresa, delete_empresa};
use crate::auth::extractor::AuthedUser;

pub fn empresas_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/empresas", post(create_empresa))
        .route("/empresas", get(list_empresas))
        .route("/empresas/:id", put(update_empresa))
        .route("/empresas/:id", delete(delete_empresa))
} 