use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Twilio {
    pub id: Uuid,
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub empresa_id: Uuid,
    pub data_criacao: DateTime<Utc>,
    pub data_alteracao: DateTime<Utc>,
} 