# Phase 1, Day 4 — Timeouts, Signals, and Failure Modes

> **Time budget:** 4-6 hours of focused work.
> **Prerequisite:** Day 3 complete. You can spawn subprocesses with `Command`,
> capture stdout/stderr, handle exit codes, and explain shell injection prevention.
> **Outcome:** You can enforce time limits on subprocesses, kill hanging tools,
> handle every failure mode (not found, timeout, permission denied, non-zero exit),
> and intentionally break your own code to prove it handles edge cases.

> [!IMPORTANT]
> A security tool that hangs is worse than one that crashes.
> A crash gives you an error. A hang gives you silence.
> Your operator stares at a frozen terminal, not knowing if nmap is scanning
> or stuck behind a firewall that drops packets.
> Today you solve this: every subprocess gets a time limit.
> If it exceeds the limit, it dies.

---

## Why This Day Matters

Day 3 gave you `run_tool` — spawn, capture, check. But it has a fatal flaw:
`.output()` **blocks forever** until the child process exits.

What happens when nmap scans a firewalled host that drops all packets?
nmap retries. And retries. And retries.
Your `run_tool` sits there waiting. The entire Achilles DAG stalls.
No error. No timeout. Just silence.

In a real engagement, you might have 50 tools queued behind that one nmap scan.
All of them wait. Your overnight recon grinds to a halt because one target
had a packet-dropping firewall.

The fix: **timeouts**. Spawn the process, start a clock. If the clock runs out
before the process finishes, kill it. Report it as a timeout error.
The DAG moves on.

---

## Part 1: `.spawn()` — Why `.output()` Can't Do Timeouts

### The Problem with `.output()`

```rust
let output = Command::new("nmap")
    .arg("-sV").arg(target)
    .stdin(Stdio::null())
    .output();    // blocks until nmap exits — could be 5 seconds or 5 hours
```

`.output()` does three things in one call:
1. Spawns the process
2. Waits for it to finish
3. Collects stdout/stderr

Steps 1-3 are fused together. You can't interrupt step 2.
There's no way to say "wait, but only for 10 seconds."

### The Solution: `.spawn()` Separates Spawn from Wait

```rust
let mut child: Child = Command::new("nmap")
    .arg("-sV").arg(target)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;
// Process is now running in the background.
// Your Rust code continues executing.
// child is a HANDLE to the running process.
```

Now spawn and wait are separate:
- `.spawn()` starts the process and returns immediately
- You decide when and how long to wait
- If you're tired of waiting, you call `child.kill()`

### The `Child` Struct

`.spawn()` returns `std::process::Child`. Key methods:

```
child.wait()              -> Result<ExitStatus>
    Blocks until process exits. Returns exit status.

child.wait_with_output()  -> Result<Output>
    Blocks until process exits. Returns Output (status + stdout + stderr).
    Consumes the Child handle.

child.kill()              -> Result<()>
    Sends SIGKILL to the process. Instant death. No cleanup.

child.id()                -> u32
    Returns the OS process ID (PID).
```

### Spawn + Wait = Same as Output

```rust
// These two are equivalent:
let output = Command::new("echo").arg("hi").output()?;

let child = Command::new("echo").arg("hi").spawn()?;
let output = child.wait_with_output()?;
```

The difference: with `.spawn()`, you have the `Child` handle between
spawn and wait. That's where you insert the timeout logic.

---

## Part 2: The Timeout Pattern — Thread + Kill

The idea:
1. Spawn the child process
2. Spawn a watchdog thread that sleeps for N seconds
3. Race: does the child finish before the watchdog wakes up?
4. If the watchdog wakes up first, it kills the child

### `std::time::Duration`

```rust
use std::time::Duration;

let timeout = Duration::from_secs(10);     // 10 seconds
let timeout = Duration::from_millis(500);  // 500 milliseconds
let timeout = Duration::from_secs(0);      // zero — instant timeout
```

Duration is just a time measurement. Nothing special.

### `std::thread::spawn`

```rust
use std::thread;

let handle = thread::spawn(|| {
    // This code runs in a NEW OS thread, in parallel
    println!("Hello from another thread!");
});

handle.join().unwrap();  // Wait for the thread to finish
```

