# Phase 1, Day 5 — Generics, Traits, and the Executable Interface

> **Time budget:** 5-7 hours of focused work.
> **Prerequisite:** Day 4 complete. You can spawn subprocesses with timeouts,
> kill hanging processes, handle all four failure modes (not found, permission
> denied, non-zero exit, timeout), and explain SIGTERM vs SIGKILL.
> **Outcome:** You can define traits, implement them for multiple structs,
> use generics with trait bounds, explain the difference between trait objects
> and enums, and build a polymorphic tool runner where the engine doesn't
> care which tool it's executing.

> [!IMPORTANT]
> Achilles runs nmap, nuclei, subfinder, ffuf, sqlmap, httpx, and dozens
> more tools. Each tool has different arguments, different output formats,
> different failure modes.
> But the engine doesn't know any of that.
> The engine only knows: "this thing implements the `Node` trait.
> I call `.execute()`. I get ADC objects back."
> Traits are how you build that wall of abstraction.
> Today you learn to build it.

---

## Why This Day Matters

Look at your Day 4 `run_tool`:

```rust
fn run_tool(binary: &str, args: &[&str], timeout: Duration) -> Result<ToolOutput, ToolError>
```

This works. But it's a raw function. To add a new tool, you pass different
strings to the same function. There's no structure. No type safety on
what tool you're running. No way for the compiler to verify that you
handled the output correctly for nmap vs nuclei.

In the real Achilles engine, every tool is a **struct** that implements
a **trait**. The struct holds configuration (target, flags, timeout).
The trait provides the interface (`.execute()`). The engine doesn't care
what struct it's holding -- it only calls the trait method.

```
Engine
  │
  ├── calls .execute() on NmapRunner     → gets Vec<Host>
  ├── calls .execute() on NucleiRunner   → gets Vec<Finding>
  └── calls .execute() on SubfinderRunner → gets Vec<Host>

Engine doesn't know about nmap or nuclei.
It only knows about the Node trait.
```

This is **polymorphism** -- the same interface, different implementations.
Java has interfaces. Rust has traits. Same idea, more power.

---

## Part 1: What is a Trait?

A trait defines a **contract**: "any type that implements this trait
must have these methods."

### Defining a trait

```rust
trait Greet {
    fn hello(&self) -> String;
}
```

This says: any type that implements `Greet` must have a method
called `hello` that takes `&self` and returns a `String`.

### Implementing a trait for a struct

```rust
struct Human {
    name: String,
}

impl Greet for Human {
    fn hello(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }
}

struct Robot {
    model: String,
}

impl Greet for Robot {
    fn hello(&self) -> String {
        format!("UNIT {} ONLINE", self.model)
    }
}
```

Two structs. Same trait. Different implementations.

### Using the trait

```rust
let h = Human { name: String::from("Phantom") };
let r = Robot { model: String::from("T-800") };

println!("{}", h.hello());  // "Hi, I'm Phantom"
println!("{}", r.hello());  // "UNIT T-800 ONLINE"
```

Both have `.hello()`, but the behavior depends on the type.

### The Java parallel

```
Rust trait       = Java interface
impl Trait for X = class X implements Interface
&self            = this
```

You know interfaces from Java. Traits are the same concept.
The difference: Rust traits can have default implementations,
associated types, and trait bounds. You'll see all of these.

---

## Part 2: Why Traits Matter for Achilles

Without traits, the engine would need to know every tool:

```rust
// BAD: engine knows about specific tools
fn run_node(node_type: &str, target: &str) {
    if node_type == "nmap" {
        // run nmap...
    } else if node_type == "nuclei" {
        // run nuclei...
    } else if node_type == "subfinder" {
        // run subfinder...
    }
    // Adding a new tool = editing this function
    // Forgetting a branch = silent failure
}
```

With traits, the engine is generic:

```rust
// GOOD: engine only knows about the trait
fn run_node(node: &dyn Executable) {
    let result = node.run();
    // handle result...
    // Adding a new tool = implementing the trait for a new struct
    // The engine code never changes
}
```

The engine is **closed for modification, open for extension**.
Adding nuclei doesn't touch the engine. You just write:

```rust
impl Executable for NucleiRunner { ... }
```

The engine picks it up automatically.

---

## Part 3: Generics — Functions That Work on Any Type

### The problem

