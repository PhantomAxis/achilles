// building a simple tool runner trait system

use std::io::{Error, ErrorKind, Read};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

trait Executable {
    fn name(&self) -> &str;
    fn run(&self) -> Result<ToolResult, ToolError>;

    // Default implementation
    fn description(&self) -> String {
        format!("{} runner", self.name())
    }
}

#[derive(Debug)]
enum ToolResult {
    Success { stdout: String, duration: Duration },
    Failure { stderr: String, exit_code: i32 },
    Timeout { elapsed: Duration },
}

#[derive(Debug)]
enum ToolError {
    NotFound(String),
    PermissionDenied(String),
    ExecutionFailed(String),
}

impl From<Error> for ToolError {
    fn from(e: Error) -> Self {
        if e.kind() == ErrorKind::NotFound {
            ToolError::NotFound(e.to_string())
        } else if e.kind() == ErrorKind::PermissionDenied {
            ToolError::PermissionDenied(e.to_string())
        } else {
            ToolError::ExecutionFailed(e.to_string())
        }
    }
}

struct EchoRunner {
    message: String,
}

struct LsRunner {
    path: String,
    timeout: Duration,
}

impl Executable for EchoRunner {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self) -> Result<ToolResult, ToolError> {
        let mut cmd = Command::new("echo");
        cmd.arg(&self.message);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd.spawn()?;
        let start = Instant::now();
        let res = child.wait_with_output()?;
        Ok(ToolResult::Success {
            stdout: String::from_utf8_lossy(&res.stdout).to_string(),
            duration: start.elapsed(),
        })
    }
}

impl Executable for LsRunner {
    fn name(&self) -> &str {
        "ls"
    }

    fn run(&self) -> Result<ToolResult, ToolError> {
        let mut cmd = Command::new("ls");
        cmd.arg(&self.path);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let start = Instant::now();

        loop {
            match child.try_wait() {
                Ok(Some(res)) => {
                    let exit_code = res.code().unwrap_or(-1);

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

                    if exit_code != 0 {
                        return Ok(ToolResult::Failure { stderr, exit_code });
                    }

                    return Ok(ToolResult::Success {
                        stdout,
                        duration: start.elapsed(),
                    });
                }
                Ok(None) => {
                    if start.elapsed() > self.timeout {
                        let _ = child.kill();
                        let _ = child.wait();
                        return Ok(ToolResult::Timeout {
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
}

fn main() {
    let echo = EchoRunner {
        message: "hello from trait".to_string(),
    };
    let ls_ok = LsRunner {
        path: "/tmp".to_string(),
        timeout: Duration::from_secs(5),
    };
    let ls_bad = LsRunner {
        path: "/root".to_string(),
        timeout: Duration::from_secs(5),
    };

    println!("\n=== Trait-Based Tool Runner ===\n");

    execute_tool(&echo);
    execute_tool(&ls_ok);
    execute_tool(&ls_bad);
}

fn execute_tool(tool: &dyn Executable) {
    println!("Running: {}", tool.description());
    match tool.run() {
        Ok(out) => match out {
            ToolResult::Success { stdout, duration } => {
                println!("Success in {:.2}s", duration.as_secs_f64());
                println!("stdout: {}\n", stdout);
            }
            ToolResult::Failure { stderr, exit_code } => {
                println!("Failure (exit {})", exit_code);
                println!("stderr: {}\n", stderr);
            }
            ToolResult::Timeout { elapsed } => {
                println!("TIMEOUT after {:.2}s\n", elapsed.as_secs_f64());
            }
        },
        Err(e) => match e {
            ToolError::NotFound(e) => println!("NotFound: {}\n", e),
            ToolError::PermissionDenied(e) => println!("PermissionDenied: {}\n", e),
            ToolError::ExecutionFailed(e) => println!("ExecutionFailed: {}\n", e),
        },
    }
}