`thread::spawn` takes a closure (the `|| { ... }` part) and runs it
on a separate OS thread. The main thread continues executing.
`.join()` blocks until the spawned thread finishes.

### `std::thread::sleep`

```rust
thread::sleep(Duration::from_secs(5));
// This thread does nothing for 5 seconds, then continues
```

### Sharing Data Between Threads

Here's the problem: the watchdog thread needs access to the `Child`
handle to kill it. But `Child` is owned by the main thread.

Two approaches:

**Approach A: Share the Child's PID**

The child's PID is just a `u32` — a number. Copy it to the watchdog thread.
The watchdog uses OS-level kill via `nix::sys::signal::kill()` or
`unsafe { libc::kill(pid, SIGKILL) }`.

This is more complex than you need right now.

**Approach B: Use `child.try_wait()` in a loop**

Instead of a separate watchdog thread, poll the child in a loop:

```
let start = Instant::now();
loop {
    match child.try_wait() {
        Ok(Some(status)) => {
            // Process finished!
            break;
        }
        Ok(None) => {
            // Still running — check timeout
            if start.elapsed() > timeout {
                child.kill()?;
                child.wait()?;  // reap the zombie — collect exit status so OS removes the entry
                // Timeout!
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        Err(e) => {
            // OS error checking process status
            break;
        }
    }
}
```

**Approach C: Use `mpsc` channel (cleanest for Day 4)**

```
let (sender, receiver) = mpsc::channel();

// Thread 1: wait for child, send result when done
let wait_thread = thread::spawn(move || {
    let output = child.wait_with_output();
    let _ = sender.send(output);
});

// Main thread: wait for result OR timeout
match receiver.recv_timeout(timeout) {
    Ok(output) => { /* process finished in time */ }
    Err(mpsc::RecvTimeoutError::Timeout) => { /* timed out! */ }
}
```

### `child.try_wait()` — Non-Blocking Check

```rust
match child.try_wait()? {
    Some(status) => println!("exited with: {}", status),
    None => println!("still running"),
}
```

Unlike `child.wait()` which blocks, `try_wait()` checks immediately
and returns `None` if the process is still alive. This is what
makes the polling loop possible.

### `std::time::Instant` — Measuring Elapsed Time

```rust
use std::time::Instant;

let start = Instant::now();

// ... do some work ...

let elapsed = start.elapsed();  // Duration since `start`
println!("Took {:?}", elapsed); // e.g., "Took 2.35s"
```

`Instant::now()` captures the current moment.
`.elapsed()` returns how much time has passed since that moment.

---

## Part 3: SIGKILL vs SIGTERM — How Processes Die

### On Unix, there are two main ways to kill a process:

```
SIGTERM (signal 15)
    "Please shut down gracefully."
    The process CAN catch this signal and do cleanup
    (flush buffers, close files, save state).
    The process can also IGNORE it.

SIGKILL (signal 9)
    "Die. Now."
    The process CANNOT catch or ignore this signal.
    The OS kernel terminates the process immediately.
    No cleanup. No finalizers. No last words.
```

### What Rust's `child.kill()` sends:

On Unix: `child.kill()` sends **SIGKILL** — immediate, uncatchable death.
There's no built-in way to send SIGTERM from `std::process::Child`.

### The Achilles Production Pattern (Day 16):

In the real Achilles engine, the shutdown sequence is:

```
1. Send SIGTERM       → "please stop"
2. Wait 3 seconds     → give the tool time to flush output
3. Send SIGKILL       → "you had your chance"
```

This matters because some tools (like sqlmap) write results to disk
on exit. SIGKILL prevents that write. SIGTERM lets it finish.

For Day 4, using `child.kill()` (SIGKILL) is fine.
You'll implement the graceful SIGTERM -> SIGKILL sequence on Day 16
with `tokio::process::Child` which has `child.start_kill()`.

---

## Part 4: Permission Errors — The Third Failure Mode

Day 3 handled two failure modes:
1. Binary not found (`.output()` returns `Err` with `ErrorKind::NotFound`)
2. Non-zero exit code (binary ran but reported failure)

