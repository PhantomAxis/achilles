// This file demonstrates the basic command execution

use std::process::{Command, Stdio};

fn main() {
    println!("=== Basic Command Execution ===");

    let result = Command::new("echo")
        .arg("Achilles subprocess test")
        .stdin(Stdio::null())
        .output()
        .expect("Failed to create a subprocess");

    println!("Command succeeded: {}", result.status.success());
    if let Some(exitcode) = result.status.code() {
        println!("Exit code: {}", exitcode);
    }
    println!("stdout: {}", String::from_utf8_lossy(&result.stdout));

    let stderr = String::from_utf8_lossy(&result.stderr);

    if stderr.is_empty() {
        println!("\nstderr: (empty)",);
    } else {
        println!("\nstderr: {}", stderr);
    }
}