```rust
fn print_twice_string(value: String) {
    println!("{}", value);
    println!("{}", value);
}

fn print_twice_i32(value: i32) {
    println!("{}", value);
    println!("{}", value);
}
```

Same logic, duplicated for each type.

### The solution: generics

```rust
fn print_twice<T: std::fmt::Display>(value: T) {
    println!("{}", value);
    println!("{}", value);
}
```

`<T: std::fmt::Display>` means: "T can be ANY type, as long as it
implements the `Display` trait." The `Display` bound is needed because
`println!("{}", value)` requires the value to be displayable.

```rust
print_twice(42);           // T = i32
print_twice("hello");      // T = &str
print_twice(String::from("world"));  // T = String
```

One function. Works for any displayable type.

### Reading the syntax

```
fn function_name<T: TraitBound>(param: T) -> ReturnType
   │              │  │             │
   │              │  │             └── param is of type T
   │              │  └── T must implement this trait
   │              └── T is a generic type parameter
   └── function name
```

### Multiple trait bounds

```rust
fn process<T: Display + Debug>(value: T) {
    println!("display: {}", value);    // uses Display
    println!("debug: {:?}", value);    // uses Debug
}
```

`T: Display + Debug` means T must implement BOTH traits.

### Where clause (same thing, cleaner for complex bounds)

```rust
fn process<T>(value: T)
where
    T: Display + Debug + Clone,
{
    let copy = value.clone();
    println!("{}", copy);
}
```

`where` clauses do the same thing as inline bounds.
Use `where` when the bounds get long.

---

## Part 4: Generics on Structs

```rust
struct Wrapper<T> {
    value: T,
}

let w1 = Wrapper { value: 42 };        // Wrapper<i32>
let w2 = Wrapper { value: "hello" };   // Wrapper<&str>
```

The struct works for any type. The compiler generates a separate
version for each concrete type used -- **monomorphization**.
At runtime, `Wrapper<i32>` and `Wrapper<&str>` are completely
separate types. Zero overhead.

### `impl` blocks for generic structs

```rust
impl<T> Wrapper<T> {
    fn get(&self) -> &T {
        &self.value
    }
}
```

`impl<T>` means: "this impl block works for any T."

### Constrained impl blocks

```rust
impl<T: Display> Wrapper<T> {
    fn print(&self) {
        println!("{}", self.value);
    }
}
```

`impl<T: Display>` means: "this impl block only exists when T
implements Display." If T doesn't implement Display, the `.print()`
method doesn't exist at all. Compile-time restriction.

---

## Part 5: Trait Objects vs Enums — The Achilles Design Decision

This is architecturally important. Read carefully.

### Option A: Trait objects (`dyn Trait`)

```rust
trait ADCType {
    fn type_name(&self) -> &str;
}

impl ADCType for Host {
    fn type_name(&self) -> &str { "Host" }
}

impl ADCType for Finding {
    fn type_name(&self) -> &str { "Finding" }
}

// Store different types in one Vec:
let objects: Vec<Box<dyn ADCType>> = vec![
    Box::new(host),
    Box::new(finding),
];
```

`Box<dyn ADCType>` is a **trait object** -- a pointer to any type
that implements `ADCType`. It uses **dynamic dispatch**: the actual
method to call is looked up at runtime through a vtable.

**Problem:** You can't `match` on trait objects. You don't know
the concrete type at compile time. To get the actual `Host` or
`Finding`, you'd need `dyn Any` with `downcast_ref()` -- which is
runtime type-checking. If you forget to handle a type, the compiler
doesn't warn you. It just panics at runtime.

### Option B: Enum (what Achilles uses)

```rust
enum ADCObject {
    Host(Host),
    Port(Port),
    Finding(Finding),
    Credential(Credential),
}
```

Now you can `match`:

```rust
match object {
    ADCObject::Host(h) => println!("Host: {}", h.ip),
    ADCObject::Port(p) => println!("Port: {}", p.number),
    ADCObject::Finding(f) => println!("Finding: {}", f.title),
    ADCObject::Credential(c) => println!("Credential: {}", c.username),
}
```

If you add `ADCObject::DnsRecord(DNSRecord)` later, the compiler
**forces** you to add a match arm everywhere. You can't forget.

**Achilles uses the enum approach** because exhaustive matching
prevents silent bugs. The tradeoff: adding a new ADC type requires
adding an enum variant and updating every match. That's acceptable
when type safety matters more than extensibility.

