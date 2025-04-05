use axum::{Router, routing::get, response::IntoResponse};
use tera::Tera;
use std::sync::Arc;
use crate::state::AppState;

pub fn create_app() -> Router {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Erro ao carregar templates: {}", e);
            std::process::exit(1);
        }
    };

    let shared_state = Arc::new(AppState {
        tera,
        // outros campos do estado
    });

    Router::new()
        .route("/", get(homepage))
        .with_state(shared_state)
}

async fn homepage(state: axum::extract::State<Arc<AppState>>) -> impl IntoResponse {
    let rendered = state.tera.render("index.html.tera", &tera::Context::new())
        .unwrap_or_else(|_| "Erro ao renderizar".to_string());

    axum::response::Html(rendered)
}
