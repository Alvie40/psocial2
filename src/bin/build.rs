use std::process::Command;

fn main() {
    println!("🔧 Build manual iniciado...");

    let online = Command::new("ping")
        .args(["-c", "1", "crates.io"])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    let mut cmd = Command::new("cargo");

    if online {
        cmd.arg("build");
        println!("🌐 Online: compilando normalmente.");
    } else {
        cmd.args(["build", "--offline"]);
        println!("🔌 Offline: compilando no modo offline.");
    }

    cmd.status()
        .expect("Erro ao executar o build");
}
