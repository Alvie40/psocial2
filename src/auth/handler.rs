use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    //routing::get,
    Json as AxumJson,
};
use serde::{Deserialize};
use std::sync::Arc;
use sqlx::{query, query_as};
use uuid::Uuid;
use validator::Validate; 
use serde_json::json;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use argon2::PasswordHasher;
use crate::state::AppState;
use crate::auth::jwt::{generate_token, Claims};
use crate::utils::validar_cpf;

/// =======================
/// ===== LOGIN ===========
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
        "SELECT id, email, senha FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.senha).unwrap();
            if Argon2::default().verify_password(payload.senha.as_bytes(), &parsed_hash).is_ok() {
                let token = generate_token(user.id.to_string());
                Ok(Json(json!({ "token": token })))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Senha inválida"))
            }
        }
        _ => Err((StatusCode::UNAUTHORIZED, "Usuário não encontrado")),
    }
}

#[derive(Debug)]
struct LoginUser {
    id: Uuid,
    email: String,
    senha: String,
}

/// ========== ME (rota protegida) ===========
pub async fn me(claims: Claims) -> AxumJson<serde_json::Value> {
    AxumJson(json!({
        "user_id": claims.sub,
        "expira_em": claims.exp
    }))
}

/// =======================
/// ==== REGISTER =========
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6, message = "Senha muito curta"))]
    pub senha: String,

    #[validate(length(min = 11, max = 11), custom = "validar_cpf")]
    pub cpf: String,

    #[validate(length(min = 10, max = 15))]
    pub telefone: String,
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // Validação
    if let Err(errors) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("Erros de validação: {:?}", errors)));
    }

    // Verifica se email já existe
    let exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro no banco: {}", e)))?
    .unwrap_or(Some(false));

    let exists = exists.unwrap_or(false);

    if exists {
        return Err((StatusCode::CONFLICT, "Email já cadastrado".to_string()));
    }

    // Gera hash da senha
    let salt = SaltString::generate(&mut OsRng);
    let senha = Argon2::default()
        .hash_password(payload.senha.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user_id = Uuid::new_v4();

    // Insere no banco
    let result = query!(
        "INSERT INTO users (id, email, senha, cpf, telefone) VALUES ($1, $2, $3, $4, $5)",
        user_id,
        payload.email,
        senha,
        payload.cpf,
        payload.telefone
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({ "id": user_id, "mensagem": "Usuário registrado com sucesso" }))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao registrar usuário".to_string())),
    }
}
