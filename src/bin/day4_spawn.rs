// Testing the .spawn() feature

use std::process::{Command, Stdio};

fn main() {
    let child = Command::new("echo")
        .arg("hello from spawn")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Process failed to spawn");

    println!("=== Spawn and Wait ===");
    println!("Child PID: {}", child.id());

    let output = child.wait_with_output();
    if let Ok(res) = output {
        println!("stdout: {}", String::from_utf8_lossy(&res.stdout));
        let stderr = String::from_utf8_lossy(&res.stderr);
        if stderr.is_empty() {
            println!("stderr: (empty)");
        } else {
            println!("stderr: {}", stderr);
        }
        if let Some(code) = res.status.code() {
            println!("Exit code: {}", code);
        }
    }
}
