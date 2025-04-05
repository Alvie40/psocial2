use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};
use crate::auth::jwt::validate_token;
use uuid::Uuid;

pub struct AuthedUser {
    pub user_id: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthedUser
where
    S: Send + Sync,
{
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, axum::http::StatusCode> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

        let token_data = validate_token(bearer.token())
            .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

        Ok(AuthedUser {
            user_id: Uuid::parse_str(&token_data.claims.sub)
                .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?,
        })
    }
}
