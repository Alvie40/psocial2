use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
};
use uuid::Uuid;
use crate::{
    state::AppState,
    models::empresa::Empresa,
    empresas::dto::{EmpresaInput, EmpresaResponse, CreateEmpresaRequest, UpdateEmpresaRequest},
};
use sqlx::{query, query_as};
use validator::Validate;
use std::sync::Arc;
use crate::auth::extractor::AuthedUser;
use axum_macros::debug_handler;

/// Cria uma nova empresa (apenas para admins)
#[debug_handler]
pub async fn create_empresa(
    State(state): State<Arc<AppState>>,
    auth_user: AuthedUser,
    Json(payload): Json<CreateEmpresaRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verifica se o usuário é admin
    let user = query_as!(
        UserWithRole,
        "SELECT categoria FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    
    if user.categoria != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Validação
    if let Err(_errors) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Verifica se CNPJ já existe
    let exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM empresas WHERE cnpj = $1)",
        payload.cnpj
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Some(false));

    if exists.unwrap_or(false) {
        return Err(StatusCode::CONFLICT);
    }

    // Verifica se email já existe
    let exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM empresas WHERE email = $1)",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Some(false));

    if exists.unwrap_or(false) {
        return Err(StatusCode::CONFLICT);
    }

    let empresa_id = Uuid::new_v4();

    // Insere no banco
    let result = query!(
        "INSERT INTO empresas (id, nome, email, cnpj, telefone, pessoa_contato) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        empresa_id,
        payload.nome,
        payload.email,
        payload.cnpj,
        payload.telefone,
        payload.pessoa_contato
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            // Busca a empresa criada para retornar
            let empresa = query_as!(
                EmpresaResponse,
                "SELECT id, nome, email, cnpj, telefone, pessoa_contato, created_at 
                 FROM empresas 
                 WHERE id = $1",
                empresa_id
            )
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((StatusCode::CREATED, Json(empresa)))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Lista todas as empresas (para usuários autenticados)
#[debug_handler]
pub async fn list_empresas(
    State(state): State<Arc<AppState>>,
    _auth_user: AuthedUser,
) -> Result<Json<Vec<EmpresaResponse>>, StatusCode> {
    let empresas = query_as!(
        EmpresaResponse,
        "SELECT id, nome, email, cnpj, telefone, pessoa_contato, created_at 
         FROM empresas 
         ORDER BY nome"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(empresas))
}

/// Atualiza uma empresa (apenas para admins)
#[debug_handler]
pub async fn update_empresa(
    State(state): State<Arc<AppState>>,
    auth_user: AuthedUser,
    Path(empresa_id): Path<Uuid>,
    Json(payload): Json<UpdateEmpresaRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verifica se o usuário é admin
    let user = query_as!(
        UserWithRole,
        "SELECT categoria FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    
    if user.categoria != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Validação
    if let Err(errors) = payload.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Verifica se a empresa existe
    let exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM empresas WHERE id = $1)",
        empresa_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Some(false));

    if !exists.unwrap_or(false) {
        return Err(StatusCode::NOT_FOUND);
    }

    // Verifica se o novo CNPJ já existe (se estiver sendo atualizado)
    if let Some(cnpj) = &payload.cnpj {
        let exists: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM empresas WHERE cnpj = $1 AND id != $2)",
            cnpj,
            empresa_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(Some(false));

        if exists.unwrap_or(false) {
            return Err(StatusCode::CONFLICT);
        }
    }

    // Verifica se o novo email já existe (se estiver sendo atualizado)
    if let Some(email) = &payload.email {
        let exists: Option<bool> = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM empresas WHERE email = $1 AND id != $2)",
            email,
            empresa_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(Some(false));

        if exists.unwrap_or(false) {
            return Err(StatusCode::CONFLICT);
        }
    }

    // Constrói a query de atualização dinamicamente
    let mut updates = Vec::new();
    let mut params = Vec::new();
    let mut param_index = 1;

    if let Some(nome) = &payload.nome {
        updates.push(format!("nome = ${}", param_index));
        params.push(nome);
        param_index += 1;
    }

    if let Some(email) = &payload.email {
        updates.push(format!("email = ${}", param_index));
        params.push(email);
        param_index += 1;
    }

    if let Some(cnpj) = &payload.cnpj {
        updates.push(format!("cnpj = ${}", param_index));
        params.push(cnpj);
        param_index += 1;
    }

    if let Some(telefone) = &payload.telefone {
        updates.push(format!("telefone = ${}", param_index));
        params.push(telefone);
        param_index += 1;
    }

    if let Some(pessoa_contato) = &payload.pessoa_contato {
        updates.push(format!("pessoa_contato = ${}", param_index));
        params.push(pessoa_contato);
        param_index += 1;
    }

    if updates.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Constrói a query final
    let query_str = format!(
        "UPDATE empresas SET {} WHERE id = ${}",
        updates.join(", "),
        param_index
    );

    // Adiciona o ID da empresa aos parâmetros
    params.push(&empresa_id.to_string());

    // Executa a query
    let result = sqlx::query(&query_str)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => {
            // Busca a empresa atualizada para retornar
            let empresa = query_as!(
                EmpresaResponse,
                "SELECT id, nome, email, cnpj, telefone, pessoa_contato, created_at 
                 FROM empresas 
                 WHERE id = $1",
                empresa_id
            )
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json(empresa))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Remove uma empresa (apenas para admins)
pub async fn delete_empresa(
    State(state): State<Arc<AppState>>,
    auth_user: AuthedUser,
    Path(empresa_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    // Verifica se o usuário é admin
    let user = query_as!(
        UserWithRole,
        "SELECT categoria FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    
    if user.categoria != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Verifica se a empresa existe
    let exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM empresas WHERE id = $1)",
        empresa_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Some(false));

    if !exists.unwrap_or(false) {
        return Err(StatusCode::NOT_FOUND);
    }

    // Verifica se existem usuários vinculados a esta empresa
    let has_users: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE empresa_id = $1)",
        empresa_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .unwrap_or(Some(false));

    if has_users.unwrap_or(false) {
        return Err(StatusCode::CONFLICT);
    }

    // Remove a empresa
    let result = query!(
        "DELETE FROM empresas WHERE id = $1",
        empresa_id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug)]
struct UserWithRole {
    categoria: String,
} 