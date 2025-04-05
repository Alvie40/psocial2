use std::process::Command;

fn main() {
    println!("🚀 Iniciando modo DEV (cargo-watch)");

    let online = Command::new("ping")
        .args(["-c", "1", "crates.io"])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    let args = if online {
        vec!["watch", "-x", "build"]
    } else {
        vec!["watch", "-x", "build --offline"]
    };

    Command::new("cargo")
        .args(&args)
        .spawn()
        .expect("Erro ao rodar cargo watch")
        .wait()
        .unwrap();
}
