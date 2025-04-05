# Estrutura Modular do Projeto `psocial2`

O projeto `psocial2` foi planejado para ser **extremamente modular, limpo e orientado a domínio**, visando facilitar manutenção, escalabilidade e colaboração assistida por IA.

## Estrutura de Diretórios

```bash
src/
├── app.rs               # Ponto de entrada da aplicação (cria roteador)
├── main.rs              # Inicializa o servidor (usa app.rs)
├── state.rs             # Define AppState compartilhado entre handlers
├── utils.rs             # Funções auxiliares reutilizáveis
├── errors.rs            # Definições centralizadas de erros
│
├── db/                  # Conexão e acesso ao banco de dados
│   └── mod.rs
│
├── models/              # Entidades e estruturas persistidas (ex: User, Questionario)
│   └── mod.rs
│
├── handlers/            # Lógica dos endpoints (camada de aplicação)
│   └── mod.rs
│
├── routes/              # Define as rotas da aplicação (camada de interface HTTP)
│   └── mod.rs
│
├── workers/             # Serviços de segundo plano, fila, tarefas async
│   └── mod.rs
│
├── helpers/             # Funções utilitárias que não são core (ex: formatação, e-mail)
    └── mod.rs
```

## Outros Diretórios

```bash
templates/               # HTML com Tera para views web (HTMX + Tailwind)
static/                  # Assets estáticos (JS/CSS se houver necessidade)
tests/                   # Testes unitários e de integração
src/bin/                 # Entradas para cargo devx, testx, buildx
```

## Convenções Adotadas

- Cada módulo tem seu `mod.rs` com `pub mod` para submódulos e reexports.
- O `app.rs` monta o roteador com `Router<AppState>` e injeta rotas.
- O `main.rs` é minimalista e apenas inicializa o `AppState` e chama `create_app()`.
- O `state.rs` define um struct `AppState` contendo conexões, configs, etc.
- Testes vivem em `tests/` ou dentro dos módulos com `#[cfg(test)]`.

---

Se você aprovar essa estrutura, posso gerar o **script de organização do projeto** e depois **corrigir o `main.rs`** conforme o padrão.

