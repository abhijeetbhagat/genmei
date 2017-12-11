use std::process;
use std::process::{Command, Stdio};

fn main() {
    let command = Command::new("cargo")
        .arg("fmt")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute cargo fmt");

    process::exit(command.status.code().unwrap_or(0));
}
