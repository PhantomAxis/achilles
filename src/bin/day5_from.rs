// Eliminates manual error conversion boilerplate

use std::io::{Error, ErrorKind};
use std::process::{Child, Command, Stdio};

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

fn spawn_tool(bin: &str) -> Result<Child, ToolError> {
    let child = Command::new(bin)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    Ok(child)
}

fn main() {
    println!("\n=== From<io::Error> for ToolError ===\n");
    print_result("echo", spawn_tool("echo"));
    print_result("fake_binary", spawn_tool("fake_binary"));
}

fn print_result(cmd: &str, output: Result<Child, ToolError>) {
    match output {
        Ok(child) => println!("{}: spwaned successfully (PID: {})", cmd, child.id()),
        Err(e) => match e {
            ToolError::NotFound(e) => println!("{}: NotFound: {}", cmd, e),
            ToolError::PermissionDenied(e) => println!("{}: PermissionDenied: {}", cmd, e),
            ToolError::ExecutionFailed(e) => println!("{}: ExecutionFailed: {}", cmd, e),
        },
    }
}