### When to use each

```
Enums:  closed set of types, you know all variants at compile time.
        Achilles ADC types -- there are exactly 7, they don't change often.

Trait objects: open set of types, plugins can add new ones at runtime.
              Achilles Transformer registry -- community can add new
              transformers without recompiling the engine.
```

---

## Part 6: `impl Trait` in Function Arguments and Return Types

### As a function parameter

```rust
fn run_anything(runner: &impl Executable) -> Result<ToolResult, ToolError> {
    runner.run()
}
```

`&impl Executable` means: "any type that implements Executable."
This is shorthand for:

```rust
fn run_anything<T: Executable>(runner: &T) -> Result<ToolResult, ToolError> {
    runner.run()
}
```

Both are equivalent. The generic version is more explicit.

### As a return type

```rust
fn make_runner() -> impl Executable {
    NmapRunner { target: "10.10.10.1".to_string(), timeout: Duration::from_secs(30) }
}
```

`-> impl Executable` means: "I return something that implements
Executable, but I'm not telling you the concrete type." The caller
can only use methods from the `Executable` trait.

---

## Part 7: `&dyn Trait` vs `impl Trait` — Static vs Dynamic Dispatch

### Static dispatch (`impl Trait` / generics)

```rust
fn run_static(runner: &impl Executable) {
    runner.run();
}
```

The compiler generates a separate copy of `run_static` for each
concrete type. `run_static::<NmapRunner>` and `run_static::<NucleiRunner>`
are different functions in the binary. The method call is resolved
at **compile time**. Zero runtime cost.

### Dynamic dispatch (`&dyn Trait`)

```rust
fn run_dynamic(runner: &dyn Executable) {
    runner.run();
}
```

One function. The concrete type is unknown at compile time.
The method call goes through a **vtable** -- a lookup table of
function pointers. Small runtime cost (pointer indirection).

### When to use each

```
Static (impl Trait / generics):
    - You know the type at compile time
    - Performance-critical code
    - Most of the time

Dynamic (&dyn Trait / Box<dyn Trait>):
    - You need to store DIFFERENT types in the same collection
    - Plugin systems where types aren't known at compile time
    - Vec<Box<dyn Executable>> -- a list of different runners
```

### The Java parallel

```
Java: interface method calls are always virtual (dynamic dispatch)
Rust: you choose. Generics = static. dyn Trait = dynamic.
      You pay for dynamic dispatch only when you use it.
```

---

## Part 8: Default Method Implementations

Traits can provide default behavior:

```rust
trait Executable {
    fn name(&self) -> &str;
    fn run(&self) -> Result<ToolResult, ToolError>;

    // Default implementation -- structs can override or use as-is
    fn description(&self) -> String {
        format!("{} runner", self.name())
    }
}
```

If `NmapRunner` implements `Executable` without defining `description()`,
it gets the default: `"nmap runner"`. If it wants custom behavior,
it overrides:

```rust
impl Executable for NmapRunner {
    fn name(&self) -> &str { "nmap" }
    fn run(&self) -> Result<ToolResult, ToolError> { /* ... */ }

    // Override the default
    fn description(&self) -> String {
        format!("Nmap {} scanner targeting {}", self.scan_type, self.target)
    }
}
```

---

## Part 9: Standard Library Traits You Already Use

You've been using traits without knowing it:

```
Debug       -> #[derive(Debug)]     -> enables {:?} formatting
Display     -> impl Display for X   -> enables {} formatting
Clone       -> #[derive(Clone)]     -> enables .clone()
PartialEq   -> #[derive(PartialEq)] -> enables == comparison
From<T>     -> impl From<X> for Y   -> enables Y::from(x) conversion
```

### `Display` -- implement it yourself

```rust
use std::fmt;

struct Host {
    ip: String,
    hostname: Option<String>,
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.hostname {
            Some(name) => write!(f, "{} ({})", self.ip, name),
            None => write!(f, "{}", self.ip),
        }
    }
}

let h = Host { ip: "10.10.10.1".to_string(), hostname: Some("target.htb".to_string()) };
println!("{}", h);  // "10.10.10.1 (target.htb)"
```

`Display` controls what `{}` prints. `Debug` controls what `{:?}` prints.
You can `#[derive(Debug)]` but you must manually implement `Display`.

### `From<T>` -- type conversion

