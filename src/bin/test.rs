use std::process::Command;

fn main() {
    println!("🧪 Rodando testes unitários...");

    let online = Command::new("ping")
        .args(["-c", "1", "crates.io"])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    let mut cmd = Command::new("cargo");

    if online {
        cmd.arg("test");
        println!("🌐 Online: testando normalmente.");
    } else {
        cmd.args(["test", "--offline"]);
        println!("🔌 Offline: testando no modo offline.");
    }

    cmd.status()
        .expect("Erro ao rodar os testes");
}
