use std::error::Error;

use axum::{
    serve,
    middleware::{self, Next},
    http::Request,
    body::Body,
    response::Response,
};
use dotenvy::dotenv;
use psocial2::create_app;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Middleware de log personalizado
async fn log_middleware(req: Request<Body>, next: Next) -> Response {
    println!("📥 Requisição: {} {}", req.method(), req.uri());

    let response = next.run(req).await;

    println!("📤 Resposta: {}", response.status());
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🌐 Servidor iniciado em http://127.0.0.1:3000");

    // ⚠️ Aqui está o await necessário!
    let app = create_app().await
        .layer(middleware::from_fn(log_middleware));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    serve(listener, app.into_make_service()).await?;

    Ok(())
}