Day 4 adds the third: **permission denied**.

This shows up in two ways:

### Case A: OS prevents execution

The binary exists but you don't have permission to execute it.

```rust
let result = Command::new("/root/secret_tool").output();
// Err(io::Error { kind: PermissionDenied, ... })
```

`.output()` (or `.spawn()`) returns `Err` with `ErrorKind::PermissionDenied`.

### Case B: Tool runs but needs elevated privileges

The binary starts fine, but the operation it performs requires root.

```bash
$ nmap -sS 10.10.10.100
# Exits with code 1
# stderr: "You requested a scan type which requires root privileges"
```

Here `.spawn()` succeeds. The process runs. But it exits non-zero with
an error message in stderr. This is NOT an `io::Error` — it's a
normal non-zero exit that you parse from stderr.

### Handling Both in `run_tool`

```rust
Err(e) => {
    if e.kind() == ErrorKind::NotFound {
        Err(ToolError::NotFound(e.to_string()))
    } else if e.kind() == ErrorKind::PermissionDenied {
        Err(ToolError::PermissionDenied(e.to_string()))
    } else {
        Err(ToolError::ExecutionFailed(e.to_string()))
    }
}
```

For Case B, you'd check stderr contents in the `NonZeroExit` handler.
But that's application-level logic — Day 4 just needs you to handle
both `ErrorKind` variants.

---

## Part 5: Exercises

> [!IMPORTANT]
> These exercises are task-based.
> I describe what to build and what the output should look like.
> You write the code yourself.
> If you're stuck, ask -- I'll hint, not solve.

### Exercise 1 -- Spawn and wait manually

Create `~/Antigravity/Achilles/src/bin/day4_spawn.rs`.

**Task:** Rewrite your Day 3 `echo` command using `.spawn()` instead of `.output()`.

1. Use `Command::new("echo").arg("hello from spawn")`
2. Set `.stdin(Stdio::null())`, `.stdout(Stdio::piped())`, `.stderr(Stdio::piped())`
3. Call `.spawn()` to get a `Child`
4. Print the child's PID using `child.id()`
5. Call `child.wait_with_output()` to get the `Output`
6. Print stdout, stderr, and exit code

**Expected output:**
```
=== Spawn and Wait ===

Child PID: 12345
stdout: hello from spawn
stderr: (empty)
Exit code: 0
```

**Hints:**
- `use std::process::{Command, Stdio};`
- `.spawn()` returns `Result<Child, io::Error>`
- `child.wait_with_output()` returns `Result<Output, io::Error>`
- This should feel almost identical to `.output()` -- because it IS.
  The only difference is you see the PID in between.

**Run:** `cargo run --bin day4_spawn`

---

### Exercise 2 -- Timeout with polling loop

Create `~/Antigravity/Achilles/src/bin/day4_timeout.rs`.

**Task:** Write a function with this signature:

```rust
fn run_with_timeout(
    binary: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<ToolOutput, ToolError>
```

Where `ToolError` now has a new variant:

```rust
#[derive(Debug)]
enum ToolError {
    NotFound(String),
    PermissionDenied(String),
    ExecutionFailed(String),
    NonZeroExit { exit_code: i32, stderr: String },
    Timeout { elapsed: Duration },
}
```

**Requirements:**
1. Use `.spawn()` to start the process
2. Use `Instant::now()` to track start time
3. Use a polling loop with `child.try_wait()`:
   - If `try_wait()` returns `Some(status)` -> process finished, collect output
   - If `try_wait()` returns `None` -> check elapsed time
   - If elapsed > timeout -> call `child.kill()`, then `child.wait()`, return `ToolError::Timeout`
   - Sleep 100ms between polls: `thread::sleep(Duration::from_millis(100))`
4. Handle `ErrorKind::NotFound` and `ErrorKind::PermissionDenied` from `.spawn()`
5. On success, collect stdout/stderr from the child's pipes

**Collecting output after try_wait succeeds:**

When `try_wait()` returns `Some(status)`, the process has exited but you
still need to read stdout and stderr from the pipes. Use:

