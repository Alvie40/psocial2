use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendWhatsAppInput {
    #[validate(length(min = 1, message = "Número de destino é obrigatório"))]
    #[validate(custom(function = "validate_whatsapp_number", message = "Número de destino deve estar no formato whatsapp:+DDD"))]
    pub to_number: String,
    #[validate(length(min = 1, message = "Mensagem é obrigatória"))]
    pub message: String,
    pub empresa_id: uuid::Uuid,
}

fn validate_whatsapp_number(value: &str) -> Result<(), validator::ValidationError> {
    if value.starts_with("whatsapp:") && value.len() > 10 && value[9..].chars().all(|c| c.is_digit(10) || c == '+') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("Número deve estar no formato whatsapp:+DDD"))
    }
}