```rust
impl From<io::Error> for ToolError {
    fn from(e: io::Error) -> Self {
        if e.kind() == ErrorKind::NotFound {
            ToolError::NotFound(e.to_string())
        } else if e.kind() == ErrorKind::PermissionDenied {
            ToolError::PermissionDenied(e.to_string())
        } else {
            ToolError::ExecutionFailed(e.to_string())
        }
    }
}
```

Once `From<io::Error>` is implemented for `ToolError`, you can use `?`
to auto-convert:

```rust
let child = cmd.spawn()?;  // io::Error auto-converts to ToolError
```

Remember in Day 3 and Day 4, you manually matched on `cmd.spawn()`
to convert `io::Error` into `ToolError`? The `From` trait eliminates
that boilerplate. The `?` operator calls `From::from()` automatically.

---

## Part 10: Supertraits

```rust
trait Node: Send + Sync {
    fn execute(&self) -> Result<NodeOutput, NodeError>;
}
```

`Node: Send + Sync` means: "to implement `Node`, you must ALSO
implement `Send` and `Sync`."

- `Send` = this type can be sent to another thread
- `Sync` = this type can be shared between threads via references

Why: Achilles runs nodes on different Tokio threads. If a node isn't
`Send + Sync`, it can't be scheduled across threads. The supertrait
enforces this at compile time -- you can't accidentally create a
node that deadlocks the scheduler.

Most types are `Send + Sync` by default. The compiler auto-derives
them. You only need to worry if you use raw pointers or `Rc` (which
are NOT Send/Sync).

---

## Part 11: Discarding Results — The `let _ =` Pattern

When a function returns `Result`, Rust warns you if you ignore it.
Ignoring errors is usually a bug, so the compiler forces you to
acknowledge the return value.

```rust
child.kill();    // COMPILER WARNING: unused `Result` that must be used
```

You have three choices:

### Option 1: `.unwrap()` — crash if it fails

```rust
child.kill().unwrap();
```

If `kill()` returns `Err`, the program panics. Use this when failure
is genuinely unexpected and should crash. **Don't use this in a timeout
handler** -- the process might already be dead, and killing a dead
process returns an error. You don't want to crash because you
successfully timed something out.

### Option 2: `?` — propagate the error up

```rust
child.kill()?;
```

If `kill()` returns `Err`, the current function immediately returns
that error. Use this when the caller should handle the failure. But
in a timeout handler, you're already handling the situation -- you
don't want to abort the timeout logic because kill failed.

### Option 3: `let _ =` — intentionally discard

```rust
let _ = child.kill();
let _ = child.wait();
```

`_` is the "black hole" pattern. It tells the compiler: "I see
the Result. I'm deliberately throwing it away." No warning. No crash.
No propagation.

### What happens under the hood

```rust
let _ = child.kill();
```

The compiler generates code that:
1. Calls `child.kill()`
2. Gets back a `Result<(), io::Error>`
3. Drops the Result immediately without inspecting it

The `_` binding never actually stores the value. The compiler
optimizes it to: "call the function, ignore the return." It's the
same as calling a void function in C.

### When to use `let _ =`

Use it when **failure is expected and harmless:**

- Killing a process that's already dead -- it's dead, you don't care
- Closing a file that might already be closed
- Sending to a channel where the receiver might have disconnected

Don't use it to silence warnings on errors you should actually handle.
If `spawn()` fails, that matters -- use `?`. If `kill()` fails on a
dead process, that's fine -- use `let _ =`.

---

## Part 12: Cargo Clippy and Cargo Fmt

Starting today, add these to your workflow:

### `cargo clippy`

A linter that teaches idiomatic Rust. It catches:
- `.unwrap()` where `?` belongs
- Unnecessary `.clone()`
- Suboptimal iterator chains
- Missing error handling
- Redundant closures
- And hundreds of other patterns

Run it: `cargo clippy`
Fix every warning. Clippy is your second reviewer.

### `cargo fmt`

Auto-formats your code to the community standard.
No arguments about indentation or brace placement.

Run it: `cargo fmt`

**New rule: run `cargo clippy` and `cargo fmt` before every commit.**
Start now. Fix 2 warnings per commit, not 200 on release day.

---

## Part 13: Exercises

> [!IMPORTANT]
> These exercises are task-based.
> I describe what to build and what the output should look like.
> You write the code yourself.
> If you're stuck, ask -- I'll hint, not solve.