```rust
let mut stdout_buf = String::new();
if let Some(mut out) = child.stdout.take() {
    use std::io::Read;
    out.read_to_string(&mut stdout_buf)?;
}
```

Same pattern for stderr. `child.stdout` is `Option<ChildStdout>`.
`.take()` removes it from the Child (gives you ownership).
Then read it into a String.

**In `main()`, test with:**

```
PART A: run_with_timeout("echo", &["hello"], Duration::from_secs(5))
        -> should succeed quickly

PART B: run_with_timeout("sleep", &["10"], Duration::from_secs(2))
        -> should timeout after ~2 seconds
        -> "sleep 10" runs for 10 seconds, but your timeout is 2

PART C: run_with_timeout("fake_binary", &[], Duration::from_secs(5))
        -> should return NotFound

PART D: run_with_timeout("ls", &["/root"], Duration::from_secs(5))
        -> may return success or NonZeroExit depending on permissions
```

**Expected output pattern:**
```
=== Subprocess Runner with Timeout ===

--- Part A: echo (5s timeout) ---
Success (exit 0) in 5ms
stdout: hello

--- Part B: sleep 10 (2s timeout) ---
TIMEOUT after 2.01s

--- Part C: fake binary ---
NotFound: No such file or directory (os error 2)

--- Part D: ls /root ---
Success (exit 0) OR NonZeroExit depending on your system
```

**Run:** `cargo run --bin day4_timeout`

---

### Exercise 3 -- Upgrade `run_tool` with timeout support

Create `~/Antigravity/Achilles/src/bin/day4_runner_v2.rs`.

**Task:** Combine Day 3's `run_tool` with Day 4's timeout into a single,
production-quality function:

```rust
fn run_tool(
    binary: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<ToolOutput, ToolError>
```

**New: `ToolOutput` includes duration**

```rust
#[derive(Debug)]
struct ToolOutput {
    stdout: String,
    stderr: String,
    exit_code: i32,
    duration: Duration,
}
```

**Requirements:**
1. Everything from Exercise 2
2. Track and return the actual execution duration in `ToolOutput`
3. Test with real tools:
   - `run_tool("uname", &["-a"], Duration::from_secs(5))`
   - `run_tool("sleep", &["30"], Duration::from_secs(3))`
   - `run_tool("id", &[], Duration::from_secs(5))`
   - `run_tool("nmap", &["--version"], Duration::from_secs(5))` (if installed)

**Expected output pattern:**
```
=== Runner v2 (with timeout) ===

--- uname -a (5s timeout) ---
Success (exit 0) in 12ms
stdout: Linux HellFire-Phoenix ...

--- sleep 30 (3s timeout) ---
TIMEOUT after 3.01s

--- id (5s timeout) ---
Success (exit 0) in 8ms
stdout: uid=1000(phantom) ...
```

**Run:** `cargo run --bin day4_runner_v2`

---

### Exercise 4 -- Deliberate break: Make everything fail

Create `~/Antigravity/Achilles/src/bin/day4_break.rs`.

**Task:** Using your `run_tool` from Exercise 3, intentionally trigger
every failure mode and document what happens.

**Tests to run:**

```
1. Empty string as binary:
   run_tool("", &[], Duration::from_secs(5))
   -> What error do you get?

2. Empty string as argument:
   run_tool("echo", &[""], Duration::from_secs(5))
   -> Does it crash or print an empty line?

3. Shell injection attempt:
   run_tool("echo", &["127.0.0.1; echo HACKED"], Duration::from_secs(5))
   -> Prove it prints the literal string, not "HACKED" separately

4. Zero timeout:
   run_tool("echo", &["hello"], Duration::from_secs(0))
   -> Does it panic? Return timeout? Or succeed because echo is instant?

5. Very long timeout:
   run_tool("echo", &["hello"], Duration::from_secs(3600))
   -> Should succeed normally (don't actually wait an hour)

6. Binary exists but wrong args:
   run_tool("ls", &["--invalid-flag-xyz"], Duration::from_secs(5))
   -> NonZeroExit with stderr showing the error message

7. Nonexistent binary:
   run_tool("this_does_not_exist_xyz", &[], Duration::from_secs(5))
   -> NotFound

8. Ctrl+C mid-execution (MANUAL TEST — not in code):
   Run: cargo run --bin day4_runner_v2
   But change the sleep test to Duration::from_secs(30) so it waits long.
   While it's waiting, press Ctrl+C in your terminal.
   -> Does your program exit cleanly?
   -> Does the child process (sleep) also die, or does it keep running?
   -> Check with: ps aux | grep sleep
   -> Write a comment about what you observed.
```

