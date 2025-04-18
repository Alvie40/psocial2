use sqlx::PgPool;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub tera: Tera,
    pub db: PgPool,
}
