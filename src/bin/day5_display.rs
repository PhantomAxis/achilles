// Implementing the display trait for ToolResult and ToolError

use std::fmt;
use std::time::Duration;

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

impl fmt::Display for ToolResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToolResult::Success { stdout, duration } => write!(
                f,
                "Success in {:.2}s - stdout: {}",
                duration.as_secs_f64(),
                stdout
            ),
            ToolResult::Failure { stderr, exit_code } => {
                write!(f, "Failure (exit {}) - stderr: {}", exit_code, stderr)
            }
            ToolResult::Timeout { elapsed } => {
                write!(f, "Timeout after {:.2}s", elapsed.as_secs_f64())
            }
        }
    }
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToolError::NotFound(e) => write!(f, "NotFound: {}", e),
            ToolError::PermissionDenied(e) => write!(f, "PermissionDenied: {}", e),
            ToolError::ExecutionFailed(e) => write!(f, "ExecutionFailed: {}", e),
        }
    }
}

fn main() {
    let success = ToolResult::Success {
        stdout: "hello".to_string(),
        duration: Duration::from_millis(10),
    };
    let failure = ToolResult::Failure {
        stderr: "permission denied".to_string(),
        exit_code: 2,
    };
    let timeout = ToolResult::Timeout {
        elapsed: Duration::from_secs(10),
    };
    println!("\n=== ToolResult Output: Formatted by Display ===\n");
    println!("{}", success);
    println!("{}", failure);
    println!("{}", timeout);

    let not_found = ToolError::NotFound("No such file or Directory (os error 2)".to_string());
    let permission_denied = ToolError::PermissionDenied("permission denied".to_string());
    let execution_failed = ToolError::ExecutionFailed("Out of Memory".to_string());
    println!("\n=== ToolError Output: Formatted by Display ===\n");
    println!("{}", not_found);
    println!("{}", permission_denied);
    println!("{}", execution_failed);
}
