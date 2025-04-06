use axum::{
    http::{request::Parts, StatusCode},
    extract::FromRequestParts,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use async_trait::async_trait;
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
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Token ausente ou inválido"))?;

        let claims = validate_token(bearer.token())
            .ok_or((StatusCode::UNAUTHORIZED, "Token inválido"))?;

        Ok(AuthedUser {
            user_id: Uuid::parse_str(&claims.sub)
                .map_err(|_| (StatusCode::UNAUTHORIZED, "ID de usuário inválido"))?,
        })
    }
}
