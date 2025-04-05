use crate::app::create_app;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
