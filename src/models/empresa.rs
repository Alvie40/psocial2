use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empresa {
    pub id: Uuid,
    pub nome: String,
    pub cnpj: String,
} 