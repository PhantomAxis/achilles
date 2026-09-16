// Testing the outputs of real tools

use std::io::ErrorKind;
use std::process::{Command, Stdio};

#[allow(dead_code)]
#[derive(Debug)]
struct ToolOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[allow(dead_code)]
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
    println!("=== Real Tool Execution ===");

    print_result("uname -a", run_tool("uname", &["-a"]));
    print_result("whoami", run_tool("whoami", &[]));
    print_result(
        "namp --version (intentionally misspelt)",
        run_tool("namp", &["--version"]),
    );
    print_result("id", run_tool("id", &[]));
}

fn print_result(tool: &str, result: Result<ToolOutput, ToolError>) {
    println!("--- {} ---", tool);
    match result {
        Ok(res) => println!("{}", res.stdout),
        Err(e) => match e {
            ToolError::NotFound(err) => println!("❌ Not installed: {}", err),
            ToolError::ExecutionFailed(err) => println!("{}", err),
            ToolError::NonZeroExit { exit_code, stderr } => {
                println!("exit code: {}\nstderr: {}", exit_code, stderr)
            }
        },
    }
    println!();
}
