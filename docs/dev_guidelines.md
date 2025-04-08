# 🛠️ Guia de Desenvolvimento - Projeto Psocial2

Este guia foi criado para ajudar no entendimento e na organização da arquitetura do projeto `psocial2`, baseado em Rust, Axum, Tera, SQLx e PostgreSQL. Aqui você encontrará explicações sobre os padrões adotados, boas práticas e estrutura de código.

---

## 📁 Estrutura de Diretórios

```bash
src/
├── auth/              # Autenticação e login (handlers, serviços, jwt, etc)
├── config/            # Configurações da aplicação (ex: leitura de .env)
├── db/                # Conexão e lógica do banco de dados
├── dto/               # Data Transfer Objects (structs de entrada/saída)
├── mailers/           # Envio de e-mails e notificações
├── middlewares/       # Middlewares como autenticação e logging
├── models/            # Modelos de domínio (User, Empresa, etc)
├── routes/            # Arquivos com rotas organizadas por funcionalidade
├── templates/         # Arquivos HTML com Tera + HTMX
├── utils/             # Funções utilitárias e auxiliares (ex: valida CPF)
└── main.rs            # Ponto de entrada da aplicação
```

---

## 📦 DTO (Data Transfer Object)

### 📌 O que é?
DTOs são **estruturas intermediárias** usadas para **entrada e saída de dados** na aplicação.

### ✅ Benefícios
- Separação entre o modelo de banco e o que vai para a API
- Evita acoplamento entre camadas
- Facilita testes unitários e validações com `validator`
- Pode evoluir a API sem mudar a estrutura do banco

### 🛠️ Onde usar
- `dto::auth::LoginPayload` → recebido no login
- `dto::user::RegisterPayload` → usado para criar usuários
- `dto::user::UserResponse` → resposta enviada pela API

---

## 🧱 Models vs DTOs

| Finalidade        | Model                          | DTO                               |
|------------------|--------------------------------|-----------------------------------|
| Banco de Dados   | Usado com `sqlx::FromRow`      | Não se conecta diretamente ao BD  |
| API Pública      | Não usado diretamente na API   | Usado para entrada/saída de dados |
| Validação        | Não (normalmente)              | Sim (usando `validator`)          |

---

## 📡 Handlers (Rotas)

- Responsáveis por **receber e responder requisições HTTP**
- Devem ser simples, delegando lógica para `services`
- Utilizam DTOs como entrada/saída

### Exemplo:
```rust
pub async fn login(Json(payload): Json<LoginPayload>, ctx: AppContext) -> impl IntoResponse {
    let token = AuthService::login(&ctx, payload).await?;
    Json(json!({ "access_token": token }))
}
```

---

## 🧠 Services

- Contêm a **lógica de negócio**
- Usam DTOs para comunicação com handlers
- Podem acessar o banco via funções de `db` ou `models`
- São testáveis isoladamente

---

## 🔐 Middlewares

- Tratam autenticação, logging, headers, etc.
- Ex: validação de token JWT

---

## 📝 Templates

- HTML com [Tera](https://tera.netlify.app/) e [HTMX](https://htmx.org/)
- Ajudam na criação de interfaces dinâmicas sem JavaScript pesado

---

## 📋 Validações

- Usamos o crate [`validator`](https://docs.rs/validator/latest/validator/) em DTOs para validar CPF, e-mail, etc.
- As validações são aplicadas **automaticamente** ao receber os dados.

---

## ✅ Testes

- Devem ser feitos para `services` e `handlers`
- Mocks podem ser usados para isolar dependências como DB e e-mail

---

## 🧪 Ambiente de Desenvolvimento

- Crie o `.env` com base no `.env.example`
- Use `cargo run` para rodar a aplicação
- Use `sqlx` com `cargo sqlx prepare` para checar queries

---

## 🧠 Convenções Gerais

- Use `snake_case` nos arquivos e pastas
- Exporte apenas o que for necessário de cada módulo
- Prefira `Result<T, AppError>` com erros padronizados
- Documente as funções públicas

---

## 🧰 Ferramentas Recomendadas

- [sqlx-cli](https://docs.rs/sqlx/latest/sqlx/macro.query.html) para gestão do banco

---

Se tiver dúvidas sobre organização ou precisar estender o projeto, siga este guia ou converse com o Alvaro. 😉

---

**Mantenha o projeto limpo, modular e bem testado!** 💪

