-- 0003_create_mensagens.sql
CREATE TABLE IF NOT EXISTS mensagens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    contato TEXT NOT NULL,
    direcao TEXT NOT NULL CHECK (direcao IN ('enviada', 'recebida')),
    corpo TEXT NOT NULL,
    midia_url TEXT,
    status TEXT,
    data_envio TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_mensagens_user_id ON mensagens(user_id);
CREATE INDEX idx_mensagens_contato ON mensagens(contato);
CREATE INDEX idx_mensagens_data_envio ON mensagens(data_envio);


