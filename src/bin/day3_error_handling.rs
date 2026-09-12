// Handling the command failure gracefully

use std::process::{Command, Stdio};

fn print_details(command: &str, args: Option<&str>) {
    let mut cmd = Command::new(command);
    if let Some(a) = args {
        cmd.arg(a);
    }
    cmd.stdin(Stdio::null());

    let result = cmd.output();

    match result {
        Ok(output) => {
            if let Some(code) = output.status.code() {
                println!("Exit code: {}", code);
            }
            println!("Success: {}", output.status.success());
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}\n\n", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("=== Command Success ===");
    print_details("ls", Some("/tmp"));

    println!("=== Command Failure (non-zero exit) ===");
    print_details("ls", Some("/nonexistent_directory_xyz"));

    println!("=== Command Not Found ===");
    print_details("this_binary_does_not_exist_12345", None);
}
