use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{state::AppState, auth::jwt::generate_token};
use sqlx::query_as;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub senha: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user = query_as!(
        LoginUser,
        "SELECT id, email, senha_hash FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.senha_hash).unwrap();
            if Argon2::default().verify_password(payload.senha.as_bytes(), &parsed_hash).is_ok() {
                let token = generate_token(user.id.to_string());
                Ok(Json(serde_json::json!({ "token": token })))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Senha inválida"))
            }
        }
        _ => Err((StatusCode::UNAUTHORIZED, "Usuário não encontrado")),
    }
}

struct LoginUser {
    id: uuid::Uuid,
    email: String,
    senha_hash: String,
}
