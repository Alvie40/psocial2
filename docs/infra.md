### 📁 `infra.md` – Infraestrutura e Comandos Customizados do Projeto `psocial2`

Este documento descreve os comandos auxiliares criados para facilitar o desenvolvimento, testes e build do projeto `psocial2`. Todos eles funcionam por meio de **binários personalizados registrados no `Cargo.toml`** e **aliases automáticos**.

---

## 📦 Comandos Customizados via Cargo

### ▶️ `cargo devx`
**Modo de desenvolvimento com recarregamento automático**

Este comando utiliza [`cargo-watch`](https://crates.io/crates/cargo-watch) para **monitorar o código** e **recompilar automaticamente** sempre que houver uma alteração nos arquivos do projeto.

#### O que ele faz:
- Observa arquivos `.rs`, `Cargo.toml`, etc.
- Roda `cargo run` automaticamente ao detectar mudanças.

#### Fonte:
```rust
// src/bin/dev.rs
use std::process::Command;

fn main() {
    println!("🚀 Iniciando modo DEV (cargo-watch)");
    let status = Command::new("cargo")
        .args(["watch", "-x", "run"])
        .status()
        .expect("Falha ao iniciar cargo-watch");
    std::process::exit(status.code().unwrap_or(1));
}
```

---

### 🧱 `cargo buildx`
**Build offline com fallback automático para online**

Tenta compilar o projeto com `--offline`. Se falhar (ex: dependências ausentes), realiza o build normal online.

#### Ideal para:
- Garantir que o projeto é autocontido.
- Otimizar builds sem internet.

#### Fonte:
```rust
// src/bin/build.rs
use std::process::Command;

fn main() {
    println!("🧱 Tentando build OFFLINE...");
    let offline_status = Command::new("cargo")
        .args(["build", "--offline"])
        .status()
        .expect("Falha ao tentar build --offline");

    if !offline_status.success() {
        println!("🔄 Fallback para build online...");
        let online_status = Command::new("cargo")
            .arg("build")
            .status()
            .expect("Falha ao tentar build online");

        std::process::exit(online_status.code().unwrap_or(1));
    }

    std::process::exit(0);
}
```

---

### 🧪 `cargo testx`
**Executa todos os testes unitários**

Executa os testes do projeto com `cargo test`, exibindo os resultados.

#### Ideal para:
- Validar funcionalidades rapidamente.
- Rodar testes locais antes de subir código.

#### Fonte:
```rust
// src/bin/test.rs
use std::process::Command;

fn main() {
    println!("🧪 Executando testes...");
    let status = Command::new("cargo")
        .arg("test")
        .status()
        .expect("Falha ao executar os testes");
    std::process::exit(status.code().unwrap_or(1));
}
```

---

## ⚙️ Configuração no `Cargo.toml`

```toml
[[bin]]
name = "dev"
path = "src/bin/dev.rs"

[[bin]]
name = "buildx"
path = "src/bin/build.rs"

[[bin]]
name = "testx"
path = "src/bin/test.rs"

[alias]
devx = "run --bin dev"
buildx = "run --bin buildx"
testx = "run --bin testx"
```

---

## 📌 Requisitos

Para utilizar o `cargo devx`, é necessário instalar o `cargo-watch`:

```bash
cargo install cargo-watch
```

---

## ✅ Benefícios desta organização

- Rápido acesso com comandos intuitivos.
- Integração nativa com o ecossistema do Cargo.
- Facilita desenvolvimento, testes e builds offline.
- Evita scripts externos ou Makefiles.