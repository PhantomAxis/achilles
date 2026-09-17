// building a simple tool runner trait system

#![allow(dead_code)]

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
    timeout: Duration,
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
        timeout: Duration::from_secs(3),
    };
    let ls_ok = LsRunner {
        path: "/tmp".to_string(),
        timeout: Duration::from_secs(5),
    };
    let ls_bad = LsRunner {
        path: "/root".to_string(),
        timeout: Duration::from_secs(5),
    };

    run_all(vec![Box::new(echo), Box::new(ls_ok), Box::new(ls_bad)]);
}

fn run_and_report(tool: &dyn Executable) {
    println!("\n=== {} ===\n", tool.name());
    println!("Description: {}", tool.description());
    match tool.run() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    println!();
}

fn run_all(tools: Vec<Box<dyn Executable>>) {
    for tool in &tools {
        run_and_report(tool.as_ref());
    }
}