**For each test, write a comment in your code explaining:**
- What you expected
- What actually happened
- Why it makes sense

**Run:** `cargo run --bin day4_break`

---

## Part 6: Key Concepts to Internalize

**1. Why can't `.output()` support timeouts?**
`.output()` fuses spawn + wait + collect into one blocking call.
You can't interrupt it. You need `.spawn()` to get a `Child` handle
that you can kill independently.

**2. What does `child.try_wait()` return?**
`Ok(Some(status))` = process exited.
`Ok(None)` = process still running.
`Err(e)` = OS error checking status.

**3. What does `child.kill()` send on Unix?**
SIGKILL (signal 9). Immediate, uncatchable.
The process gets no chance to clean up.

**4. What's the difference between SIGTERM and SIGKILL?**
SIGTERM = "please stop" -- process can catch it and do cleanup.
SIGKILL = "die now" -- process cannot catch or ignore it.

**5. Why does the timeout loop sleep 100ms between polls?**
Without sleep, the loop would spin at millions of iterations per second,
wasting CPU. 100ms is a balance: responsive enough to catch the timeout
within 100ms of the deadline, efficient enough to not burn CPU.

**6. What is `Instant::now()` vs `Duration`?**
`Instant` = a point in time (like a stopwatch start).
`Duration` = a length of time (like "5 seconds").
`instant.elapsed()` returns a `Duration`.

**7. What are the four failure modes of a subprocess?**
1. Not found -- binary doesn't exist on the system.
2. Permission denied -- binary exists but can't be executed.
3. Non-zero exit -- binary ran but reported failure.
4. Timeout -- binary ran too long and was killed.

**8. Why does Achilles use SIGTERM then SIGKILL in production?**
SIGTERM gives tools time to flush output (write results to disk).
SIGKILL is the fallback if the tool ignores SIGTERM.
3-second grace period between them.

**9. What is `child.stdout.take()`?**
`.take()` removes the `Option<ChildStdout>` from the Child struct,
giving you ownership. Returns `Some(stdout_pipe)` the first time,
`None` if already taken. You need ownership to read the pipe.

**10. Why poll instead of using a watchdog thread?**
The polling loop is simpler to understand and doesn't require
sharing the `Child` handle across threads (which involves
`Arc<Mutex<...>>` -- concepts you haven't learned yet).
The async version (Day 10) uses `tokio::time::timeout` which
is cleaner than both approaches.

---

## Completion Checklist

- [X] Read and understand Parts 1-4
      (spawn, timeout pattern, signals, permission errors)
- [X] Exercise 1 -- Spawn and wait with `.spawn()` (`day4_spawn.rs`)
- [X] Exercise 2 -- Timeout with polling loop (`day4_timeout.rs`)
- [X] Exercise 3 -- Runner v2 with timeout + duration (`day4_runner_v2.rs`)
- [X] Exercise 4 -- Deliberate break: trigger every failure mode (`day4_break.rs`)
- [X] Can explain why `.output()` can't do timeouts
- [X] Can explain `try_wait()` vs `wait()` vs `wait_with_output()`
- [X] Can explain the difference between SIGTERM and SIGKILL
- [X] Can list all four subprocess failure modes

---

## Lesson

> A hanging tool is invisible.
> A crashed tool screams.
> The timeout is not optional -- it is the difference between
> "the scan failed, moving to the next target"
> and "the operator wakes up to find Achilles frozen at 3 AM
> because one firewall was dropping packets."
> Every subprocess in Achilles gets a time budget.
> Exceed it, and you die.

---

**Next: Day 5 -- Generics, Traits, and the `Executable` Trait ->**