### Exercise 1 -- Define and implement a trait

Create `~/Antigravity/Achilles/src/bin/day5_traits.rs`.

**Task:** Build a simple tool runner trait system.

1. Define this trait:

```rust
trait Executable {
    fn name(&self) -> &str;
    fn run(&self) -> Result<ToolResult, ToolError>;

    // Default implementation
    fn description(&self) -> String {
        format!("{} runner", self.name())
    }
}
```

2. Define `ToolResult` and `ToolError`:

```rust
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
```

3. Create two structs that implement `Executable`:

```rust
struct EchoRunner {
    message: String,
    timeout: Duration,
}

struct LsRunner {
    path: String,
    timeout: Duration,
}
```

Both runners have a `timeout` field. Every tool gets a time limit.
No exceptions. A tool that can hang forever is a liability.

`EchoRunner::run()` should:
- Spawn `echo` with the message, using a timeout (polling loop from Day 4)
- Return `ToolResult::Success` with stdout and duration
- Return `ToolResult::Timeout` if timeout exceeded
- Use `let _ = child.kill()` and `let _ = child.wait()` in the timeout
  path (the process might already be dead -- use `let _ =`, not `.unwrap()`)

`LsRunner::run()` should:
- Spawn `ls` with the path, using a timeout (polling loop from Day 4)
- Return `ToolResult::Success` on exit 0
- Return `ToolResult::Failure` on non-zero exit
- Return `ToolResult::Timeout` if timeout exceeded
- Return `ToolError::NotFound` / `ToolError::PermissionDenied` on spawn failure

4. In `main()`, create instances and call `.run()` on each:

```rust
let echo = EchoRunner { message: "hello from trait".to_string(), timeout: Duration::from_secs(5) };
let ls_ok = LsRunner { path: "/tmp".to_string(), timeout: Duration::from_secs(5) };
let ls_bad = LsRunner { path: "/root".to_string(), timeout: Duration::from_secs(5) };
```

5. Write a function that takes `&dyn Executable`:

```rust
fn execute_tool(tool: &dyn Executable) {
    println!("Running: {}", tool.description());
    match tool.run() {
        Ok(result) => { /* print based on variant */ }
        Err(e) => { /* print error */ }
    }
}
```

Call `execute_tool` with both `&echo` and `&ls_ok` and `&ls_bad`.

**Expected output pattern:**
```
=== Trait-Based Tool Runner ===

Running: echo runner
Success in 0.01s
stdout: hello from trait

Running: ls runner
Success in 0.01s
stdout: (contents of /tmp)

Running: ls runner
Failure (exit 2)
stderr: ls: cannot open directory '/root': Permission denied
```

**Run:** `cargo run --bin day5_traits`

---

### Exercise 2 -- Implement `Display` for your types

Create `~/Antigravity/Achilles/src/bin/day5_display.rs`.

**Task:** Implement the `Display` trait for `ToolResult` and `ToolError`
so they can be printed with `{}` instead of `{:?}`.

1. Copy your `ToolResult` and `ToolError` enums from Exercise 1

2. Implement `Display` for `ToolResult`:

```
Success in 0.05s — stdout: hello
Failure (exit 2) — stderr: permission denied
Timeout after 5.00s
```

3. Implement `Display` for `ToolError`:

```
NotFound: No such file or directory (os error 2)
PermissionDenied: permission denied
ExecutionFailed: some error message
```

4. In `main()`, create instances of each variant and print them
   with `println!("{}", result)` and `println!("{}", error)`.

**Hints:**
- `use std::fmt;`
- `impl fmt::Display for ToolResult { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { ... } }`
- Inside `fmt`, use `match self { ... }` and `write!(f, "...")` for each variant

**Run:** `cargo run --bin day5_display`

---

### Exercise 3 -- Implement `From<io::Error>` for `ToolError`

Create `~/Antigravity/Achilles/src/bin/day5_from.rs`.

**Task:** Eliminate the manual error conversion boilerplate from Day 4.

1. Copy your `ToolError` enum
2. Implement `From<io::Error> for ToolError` that checks `e.kind()`
   and maps to the right variant (NotFound, PermissionDenied, ExecutionFailed)
3. Write a function:

```rust
fn spawn_tool(binary: &str) -> Result<Child, ToolError> {
    let child = Command::new(binary)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;  // <-- this ? now auto-converts io::Error to ToolError!
    Ok(child)
}
```

