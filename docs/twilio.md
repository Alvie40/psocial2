# Documentação do Módulo Twilio

## Visão Geral
Este documento descreve a implementação do módulo Twilio para envio de mensagens WhatsApp no sistema. A integração é feita diretamente com a API REST do Twilio, sem uso de bibliotecas específicas.

## 1. Estrutura do Banco de Dados
```sql
CREATE TABLE twilio (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_sid VARCHAR NOT NULL,
    auth_token VARCHAR NOT NULL,
    from_number VARCHAR NOT NULL,
    empresa_id UUID NOT NULL REFERENCES empresas(id),
    data_criacao TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    data_alteracao TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
- Armazena as credenciais do Twilio por empresa
- Cada empresa pode ter sua própria configuração do Twilio

## 2. Modelo (src/models/twilio.rs)
```rust
pub struct Twilio {
    pub id: Uuid,
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub empresa_id: Uuid,
    pub data_criacao: DateTime<Utc>,
    pub data_alteracao: DateTime<Utc>,
}
```
- Representa a tabela do banco de dados
- Usa `serde` para serialização/deserialização

## 3. DTOs (src/twilio/dto.rs)
```rust
pub struct TwilioInput {
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    pub empresa_id: Uuid,
}

pub struct TwilioResponse {
    pub id: Uuid,
    pub account_sid: String,
    pub from_number: String,
    pub empresa_id: Uuid,
}

pub struct SendSmsInput {
    pub to_number: String,
    pub message: String,
    pub empresa_id: Uuid,
}
```
- `TwilioInput`: Para criar/atualizar configuração
- `TwilioResponse`: Retorna dados sem expor o token
- `SendSmsInput`: Para enviar mensagens

## 4. Handler (src/twilio/handler.rs)
```rust
pub async fn criar_twilio(
    State(state): State<AppState>,
    Json(payload): Json<TwilioInput>,
) -> Result<impl IntoResponse, StatusCode>
```
- Valida o payload
- Insere no banco de dados
- Retorna resposta sem o token

```rust
pub async fn enviar_sms(
    State(state): State<AppState>,
    Json(payload): Json<SendSmsInput>,
) -> Result<impl IntoResponse, StatusCode>
```
- Busca configuração do Twilio da empresa
- Formata números para WhatsApp
- Envia mensagem usando a API REST do Twilio
- Implementa autenticação básica
- Trata respostas e erros da API

## 5. Rotas (src/routes/twilio.rs)
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(handler::criar_twilio))
        .route("/send-sms", post(handler::enviar_sms))
}
```
- `/create`: Criar configuração do Twilio
- `/send-sms`: Enviar mensagem WhatsApp

## 6. Variáveis de Ambiente (.env)
```env
TWILIO_ACCOUNT_SID=seu_account_sid
TWILIO_AUTH_TOKEN=seu_auth_token
TWILIO_FROM_NUMBER=whatsapp:+14155238886
TWILIO_TO_TEST_NUMBER=whatsapp:+5511910114427
TWILIO_SANDBOX=true
```
- Credenciais do Twilio
- Número do sandbox para envio
- Seu número para testes
- Flag de sandbox

## 7. Uso do Sistema

### a. Criar configuração do Twilio:
```json
POST /twilio/create
{
    "account_sid": "seu_account_sid",
    "auth_token": "seu_auth_token",
    "from_number": "whatsapp:+14155238886",
    "empresa_id": "uuid-da-empresa"
}
```

### b. Enviar mensagem:
```json
POST /twilio/send-sms
{
    "to_number": "whatsapp:+5511910114427",
    "message": "Sua mensagem aqui",
    "empresa_id": "uuid-da-empresa"
}
```

## 8. Ambiente Sandbox
- Número do sandbox: `+14155238886`
- Para usar:
  1. Envie mensagem para o número do sandbox
  2. Use o código recebido para verificar seu número
  3. Aguarde a confirmação
  4. Agora você pode receber mensagens

## 9. Tratamento de Números
- Todos os números são formatados com prefixo `whatsapp:`
- Exemplo: `+5511910114427` → `whatsapp:+5511910114427`
- O sistema verifica e adiciona o prefixo se necessário

## 10. Segurança
- Tokens são armazenados de forma segura no banco
- Respostas não incluem tokens sensíveis
- Validação de payload em todas as requisições
- Middleware de autenticação nas rotas
- Autenticação básica na API do Twilio

## 11. Testes
Os testes estão implementados em `src/twilio/tests.rs` e cobrem:
- Criação de configuração do Twilio
- Envio de mensagens
- Validação de números
- Tratamento de erros

## 12. Dependências
Adicione ao `Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
base64 = "0.21"
```

## 13. Observações Importantes
1. Sempre use o prefixo `whatsapp:` nos números
2. No ambiente sandbox, os números precisam ser verificados
3. Mantenha as credenciais seguras
4. Use variáveis de ambiente para configurações sensíveis
5. Monitore os logs para erros de envio
6. A implementação manual permite maior controle sobre a integração
7. A autenticação é feita usando Basic Auth com as credenciais do Twilio
8. Os erros da API são logados para facilitar o debug 