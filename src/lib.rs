use axum::{Router, routing::get};
use std::sync::Arc;
use tera::Tera;

pub mod state;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod helpers;
pub mod models;
pub mod routes;
pub mod utils;
pub mod workers;
pub mod auth;
pub mod twilio;


use state::AppState;
use handlers::home::index as homepage;
use routes::public_routes;

pub async fn create_app() -> Router<Arc<AppState>> {
    // Carrega os templates
    let tera = Tera::new("templates/**/*").expect("Erro ao carregar templates");

    // Conecta ao banco de dados
    let db = db::connect().await;

    // Compartilha o estado com a aplicação
    let shared_state = Arc::new(AppState { tera, db });

    // Cria o router
    public_routes(shared_state.clone())
    .route("/", get(homepage))
    .with_state(shared_state)

}

