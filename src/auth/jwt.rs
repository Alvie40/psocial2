use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

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
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Erro ao gerar token")
}
