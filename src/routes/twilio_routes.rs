use axum::{Router, routing::post};
use std::sync::Arc;
use crate::{
    state::AppState,
    twilio::handler::{enviar_whatsapp, receive_message},
};

// pub fn routes(state: Arc<AppState>) -> Router {
//     Router::new()
//         .route("/twilio/send", post(enviar_whatsapp))
//         .route("/twilio/receive", post(receive_message))
//         .with_state(state)
// }
pub fn routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/twilio/send", post(enviar_whatsapp))
        .route("/twilio/receive", post(receive_message))
        .with_state(state)
}