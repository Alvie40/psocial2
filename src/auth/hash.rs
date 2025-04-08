use argon2::{self, Config};

pub fn hash_password(password: &str) -> String {
    let salt = b"static-salt"; // Substitua por salt seguro no futuro
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default()).unwrap()
} 