4. In `main()`, test:

```rust
spawn_tool("echo");           // should work
spawn_tool("fake_binary");    // should return ToolError::NotFound
```

5. Verify that `?` works without manual `match` on the error.

**Expected output pattern:**
```
=== From<io::Error> for ToolError ===

echo: spawned successfully (PID: 12345)
fake_binary: NotFound: No such file or directory (os error 2)
```

**Run:** `cargo run --bin day5_from`

---

### Exercise 4 -- Trait objects and dynamic dispatch

Create `~/Antigravity/Achilles/src/bin/day5_generics.rs`.

**Task:** Store different tool types in one collection and run them
through a single interface.

1. Copy your `Executable` trait, `EchoRunner`, and `LsRunner` from Exercise 1

2. Write a function that takes a trait object reference:

```rust
fn run_and_report(tool: &dyn Executable) {
    println!("=== {} ===", tool.name());
    println!("Description: {}", tool.description());
    match tool.run() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    println!();
}
```

**Why `&dyn Executable` and not `&impl Executable`:**
`run_and_report` is called from `run_all`, which iterates over
`Vec<Box<dyn Executable>>`. At that point, the concrete type is
unknown -- it could be EchoRunner or LsRunner. Generics (`impl Trait`)
require the compiler to know the exact type at compile time (the
hidden `Sized` bound). `dyn Trait` is NOT `Sized` -- its size is
unknown. So you must use `&dyn Executable` (dynamic dispatch), not
a generic.

3. Write a function that takes a `Vec` of trait objects:

```rust
fn run_all(tools: Vec<Box<dyn Executable>>) {
    for tool in &tools {
        run_and_report(tool.as_ref());
    }
}
```

**Why `.as_ref()`:**
In the loop, `tool` is `&Box<dyn Executable>` -- a reference to a Box.
But `run_and_report` wants `&dyn Executable` -- a reference to the
trait object directly. `.as_ref()` peels off the Box layer:

```
&Box<dyn Executable>   ← what you have
        │
        │  .as_ref()
        ▼
&dyn Executable        ← what run_and_report needs
```

4. In `main()`, create a `Vec<Box<dyn Executable>>` containing both
   an `EchoRunner` and an `LsRunner`, and pass it to `run_all`.

**Why `Box<dyn Executable>`:**
- A `Vec` needs all elements to be the same size
- `EchoRunner` and `LsRunner` are different sizes
- `Box<dyn Executable>` is always the same size (a pointer)
- This is how you store different types in one collection

**Run:** `cargo run --bin day5_generics`

---

### Exercise 5 -- Preview: the ADCObject enum pattern

Create `~/Antigravity/Achilles/src/bin/day5_adc_preview.rs`.

**Task:** Build a simplified version of the ADCObject enum to
understand the Achilles data contract pattern.

1. Define simplified ADC types:

```rust
#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
}

#[derive(Debug, Clone)]
struct Port {
    number: u16,
    protocol: String,
    state: String,
}

#[derive(Debug, Clone)]
struct Finding {
    title: String,
    severity: String,
    host_ip: String,
}
```

2. Define the ADCObject enum:

```rust
enum ADCObject {
    Host(Host),
    Port(Port),
    Finding(Finding),
}
```

3. Implement `Display` for `ADCObject` (match on each variant, print
   a meaningful one-line summary)

4. Write a function:

```rust
fn process_objects(objects: &[ADCObject]) {
    let mut host_count = 0;
    let mut port_count = 0;
    let mut finding_count = 0;

    for obj in objects {
        match obj {
            ADCObject::Host(_) => host_count += 1,
            ADCObject::Port(_) => port_count += 1,
            ADCObject::Finding(_) => finding_count += 1,
        }
    }

    println!("Hosts: {}, Ports: {}, Findings: {}", host_count, port_count, finding_count);
}
```

5. In `main()`, create a `Vec<ADCObject>` with a mix of hosts, ports,
   and findings. Pass it to `process_objects`. Then print each object
   using your `Display` implementation.

**Why this matters:**
This is exactly how data flows in Achilles. Nmap produces
`ADCObject::Host` and `ADCObject::Port`. Nuclei produces
`ADCObject::Finding`. The merge node receives `Vec<ADCObject>`
and sorts them by type. The report node matches on each variant
to format the output. **The compiler ensures every variant is handled.**

