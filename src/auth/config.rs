use std::time::Duration;

/// Configuração para JWT
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Chave secreta para assinatura do token
    pub secret: String,
    /// Duração do token em segundos
    pub expiration: Duration,
    /// Algoritmo de assinatura (HS256 por padrão)
    pub algorithm: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "chave_secreta_padrao".to_string()),
            expiration: Duration::from_secs(3600), // 1 hora por padrão
            algorithm: "HS256".to_string(),
        }
    }
}

impl JwtConfig {
    /// Cria uma nova configuração de JWT a partir de variáveis de ambiente
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET deve estar definido");
        
        // Duração em segundos (padrão: 1 hora)
        let expiration_secs = std::env::var("JWT_EXPIRATION_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3600);
            
        Self {
            secret,
            expiration: Duration::from_secs(expiration_secs),
            algorithm: "HS256".to_string(),
        }
    }
} 