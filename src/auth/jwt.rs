use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization, Header};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Cabeçalho inválido".to_string()))?;

        let token = bearer.token();

        validate_token(token).ok_or((StatusCode::UNAUTHORIZED, "Token inválido".to_string()))
    }
}

pub fn validate_token(token: &str) -> Option<Claims> {
    let secret = std::env::var("JWT_SECRET").ok()?;
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .ok()?;

    Some(decoded.claims)
}

use jsonwebtoken::{encode, Header as JwtHeader, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub fn generate_token(user_id: String) -> String {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(60 * 60)) // 1 hora
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET não definido");
    encode(&JwtHeader::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Erro ao gerar token")
}