**Run:** `cargo run --bin day5_adc_preview`

---

## Part 14: Rustlings

Before or after the exercises (your choice), complete these
Rustlings sections:

```
generics1, generics2
traits1, traits2, traits3, traits4, traits5
```

These reinforce the concepts from a different angle.
They're short -- 30-45 minutes total.

Run: `rustlings`

---

## Part 15: Key Concepts to Internalize

**1. What is a trait?**
A contract. Any type implementing it must have the specified methods.
Like a Java interface, but with default implementations and trait bounds.

**2. What is `impl Trait for Struct`?**
"This struct fulfills the trait's contract by providing these method
implementations."

**3. What is a generic `<T>`?**
A type parameter. The function/struct works for any type T.
The compiler generates a specialized version for each concrete type.

**4. What is a trait bound `<T: Display>`?**
A constraint on a generic. "T can be any type, BUT it must implement Display."
Without bounds, you can't call any methods on T because the compiler
doesn't know what T is.

**5. What is `&dyn Trait`?**
A reference to any type implementing the trait.
Uses dynamic dispatch (vtable lookup at runtime).
Needed when you want to store different types in the same collection.

**6. What is `Box<dyn Trait>`?**
An owned, heap-allocated trait object.
Needed for `Vec<Box<dyn Executable>>` -- different types in one Vec.

**7. Why does Achilles use an enum for ADCObject instead of `Box<dyn ADCType>`?**
Exhaustive matching. When you add a new ADC type, the compiler forces
you to handle it everywhere. With trait objects, you'd need runtime
downcasting and could forget a type.

**8. What is the `From` trait?**
A conversion trait. `impl From<A> for B` lets you do `B::from(a)`.
The `?` operator uses `From` to auto-convert error types.

**9. What is `Display` vs `Debug`?**
`Display` = user-facing output with `{}`.
`Debug` = developer-facing output with `{:?}`.
You derive `Debug`. You implement `Display` manually.

**10. What are `Send` and `Sync`?**
`Send` = type can be transferred to another thread.
`Sync` = type can be shared between threads via `&T`.
Most types are both. The `Node: Send + Sync` supertrait
ensures nodes can run on Tokio's thread pool.

**11. What is `cargo clippy`?**
A linter that catches non-idiomatic Rust patterns.
Run it before every commit starting today.

**12. What is `cargo fmt`?**
An auto-formatter. Standardizes code style.
Run it before every commit starting today.

**13. What is `let _ =`?**
Intentional Result discard. Tells the compiler: "I know this returns
a Result. I'm deliberately ignoring it." Use when failure is expected
and harmless (killing a dead process, closing a closed file).
Don't use to silence warnings on errors that actually matter.

---

## Completion Checklist

- [ ] Read and understand Parts 1-12
      (traits, generics, trait bounds, trait objects vs enums,
      Display, From, supertraits, discarding Results, clippy, fmt)
- [ ] Exercise 1 -- Trait-based tool runner (`day5_traits.rs`)
- [ ] Exercise 2 -- Display implementations (`day5_display.rs`)
- [ ] Exercise 3 -- From trait to eliminate boilerplate (`day5_from.rs`)
- [ ] Exercise 4 -- Generic functions + Vec<Box<dyn Executable>> (`day5_generics.rs`)
- [ ] Exercise 5 -- ADCObject enum preview (`day5_adc_preview.rs`)
- [ ] Complete Rustlings: generics1-2, traits1-5
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Run `cargo fmt` on all files
- [ ] Can explain the difference between `impl Trait` and `dyn Trait`
- [ ] Can explain why Achilles uses enums instead of trait objects for ADC
- [ ] Can implement `Display` and `From` for custom types

---

## Lesson

> The engine doesn't know what tool it's running.
> It doesn't know nmap from nuclei from sqlmap.
> All it knows is: "this thing has `.execute()`.
> I call it. I get typed data back."
>
> That wall of abstraction is a trait.
> It means adding a new tool to Achilles is writing one struct
> and one `impl` block. The engine never changes.
> The DAG never changes. The audit log never changes.
>
> The trait is the contract.
> Break the contract, the compiler breaks your build.
> Honor the contract, the engine runs your tool.

---

**Next: Day 6 -- Vec, HashMap, Iterators, Closures, and Collection Transforms ->**
