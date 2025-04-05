#!/bin/bash

echo "🔧 Criando estrutura modular para o projeto psocial2..."

mkdir -p src/{handlers,routes,models,db,workers,helpers,bin}
mkdir -p templates static tests

touch src/{app.rs,state.rs,utils.rs,errors.rs,main.rs}
touch src/handlers/mod.rs
touch src/routes/mod.rs
touch src/models/mod.rs
touch src/db/mod.rs
touch src/workers/mod.rs
touch src/helpers/mod.rs

echo "✅ Estrutura modular criada com sucesso!"
