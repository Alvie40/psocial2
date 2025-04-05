# Arquitetura do Projeto `psocial2`

O projeto `psocial2` é uma aplicação web backend desenvolvida em Rust com o framework Axum, voltada para atendimento psicossocial com integração via WhatsApp (Twilio), autenticação JWT, geração de relatórios em PDF e renderização de views com Tera + HTMX + Tailwind CSS.

## 🔧 Stack Tecnológica

| Categoria                | Função                                     | Crates / Ferramentas                                         |
|--------------------------|--------------------------------------------|--------------------------------------------------------------|
| **Framework Web**        | HTTP server e roteamento                   | `axum = "0.8.3"`                                             |
| **Async Runtime**        | Executor de I/O assíncrono                 | `tokio = "1.36"` com `"full"`                                |
| **Banco de Dados**       | ORM explícito + queries SQL                | `sqlx = "0.7.3"` com `postgres`, `macros`, `uuid`, `chrono`  |
| **Identificadores**      | UUIDs nas entidades                        | `uuid = "1.6.1"`                                             |
| **Datas e Horários**     | Manipulação de datas                       | `chrono = "0.4.34"` com `serde`                              |
| **Templates HTML**       | Frontend com renderização de views         | `tera = "1.19.1"`                                             |
| **Frontend CSS**         | Estilo moderno com classes utilitárias     | **Tailwind CSS** (via CDN inicialmente)                      |
| **Tabelas no Frontend**  | Exibição de dados com visual organizado    | `HTMX` + `Tera` + Tailwind classes (`table`, `grid`, etc.)   |
| **Autenticação JWT**     | Login e autorização                        | `jsonwebtoken = "9.2.0"`                                     |
| **Senhas seguras**       | Armazenamento seguro                       | `argon2 = "0.5.2"` + `rand_core = "0.6.4"`                    |
| **Validação automática** | Campos obrigatórios, e-mails válidos etc.  | `validator = "0.16.1"`                                       |
| **Serialização JSON**    | API REST                                   | `serde = "1.0.197"`, `serde_json = "1.0.114"`                |
| **.env configs**         | Carregar secrets locais                    | `dotenvy = "0.15.7"`                                         |
| **Logs estruturados**    | Debug e rastreabilidade                    | `tracing = "0.1.40"`, `tracing-subscriber = "0.3.18"`       |
| **Erros elegantes**      | Trait `Error`, integração com `?`          | `thiserror = "1.0.58"`                                       |
| **Integração Twilio**    | WhatsApp/SMS via API oficial               | `twilio = "0.5.0"`                                           |
| **Geração de PDF**       | Cria relatórios em PDF no backend           | `printpdf = "0.6.0"`                                         |
| **Testes Unitários**     | TDD, testes de domínio e handlers          | `tokio::test`, `assert_eq!`, `#[cfg(test)]` etc.             |

## 📆 Padrões de Projeto
- **Orientado a Domínio (DDD)** com separação de camadas
- **Responsabilidade bem definida** entre `models`, `handlers`, `routes`, `state`
- **Testes unitários desde o início**, com foco em entidades e regras de negócio

## 🌐 Frontend
- `Tera` para renderizar HTML diretamente com dados
- `HTMX` para interatividade sem JavaScript complexo
- `Tailwind CSS` via CDN para responsividade e tabelas bonitas

## 🚀 Integrações
- `Twilio`: envio de mensagens via WhatsApp
- `printpdf`: criação de relatórios PDF para download ou armazenamento

## 🔄 Migrations
- Utiliza `sqlx migrate` com arquivos `.sql` simples e versionados

## ✅ Exclusões propositalmente evitadas
- `SeaORM`, `Refinery`, `Loco.rs`, `Leptos`, `Rocket`, `Redis`, `GraphQL`
- Nenhum framework opinativo será usado
- Foco total em legibilidade, extensibilidade e simplicidade para IA/Dev

