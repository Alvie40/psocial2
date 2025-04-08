use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// DTO para criação de empresa
#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmpresaRequest {
    #[validate(length(min = 3, message = "Nome deve ter no mínimo 3 caracteres"))]
    pub nome: String,

    #[validate(length(min = 14, max = 14, message = "CNPJ deve ter 14 caracteres"))]
    pub cnpj: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 10, max = 11))]
    pub telefone: String,

    #[validate(length(min = 2, max = 100))]
    pub pessoa_contato: String,
}

/// DTO para atualização de empresa
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmpresaRequest {
    #[validate(length(min = 3, message = "Nome deve ter no mínimo 3 caracteres"))]
    pub nome: Option<String>,

    #[validate(length(min = 14, max = 14, message = "CNPJ deve ter 14 caracteres"))]
    pub cnpj: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 10, max = 11))]
    pub telefone: Option<String>,

    #[validate(length(min = 2, max = 100))]
    pub pessoa_contato: Option<String>,
}

/// DTO para resposta com dados da empresa
#[derive(Debug, Serialize)]
pub struct EmpresaResponse {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub cnpj: String,
    pub telefone: String,
    pub pessoa_contato: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmpresaInput {
    #[validate(length(min = 3, message = "Nome deve ter no mínimo 3 caracteres"))]
    pub nome: String,
    #[validate(length(min = 14, max = 14, message = "CNPJ deve ter 14 caracteres"))]
    pub cnpj: String,
} 