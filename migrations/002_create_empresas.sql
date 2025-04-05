-- 002_create_empresas.sql

CREATE TABLE IF NOT EXISTS empresas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nome TEXT NOT NULL UNIQUE,
    cnpj TEXT NOT NULL UNIQUE,
    telefone TEXT NOT NULL,
    email TEXT NOT NULL,
    pessoa_contato TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
