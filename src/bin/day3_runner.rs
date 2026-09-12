// Reusable tool execution function

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
    println!("=== Achilles Subprocess Runner ===\n");

    println!("--- Part A: echo ---");
    let part_a_result = run_tool("echo", &["hello", "from", "achilles"]);
    if let Ok(res) = part_a_result {
        println!("✅ Success (exit {})", res.exit_code);
        println!("stdout: {}", res.stdout);
    }

    println!("--- Part B: ls nonexistent ---");
    let part_b_result = run_tool("ls", &["/nonexistent_path_xyz"]);
    if let Err(e) = part_b_result {
        match e {
            ToolError::NotFound(x) => println!("{}", x),
            ToolError::ExecutionFailed(x) => println!("{}", x),
            ToolError::NonZeroExit { exit_code, stderr } => {
                println!("❌ NonZeroExit (exit {})", exit_code);
                println!("stderr: {}", stderr);
            }
        }
    }

    println!("--- Part C: binary not found ---");
    let part_c_result = run_tool("fake_binary_xyz", &["--version"]);
    if let Err(e) = part_c_result {
        if let ToolError::NotFound(e) = e {
            println!("❌ NotFound: {}\n", e);
        }
    }

    println!("--- Part D: cat hostname ---");
    let part_d_result = run_tool("cat", &["/etc/hostname"]);
    if let Ok(res) = part_d_result {
        println!("✅ Success (exit {})", res.exit_code);
        println!("stdout: {}", res.stdout);
    }
}
