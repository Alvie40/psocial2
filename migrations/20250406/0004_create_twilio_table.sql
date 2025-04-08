CREATE TABLE IF NOT EXISTS twilio (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_sid TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    from_number TEXT NOT NULL,
    empresa_id UUID REFERENCES empresas(id) ON DELETE CASCADE,
    data_criacao TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    data_alteracao TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Índices
CREATE INDEX idx_twilio_empresa_id ON twilio(empresa_id);

-- Trigger para atualizar data_alteracao
CREATE TRIGGER set_timestamp
    BEFORE UPDATE ON twilio
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_timestamp(); 