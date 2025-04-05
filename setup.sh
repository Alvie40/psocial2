#!/bin/bash

echo "🦀 Iniciando projeto Rust 'psocial2'..."

# Inicializa cargo
cargo init --vcs git psocial2
cd psocial2 || exit

# Cria estrutura de diretórios
mkdir -p src/{routes,handlers,models,db}
touch src/{app.rs,state.rs,errors.rs,utils.rs}
touch src/routes/mod.rs src/handlers/mod.rs src/db/mod.rs

# Cria README e .env exemplo
echo "# Psocial2" > README.md
cp .env.example .env 2>/dev/null || echo 'DATABASE_URL=postgres://postgres:postgres@localhost/psocial2' > .env

# Atualiza Cargo.toml com dependências essenciais
cat >> Cargo.toml <<EOF

[dependencies]
axum = "0.8.3"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "macros", "uuid", "chrono"] }
uuid = { version = "1" }
dotenvy = "0.15"
tracing = "0.1"
EOF

echo "✅ Estrutura criada com sucesso!"
