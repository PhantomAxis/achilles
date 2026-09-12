// Proof that Achilles's subprocess model is immune to shell injection

use std::io::ErrorKind;
use std::process::{Command, Stdio};

#[derive(Debug)]
struct ToolOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[derive(Debug)]
enum ToolError {
    NotFound(String),        // Binary doesn't exist
    ExecutionFailed(String), // Other OS errors (permissions, etc.)
    NonZeroExit { exit_code: i32, stderr: String },
}

fn run_tool(binary: &str, args: &[&str]) -> Result<ToolOutput, ToolError> {
    let mut cmd = Command::new(binary);
    cmd.args(args);
    cmd.stdin(Stdio::null());

    let output = cmd.output();

    match output {
        Ok(out) => {
            let exit_code = out.status.code().unwrap_or(-1);
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();

            if exit_code != 0 {
                return Err(ToolError::NonZeroExit { exit_code, stderr });
            }

            let result = ToolOutput {
                stdout,
                stderr,
                exit_code,
            };

            Ok(result)
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(ToolError::NotFound(format!("{}", e)))
            } else {
                Err(ToolError::ExecutionFailed(format!("{}", e)))
            }
        }
    }
}

fn main() {
    println!("=== Shell Injection Proof ===");

    let malicious_target = String::from("127.0.0.1; echo HACKED");
    println!("\n--- Direct (safe) ---");

    let safe_result = run_tool("echo", &[&malicious_target]);
    if let Ok(res) = safe_result {
        println!("stdout: {}", res.stdout);
        println!("✅ Metacharacters are literal text - no injection");
    }

    println!("\n--- Shell (dangerous) ---");

    let malicious_result = run_tool("sh", &["-c", &format!("echo {}", malicious_target)]);
    if let Ok(res) = malicious_result {
        println!("stdout: {}", res.stdout);
        println!("Shell interpreted ';' - attacker payload executed!");
    }

    println!("\n--- Conclusion ---");
    println!("Achilles uses direct execvp. Shell metacharacters are never interpreted.");
}
