use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub empresa_id: Uuid,
    pub categoria: String,
    pub nome: String,
    pub cpf: String,
    pub telefone: String,
    pub email: String,
    pub senha: String,
    pub created_at: DateTime<Utc>,
}
