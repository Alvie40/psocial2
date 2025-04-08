use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{
    auth::{
        extractor::AuthedUser,
        jwt::{extract_token, validate_token},
    },
    state::AppState,
};

pub async fn auth_middleware(
    State(_state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let Some(token) = extract_token(&req) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let Some(claims) = validate_token(&token) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    req.extensions_mut().insert(AuthedUser {
        user_id: claims.sub.parse().unwrap(),
        categoria: claims.categoria.clone(),
    });

    Ok(next.run(req).await)
}
