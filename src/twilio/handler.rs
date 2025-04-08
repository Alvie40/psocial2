use axum::{
    extract::{Form, Json, State},
    response::IntoResponse,
    http::StatusCode,
};
use crate::{
    state::AppState,
    twilio::dto::SendWhatsAppInput,
};
use std::sync::Arc;
use reqwest::Client as HttpClient;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde_json::json;
use validator::Validate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    pub From: String,
    pub Body: String,
    pub MessageSid: String,
}

/// Envia uma mensagem via WhatsApp usando a API do Twilio
pub async fn enviar_whatsapp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendWhatsAppInput>,
) -> Result<impl IntoResponse, StatusCode> {
    payload.validate().map_err(|e| {
        tracing::warn!("Erro de validação no payload: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    let account_sid = std::env::var("TWILIO_ACCOUNT_SID")
        .unwrap_or_else(|_| "SEU_ACCOUNT_SID_AQUI".to_string());
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN")
        .unwrap_or_else(|_| "SEU_AUTH_TOKEN_AQUI".to_string());
    let from_number = std::env::var("TWILIO_FROM_NUMBER")
        .unwrap_or_else(|_| "whatsapp:+SEU_NUMERO_AQUI".to_string());

    let client = HttpClient::new();
    let auth = format!("{}:{}", account_sid, auth_token);
    let auth_header = format!("Basic {}", BASE64.encode(auth));

    let url = format!(
        "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
        account_sid
    );

    let form_data = vec![
        ("To", payload.to_number.clone()),
        ("From", from_number.clone()),
        ("Body", payload.message.clone()),
    ];

    let response = client
        .post(&url)
        .header("Authorization", auth_header)
        .form(&form_data)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Erro ao enviar mensagem WhatsApp: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let status = response.status();
    let response_json = response.json::<serde_json::Value>().await.map_err(|e| {
        tracing::error!("Erro ao parsear resposta do Twilio: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !status.is_success() {
        tracing::error!("Erro na resposta do Twilio: {:?}", response_json);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let sid = response_json["sid"]
        .as_str()
        .unwrap_or("desconhecido")
        .to_string();

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Mensagem WhatsApp enviada com sucesso",
            "sid": sid,
            "to": payload.to_number,
            "from": from_number
        })),
    ))
}

/// Recebe mensagens enviadas ao webhook da Twilio (modo sandbox)
pub async fn receive_message(
    State(_state): State<Arc<AppState>>,
    Form(payload): Form<IncomingMessage>,
) -> StatusCode {
    tracing::info!(
        "📩 Mensagem recebida de {}: {} (SID: {})",
        payload.From,
        payload.Body,
        payload.MessageSid
    );

    StatusCode::OK
}
