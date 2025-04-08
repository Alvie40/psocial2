use std::error::Error;
use std::sync::Arc;

use axum::{
    serve,
    middleware::{self, Next},
    http::Request,
    body::Body,
    response::Response,
};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use psocial2::create_app;

/// Middleware de log personalizado
async fn log_middleware(req: Request<Body>, next: Next) -> Response {
    println!("📥 Requisição: {} {}", req.method(), req.uri());

    let response = next.run(req).await;

    println!("📤 Resposta: {}", response.status());
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🌐 Servidor iniciado em http://127.0.0.1:3000");

    // Cria o app com estado e rotas
    let app = create_app().await
        .layer(middleware::from_fn(log_middleware));

    // Cria listener com tokio
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    // Serve a aplicação corretamente (Axum 0.8 + Hyper 1.6)
    serve(listener, app).await?;

    Ok(())
}
