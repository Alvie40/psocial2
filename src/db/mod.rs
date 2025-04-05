use std::env;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

/// Alias para simplificar o uso do pool
pub type Db = Pool<Postgres>;

/// Função assíncrona que cria a conexão com o banco
pub async fn connect() -> Db {
    // Lê a variável DATABASE_URL do .env
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida no .env");

    // Cria o pool de conexões com até 10 conexões simultâneas
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar no banco de dados")
}
