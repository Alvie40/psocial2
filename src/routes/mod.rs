use axum::Router;
use std::sync::Arc;
use crate::state::AppState;

mod auth;
mod twilio_routes;

// pub fn public_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
//     Router::new()
//         .merge(auth::auth_routes())
//         .merge(twilio_routes::routes(state))
// }

pub fn public_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .merge(auth::auth_routes().with_state(state.clone()))
        .merge(twilio_routes::routes(state))
}