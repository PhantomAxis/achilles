// This demonstrates timeout with polling loop

use std::io::{ErrorKind, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
#[allow(dead_code)]
struct ToolOutput {
    exit_code: i32,
    stdout: String,
    stderr: String,
    duration: Duration,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ToolError {
    NotFound(String),
    PermissionDenied(String),
    ExecutionFailed(String),
    NonZeroExit { exit_code: i32, stderr: String },
    Timeout { elapsed: Duration },
}

fn run_tool(binary: &str, args: &[&str], timeout: Duration) -> Result<ToolOutput, ToolError> {
    let mut cmd = Command::new(binary);
    cmd.args(args);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err(ToolError::NotFound(e.to_string()));
            } else if e.kind() == ErrorKind::PermissionDenied {
                return Err(ToolError::PermissionDenied(e.to_string()));
            } else {
                return Err(ToolError::ExecutionFailed(e.to_string()));
            }
        }
    };

    let start = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let exit_code = status.code().unwrap_or(-1);

                let mut stdout_buf = Vec::new();
                if let Some(mut out) = child.stdout.take() {
                    out.read_to_end(&mut stdout_buf).unwrap();
                }
                let stdout = String::from_utf8_lossy(&stdout_buf).to_string();

                let mut stderr_buf = Vec::new();
                if let Some(mut err) = child.stderr.take() {
                    err.read_to_end(&mut stderr_buf).unwrap();
                }
                let stderr = String::from_utf8_lossy(&stderr_buf).to_string();
                let duration = start.elapsed();

                if exit_code != 0 {
                    return Err(ToolError::NonZeroExit { exit_code, stderr });
                }

                let res = ToolOutput {
                    exit_code,
                    stdout,
                    stderr,
                    duration,
                };

                return Ok(res);
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    child.kill().unwrap();
                    child.wait().unwrap();

                    return Err(ToolError::Timeout {
                        elapsed: start.elapsed(),
                    });
                }

                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(ToolError::ExecutionFailed(e.to_string()));
            }
        }
    }
}

fn main() {
    println!("\n=== Runner v2 (with timeout) ===\n");
    print_output(
        run_tool("uname", &["-a"], Duration::from_secs(5)),
        "uname -a",
        5,
    );
    print_output(
        run_tool("sleep", &["30"], Duration::from_secs(30)),
        "sleep 30",
        3,
    );
    print_output(run_tool("id", &[], Duration::from_secs(5)), "id", 5);
    print_output(
        run_tool("nmap", &["--version"], Duration::from_secs(5)),
        "nmap --version",
        5,
    );
}

fn print_output(result: Result<ToolOutput, ToolError>, cmd: &str, duration: i32) {
    println!("\n--- {} ({}s timeout) ---\n", cmd, duration);

    match result {
        Ok(output) => {
            println!(
                "Success (exit {}) in {:.2}s",
                output.exit_code,
                output.duration.as_secs_f64()
            );
            println!("stdout: {}", output.stdout);
        }
        Err(e) => match e {
            ToolError::NotFound(e) => println!("NotFound: {}", e),
            ToolError::PermissionDenied(e) => println!("PermissionDenied: {}", e),
            ToolError::Timeout { elapsed } => {
                println!("TIMEOUT after {:.2}s", elapsed.as_secs_f64())
            }
            ToolError::ExecutionFailed(e) => println!("ExecutionFailed: {}", e),
            ToolError::NonZeroExit { exit_code, stderr } => {
                println!("Exit code: {}\nstderr: {}", exit_code, stderr)
            }
        },
    }
}
