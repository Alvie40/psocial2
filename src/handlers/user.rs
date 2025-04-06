use axum::{extract::{State, Json}, http::StatusCode};
use std::sync::Arc;
use crate::state::AppState;
use crate::auth::extractor::AuthedUser;
use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse {
    id: uuid::Uuid,
}

pub async fn me(State(state): State<Arc<AppState>>, AuthedUser { user_id }: AuthedUser) -> Result<Json<MeResponse>, StatusCode> {
    Ok(Json(MeResponse { id: user_id }))
}
