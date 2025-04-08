use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

/// DTO para requisição de login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// DTO para resposta de login
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

/// DTO para registro de novo usuário
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100))]
    pub nome: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub senha: String,

    #[validate(custom = "crate::utils::validar_cpf")]
    pub cpf: String,

    #[validate(custom = "crate::utils::validar_telefone")]
    pub telefone: String,
}

/// DTO para resposta de registro
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub empresa_id: Uuid,
    pub token: String,
    pub mensagem: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterInput {
    pub nome: String,
    pub cpf: String,
    pub telefone: String,
    pub email: String,
    pub senha: String,
    pub categoria: String,
    pub empresa_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginInput {
    pub email: String,
    pub senha: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub empresa_id: Uuid,
} 