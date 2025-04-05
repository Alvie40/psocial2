-- Habilita extensão para UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabela de usuários
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    empresa_id UUID NOT NULL,
    categoria TEXT NOT NULL CHECK (categoria IN ('admin', 'gestor', 'psicologo')),
    nome TEXT NOT NULL,
    cpf TEXT NOT NULL UNIQUE,
    telefone TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    senha_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Exemplo de tabela de empresas (caso não exista ainda, para satisfazer a FK)
-- Você pode comentar isso se for criar em uma migração separada.
CREATE TABLE IF NOT EXISTS empresas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT NOT NULL
);

-- Adiciona a constraint de chave estrangeira
ALTER TABLE users
ADD CONSTRAINT fk_empresa
FOREIGN KEY (empresa_id)
REFERENCES empresas(id)
ON DELETE CASCADE;
