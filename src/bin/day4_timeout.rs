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

fn run_with_timeout(
    binary: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<ToolOutput, ToolError> {
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
                let exit_code = match status.code() {
                    Some(code) => code,
                    None => -1,
                };

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
                    return Err(ToolError::NonZeroExit { exit_code, stderr });
                }

                let res = ToolOutput {
                    exit_code,
                    stdout,
                    stderr,
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
    println!("=== Subprocess Runner with Timeout ===\n");

    println!("--- Part A: echo (5s timeout) ---");
    if let Ok(out) = run_with_timeout("echo", &["hello"], Duration::from_secs(5)) {
        println!("Success (exit {})", out.exit_code);
        println!("stdout: {}", out.stdout);
    }

    println!("--- Part B: sleep 10s (2s timeout) ---");
    if let Err(e) = run_with_timeout("sleep", &["10"], Duration::from_secs(2)) {
        if let ToolError::Timeout { elapsed } = e {
            println!("TIMEOUT after {:?}\n", elapsed);
        }
    }

    println!("--- Part C: fake binary ---");
    if let Err(ToolError::NotFound(e)) =
        run_with_timeout("fake_binary", &[], Duration::from_secs(5))
    {
        println!("NotFound: {}\n", e);
    }

    println!("--- Part D: ls /root ---");
    if let Err(ToolError::NonZeroExit { exit_code, stderr }) =
        run_with_timeout("ls", &["/root"], Duration::from_secs(5))
    {
        println!("Exit code: {}\nstderr: {}", exit_code, stderr);
    }
}
