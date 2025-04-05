use tera::Tera;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub tera: Tera,
    // adicione outros campos, como pool do banco
}
