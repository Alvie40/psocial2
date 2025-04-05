use axum::{extract::State, response::Html};
use std::sync::Arc;
use tera::Context;

use crate::state::AppState;

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let ctx = Context::new();
    let rendered = state
        .tera
        .render("index.html.tera", &ctx)
        .unwrap_or_else(|_| "<h1>Erro ao renderizar</h1>".to_string());

    Html(rendered)
}
