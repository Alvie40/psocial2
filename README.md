# PSocial2

**PSocial2** é uma plataforma web moderna e minimalista para gestão de atendimentos e comunicação em ambientes ocupacionais e de saúde, com foco em desempenho, extensibilidade e integração com sistemas externos como WhatsApp (Twilio), eSocial, e outros. Desenvolvido em **Rust** com **Axum 0.8.3**, este projeto adota uma arquitetura orientada a domínio, com ênfase em testes automatizados desde o início e um frontend responsivo usando **Tera + HTMX + TailwindCSS**.

---

## 🔍 Visão Geral

- **Backend**: Rust + Axum 0.8.3 + Tokio + SQLx (PostgreSQL)
- **Frontend**: Tera templates + HTMX + TailwindCSS
- **Autenticação**: JWT + Argon2
- **PDFs**: Geração de relatórios com `printpdf`
- **Validação**: Crate `validator`
- **Integração externa**: Twilio API custom (sem crate)
- **Arquitetura**: Modular, orientada a domínio, com testes unitários
- **Execução**: Bins auxiliares `dev`, `build`, `test`

---

## 📁 Estrutura de Pastas

```bash
psocial2/
├── Cargo.toml
├── infra.md
├── arquitetura.md
├── README.md
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── state.rs
│   ├── utils.rs
│   ├── errors.rs
│   ├── db/
│   ├── handlers/
│   ├── models/
│   ├── routes/
│   └── bin/
│       ├── dev.rs
│       ├── build.rs
│       └── test.rs
```

---

## 🚀 Comandos Rápidos

| Alias       | Descrição                                       |
|-------------|--------------------------------------------------|
| `cargo devx`| Roda o servidor com `cargo-watch` em modo DEV   |
| `cargo buildx`| Compila em modo offline (sem internet)        |
| `cargo testx`| Executa todos os testes unitários             |

---

## ⚙️ Tecnologias

- **Axum**: Web framework moderno baseado em `tower`
- **Tokio**: Runtime assíncrono
- **SQLx**: Execução de queries SQL seguras e assíncronas
- **Tera**: Templates HTML rápidos e seguros
- **HTMX**: Interatividade sem SPA ou JavaScript manual
- **TailwindCSS**: Estilo moderno e responsivo com utilitários CSS
- **PrintPDF**: Geração de relatórios e laudos
- **jsonwebtoken**: Cria e valida tokens JWT
- **argon2**: Hash seguro de senhas

---

## 🎯 Roadmap (Resumo)

- [x] Setup inicial com Axum 0.8.3 e SQLx
- [x] Integração com PostgreSQL via pool
- [x] Autenticação JWT e roles (Admin, Psicólogo, Gestor)
- [x] Frontend com HTMX e Tera
- [x] Geração de PDFs (laudos, relatórios)
- [x] Integração com Twilio para envio/recebimento de mensagens
- [ ] Painel administrativo com filtros dinâmicos
- [ ] Relatórios públicos acessíveis por link seguro

---

## 📄 Licença

Este projeto é distribuído sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

---

Se quiser contribuir ou sugerir melhorias, fique à vontade para abrir uma issue ou pull request ✨


