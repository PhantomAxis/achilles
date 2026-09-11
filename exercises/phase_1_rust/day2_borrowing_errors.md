# Phase 1, Day 2 — Borrowing, References, Lifetimes, `String` vs `&str`, `Result<T, E>`, and the `?` Operator

> **Time budget:** 3–5 hours of focused work.
> **Prerequisite:** Day 1 complete. You understand ownership, moves, Copy vs Move, and stack vs heap.
> **Outcome:** You can write functions that borrow data instead of consuming it, understand the difference between `String` and `&str`, handle
> all errors with `Result<T, E>`, and use `serde_json` for serialization.

> [!IMPORTANT]
> Day 1 was about **ownership** — who owns data. Day 2 is about **borrowing** — how to use data without owning it. Together, these two concepts
> are the foundation of every line of Rust you'll ever write.

---

## Why This Day Matters

On Day 1, every function that received a `String` consumed it. You had to clone or return ownership — both clumsy. In real code (especially Achilles), you need
functions that *read* data without taking ownership. An nmap transformer reads the XML output but shouldn't own it — other transformers might need the same data.

This is what **references** solve. And `Result<T, E>` solves the other half of the problem: what happens when things go wrong. Python uses exceptions — your 
program crashes at runtime if you forget to handle one. Rust uses `Result` — the compiler **forces** you to handle every error path at compile time.

---

## Part 1: References and Borrowing

### The Problem (from Day 1)

```rust
fn main() {
    let target = String::from("10.10.10.100");
    validate(target);           // target is MOVED
    println!("{}", target);     // ❌ Can't use target — it was consumed
}

fn validate(t: String) { /* ... */ }
```

### The Solution: Borrow with `&`

```rust
fn main() {
    let target = String::from("10.10.10.100");
    validate(&target);          // BORROW — target is lent, not given
    println!("{}", target);     // ✅ Works — target was never moved
}

fn validate(t: &String) {      // t is a REFERENCE — it borrows, doesn't own
    println!("Validating: {}", t);
}                               // t goes out of scope, but it doesn't own the data
                                // so nothing is dropped
```

**What `&` means:** It creates a **reference** — a pointer to the data. The function can read the data but doesn't own it. When the reference goes 
out of scope, nothing is freed because the reference never owned anything.

```
                  Ownership:              Borrowing:
                  fn validate(t: String)  fn validate(t: &String)

Stack:            ┌────┐                  ┌────┐
  target          │ ptr│──→ "10.10.10.100"│ ptr│──→ "10.10.10.100"
                  └────┘   (MOVED to t)   └────┘   (still owned by target)
                                          ┌────┐
                  target is GONE          │  t │──→ (points to target)
                                          └────┘
```

### Two Kinds of References

```
┌──────────────────────────────────────────────────────┐
│            BORROWING RULES                            │
├──────────────────────────────────────────────────────┤
│                                                        │
│  1. You can have MANY immutable references (&T)        │
│     OR ONE mutable reference (&mut T)                  │
│     — but never both at the same time.                │
│                                                        │
│  2. References must always be valid                    │
│     (no dangling pointers).                           │
│                                                        │
│  Think of it like a library book:                     │
│  • Many people can READ the book at the same time     │
│  • Only ONE person can WRITE in it at a time          │
│  • Nobody can write while others are reading          │
│                                                        │
└──────────────────────────────────────────────────────┘
```

**Immutable reference (`&T`):** Read-only access. Multiple allowed simultaneously.

```rust
fn main() {
    let target = String::from("10.10.10.100");

    let r1 = &target;     // ✅ First immutable borrow
    let r2 = &target;     // ✅ Second immutable borrow — multiple readers OK
    println!("{}, {}", r1, r2);  // ✅ Both valid
}
```

**Mutable reference (`&mut T`):** Read-write access. Only ONE allowed, and no immutable references can coexist.

```rust
fn main() {
    let mut target = String::from("10.10.10.100");

    let r1 = &mut target;    // ✅ Mutable borrow
    r1.push_str(":8080");    // ✅ Can modify through the reference
    println!("{}", r1);      // "10.10.10.100:8080"

    // let r2 = &target;     // ❌ Can't borrow as immutable while mutably borrowed
}
```

**Why this restriction?** It prevents data races at compile time:
- Multiple readers = safe (nobody is modifying)
- One writer, no readers = safe (exclusive access)
- One writer + readers = **data race** → compiler rejects it

### Borrowing in functions

```rust
// Immutable borrow — reads but doesn't modify
fn count_ports(ports: &Vec<u16>) -> usize {
    ports.len()
}

// Mutable borrow — can modify the data
fn add_port(ports: &mut Vec<u16>, port: u16) {
    ports.push(port);
}

fn main() {
    let mut ports = vec![22, 80, 443];

    // Multiple immutable borrows — fine
    let count = count_ports(&ports);
    println!("Ports: {}", count);

    // Mutable borrow — fine (no immutable borrows active)
    add_port(&mut ports, 8080);
    println!("Ports: {:?}", ports);  // [22, 80, 443, 8080]
}
```

---

## Part 2: `String` vs `&str` — The Two String Types

This is one of the most confusing things for Rust beginners. Once it clicks, it clicks forever.

```
┌──────────────────────────────────────────────────────┐
│              String vs &str                           │
├──────────────────────────────────────────────────────┤
│                                                        │
│  String                        &str                    │
│  ──────                        ────                    │
│  Owned, heap-allocated         Borrowed string slice   │
│  Growable (push, push_str)     Read-only view          │
│  Like Vec<u8> with UTF-8       Like &[u8] with UTF-8   │
│                                                        │
│  String::from("hello")        "hello" (a literal)      │
│  let s = String::new();       let s: &str = "hello";   │
│  s.push_str(" world");        // Can't modify           │
│                                                        │
│  Memory:                                               │
│                                                        │
│  String:     ┌─────────┐     ┌──────────┐             │
│              │ ptr ─────│────→│ h e l l o│  (heap)     │
│              │ len: 5   │     └──────────┘             │
│              │ cap: 8   │                              │
│              └─────────┘                               │
│              (stack, 24B)                               │
│                                                        │
│  &str:       ┌─────────┐     ┌──────────┐             │
│              │ ptr ─────│────→│ h e l l o│  (anywhere) │
│              │ len: 5   │     └──────────┘             │
│              └─────────┘                               │
│              (stack, 16B)                               │
│                                                        │
│  &str can point to:                                    │
│  • A string literal (baked into the binary)            │
│  • A slice of a String (heap)                          │
│  • A slice of another &str                             │
│                                                        │
└──────────────────────────────────────────────────────┘
```

### The Rule for Function Parameters

**If your function only needs to READ a string, take `&str`:**

```rust
// ✅ GOOD — accepts both String and &str
fn validate_target(target: &str) -> bool {
    !target.is_empty() && target.contains('.')
}

fn main() {
    let owned = String::from("10.10.10.100");
    let literal = "scanme.nmap.org";

    validate_target(&owned);    // ✅ &String auto-converts to &str
    validate_target(literal);   // ✅ &str passed directly
}
```

**If your function needs to OWN or MODIFY the string, take `String`:**

```rust
// Takes ownership — the caller can't use the String anymore
fn consume(target: String) {
    println!("Consumed: {}", target);
}
```

**The guideline:**
- Function parameters: prefer `&str` (most flexible — accepts both `String` and `&str`)
- Struct fields that own data: use `String`
- Return types when creating new strings: use `String`

### Converting between them

```rust
let s: String = String::from("Achilles");
let r: &str = &s;              // String → &str (free, just a reference)
let s2: String = r.to_string(); // &str → String (allocates new heap memory)
let s3: String = "literal".to_string(); // same
let s4: String = String::from("literal"); // same
```

---

## Part 3: Lifetimes (The Basics)

Lifetimes ensure that references don't outlive the data they point to. The compiler tracks this automatically for most cases.

```rust
fn main() {
    let r;                      // Declare a reference
    {
        let s = String::from("temporary");
        r = &s;                 // ❌ s will be dropped at the end of this block
    }                           // s is dropped here
    // println!("{}", r);       // ❌ r would be a DANGLING REFERENCE
}
```

The compiler catches this: `s does not live long enough`. The reference `r` would outlive the data it points to — Rust prevents this.

### When you see lifetime annotations

Sometimes the compiler can't figure out lifetimes automatically. You'll see this syntax:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

**What `'a` means:** "The returned reference lives at least as long as both input references." This tells the compiler how to check that the return value won't be a dangling pointer.

> [!NOTE]
> Don't worry about writing complex lifetime annotations yet. For now, understand:
> - References can't outlive the data they borrow
> - The compiler tracks this automatically most of the time
> - When it can't, it asks you to annotate with `'a`
> - You'll get much more practice with lifetimes as the project progresses

---

## Part 4: `Result<T, E>` and the `?` Operator

### The Problem with Exceptions

In Python:

```python
data = open("config.json").read()  # What if the file doesn't exist?
parsed = json.loads(data)          # What if it's not valid JSON?
```

This code silently assumes success. If either line fails, the program crashes with a traceback. You can add `try/except`, but no one forces you to. 
In production, the unhandled exception crashes your service at 3 AM.

### Rust's Solution: `Result<T, E>`

Every operation that can fail returns a `Result`:

```rust
enum Result<T, E> {
    Ok(T),    // Success — contains the value
    Err(E),   // Failure — contains the error
}
```

You **must** handle both cases. The compiler won't let you ignore a `Result`.

```rust
use std::fs;

fn main() {
    // read_to_string returns Result<String, io::Error>
    let result = fs::read_to_string("config.json");

    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Failed to read file: {}", error),
    }
}
```

### The `?` Operator — Elegant Error Propagation

Writing `match` for every `Result` is verbose. The `?` operator is shorthand:

```rust
use std::fs;
use std::io;

fn read_config() -> Result<String, io::Error> {
    let contents = fs::read_to_string("config.json")?;
    // If read_to_string returns Ok(contents), ? unwraps it and assigns to `contents`
    // If read_to_string returns Err(e), ? immediately returns Err(e) from THIS function
    Ok(contents)
}
```

**`?` does exactly this:**

```rust
// This:
let contents = fs::read_to_string("config.json")?;

// Is equivalent to:
let contents = match fs::read_to_string("config.json") {
    Ok(c) => c,
    Err(e) => return Err(e),
};
```

**Chain multiple `?` for clean error handling:**

```rust
use std::fs;
use std::io;

fn read_and_parse_config() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("config.json")?;  // File error?
    let parsed = serde_json::from_str(&contents)?;       // Parse error?
    Ok(parsed)
}
// If either line fails, the error propagates up to the caller.
// No try/catch. No exceptions. Just types.
```

### `unwrap()` and `expect()` — The Emergency Exits

```rust
// unwrap: if it's Ok, give me the value. If it's Err, PANIC (crash).
let contents = fs::read_to_string("config.json").unwrap();

// expect: same as unwrap, but with a custom panic message.
let contents = fs::read_to_string("config.json")
    .expect("Failed to read config.json");
```

> [!CAUTION]
> **Never use `unwrap()` in production code.** It crashes the program. Use it only in:
> - Quick tests and prototypes
> - Situations where failure is truly impossible (and add a comment explaining why)
>
> In Achilles, every error is handled with `Result` and `?`. No `unwrap()` in the engine.

---

## Part 5: Exercises

### Exercise 1 — Continue Rustlings through `error_handling`

```bash
cd ~/Antigravity/rustlings
rustlings
```

Continue from where you left off. Complete all exercises through `error_handling`:

| Category | What you learn |
|----------|---------------|
| `structs` | Struct definition, methods, associated functions |
| `enums` | Enums, `match`, `Option<T>` |
| `strings` | `String` vs `&str`, conversions |
| `modules` | Module system, visibility (`pub`) |
| `hashmaps` | `HashMap` creation, insertion, access |
| `options` | `Option<T>`, `Some`, `None`, pattern matching |
| `error_handling` | `Result<T, E>`, `?` operator, custom errors |

Write one sentence per exercise (just like Day 1) in your notes.

---

### Exercise 2 — Word counter with borrowing

Create `~/Antigravity/Achilles/src/bin/day2_borrowing.rs`:

```rust
// Day 2: Borrowing — word counter
//
// The function borrows text with &str — it reads without owning.
// Run with: cargo run --bin day2_borrowing

use std::collections::HashMap;

/// Count the frequency of each word in the given text.
///
/// Takes &str (a borrow) — the caller keeps ownership of the original text.
/// Returns a HashMap that the caller OWNS (new data created inside the function).
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        // word is a &str — a slice of the original text
        // We need to own it to store in the HashMap, so we call .to_string()
        let count = counts.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    counts
}

fn main() {
    let nmap_output = "PORT STATE SERVICE
22/tcp open ssh
80/tcp open http
443/tcp open https
8080/tcp open http
22/tcp filtered ssh";

    // We pass &nmap_output — borrowing, not moving
    let word_counts = count_words(nmap_output);

    // nmap_output is still valid — it was only borrowed
    println!("Original text length: {} chars", nmap_output.len());
    println!("\nWord frequencies:");

    // Collect into a Vec and sort for consistent output
    let mut sorted: Vec<_> = word_counts.iter().collect();
    sorted.sort_by_key(|(word, _)| word.clone());

    for (word, count) in &sorted {
        println!("  {:15} → {}", word, count);
    }

    // TASK 1: Why does count_words take &str instead of String?
    //         Write your answer as a comment below.

    // TASK 2: What would happen if count_words took String instead of &str?
    //         Try changing the signature and see the compiler error.

    // TASK 3: Call count_words TWICE with the same text.
    //         This works because &str is a borrow — nmap_output isn't consumed.
    let _counts_again = count_words(nmap_output);
    println!("\nCalled count_words twice with the same text ✅");

    // TASK 4: Call count_words with a string literal directly.
    //         This works because &str accepts both String references and literals.
    let _literal_counts = count_words("hello world hello");
    println!("Called count_words with a literal ✅");
}
```

**Run it:**

```bash
cargo run --bin day2_borrowing
```

---

### Exercise 3 — File reading with `Result` and `?`

First, add `serde_json` to your project dependencies:

```bash
cd ~/Antigravity/Achilles
cargo add serde_json
cargo add serde --features derive
```

Create `~/Antigravity/Achilles/src/bin/day2_errors.rs`:

```rust
// Day 2: Error handling — Result<T, E> and the ? operator
//
// Run with: cargo run --bin day2_errors

use std::collections::HashMap;
use std::fs;
use std::io;

/// Read a file and return its contents.
/// Uses the ? operator to propagate errors to the caller.
fn read_file(path: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(path)?;
    // If the file doesn't exist → ? returns Err(io::Error) immediately
    // If it succeeds → contents holds the file text
    Ok(contents)
}

/// Count words in a file. Demonstrates chaining ? operators.
fn count_words_in_file(path: &str) -> Result<HashMap<String, usize>, io::Error> {
    let contents = read_file(path)?;  // ? propagates file errors

    let mut counts = HashMap::new();
    for word in contents.split_whitespace() {
        let count = counts.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    Ok(counts)
}

fn main() {
    println!("=== Error Handling with Result<T, E> ===\n");

    // ----- SUCCESS CASE -----
    // Create a test file to read
    let test_path = "/tmp/achilles_day2_test.txt";
    fs::write(test_path, "PORT STATE SERVICE\n22/tcp open ssh\n80/tcp open http")
        .expect("Failed to create test file");

    match read_file(test_path) {
        Ok(contents) => println!("✅ File read successfully:\n{}\n", contents),
        Err(e) => println!("❌ Error reading file: {}", e),
    }

    // ----- FAILURE CASE -----
    match read_file("/tmp/this_file_does_not_exist.txt") {
        Ok(contents) => println!("✅ File read: {}", contents),
        Err(e) => {
            println!("❌ Error reading nonexistent file: {}", e);
            println!("   Error kind: {:?}", e.kind());
            // e.kind() returns io::ErrorKind — you can match on specific errors
        }
    }

    // ----- CHAINED ? WITH count_words_in_file -----
    println!("\n=== Chained ? Operator ===\n");

    match count_words_in_file(test_path) {
        Ok(counts) => {
            println!("✅ Word counts from file:");
            for (word, count) in &counts {
                println!("  {:15} → {}", word, count);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    // ----- THE WRONG WAY: unwrap -----
    println!("\n=== unwrap() vs proper error handling ===\n");

    // This would PANIC if the file doesn't exist:
    // let contents = fs::read_to_string("nonexistent.txt").unwrap(); // 💥 CRASH

    // This handles the error properly:
    let result = fs::read_to_string("nonexistent.txt");
    if let Err(e) = result {
        println!("Handled gracefully: {}", e);
    }

    // Clean up
    let _ = fs::remove_file(test_path);

    // ----- TASKS -----
    // TASK 1: Write a function that reads TWO files and concatenates them.
    //         Signature: fn concat_files(path1: &str, path2: &str) -> Result<String, io::Error>
    //         Use ? for both reads.

    // TASK 2: Call your concat_files with:
    //         a) Two files that exist
    //         b) First exists, second doesn't
    //         c) First doesn't exist, second does
    //         Observe which error propagates in each case.
}
```

**Run it:**

```bash
cargo run --bin day2_errors
```

---

### Exercise 4 — JSON serialization with serde

Create `~/Antigravity/Achilles/src/bin/day2_serde.rs`:

```rust
// Day 2: Serde — serialization and deserialization
//
// This is how Achilles will convert between Rust types and JSON.
// Every ADC type (Host, Port, Finding) will derive Serialize + Deserialize.
//
// Run with: cargo run --bin day2_serde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simplified version of the Achilles Host type.
/// The derive macros auto-generate serialization code.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Host {
    id: String,
    ip: String,
    hostnames: Vec<String>,
    open_ports: Vec<u16>,
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    println!("=== Serde: Rust ↔ JSON ===\n");

    // ----- PART A: Serialize a HashMap to JSON -----
    let text = "PORT STATE SERVICE\n22/tcp open ssh\n80/tcp open http\n22/tcp open ssh";
    let word_counts = count_words(text);

    let json = serde_json::to_string_pretty(&word_counts)
        .expect("Failed to serialize");
    println!("HashMap → JSON:\n{}\n", json);

    // Deserialize it back
    let deserialized: HashMap<String, usize> = serde_json::from_str(&json)
        .expect("Failed to deserialize");

    // Assert round-trip is lossless
    assert_eq!(word_counts, deserialized);
    println!("✅ Round-trip HashMap → JSON → HashMap: lossless\n");

    // ----- PART B: Serialize a struct -----
    let host = Host {
        id: "host-001".to_string(),
        ip: "10.10.10.100".to_string(),
        hostnames: vec!["target.local".to_string(), "metasploitable.lab".to_string()],
        open_ports: vec![22, 80, 443, 3306, 5432],
    };

    let host_json = serde_json::to_string_pretty(&host)
        .expect("Failed to serialize Host");
    println!("Host → JSON:\n{}\n", host_json);

    // Deserialize back to a Host struct
    let host_restored: Host = serde_json::from_str(&host_json)
        .expect("Failed to deserialize Host");

    assert_eq!(host, host_restored);
    println!("✅ Round-trip Host → JSON → Host: lossless\n");

    // ----- PART C: Handle invalid JSON -----
    let bad_json = r#"{"id": "host-002", "ip": 12345}"#;  // ip should be a string, not a number

    match serde_json::from_str::<Host>(bad_json) {
        Ok(h) => println!("Parsed: {:?}", h),
        Err(e) => {
            println!("❌ Failed to parse bad JSON: {}", e);
            println!("   (This is good — serde caught the type mismatch at runtime)");
        }
    }

    // ----- PART D: Serialize to file, read back -----
    let path = "/tmp/achilles_host.json";
    let json_bytes = serde_json::to_string_pretty(&host).unwrap();
    std::fs::write(path, &json_bytes).expect("Failed to write file");
    println!("\n✅ Wrote Host to {}", path);

    let file_contents = std::fs::read_to_string(path).expect("Failed to read file");
    let file_host: Host = serde_json::from_str(&file_contents).expect("Failed to parse file");
    assert_eq!(host, file_host);
    println!("✅ Read Host back from file: lossless round-trip\n");

    // Clean up
    let _ = std::fs::remove_file(path);

    // ----- WHY THIS MATTERS -----
    println!("=== Achilles Context ===");
    println!("Every ADC type (Host, Port, Finding, Credential) will derive");
    println!("Serialize + Deserialize. This means:");
    println!("  • nmap XML → parsed → Host struct → JSON → passed to next node");
    println!("  • Workflow state saved to disk as JSON");
    println!("  • API responses serialized automatically");
    println!("Serde is the foundation of all data flow in Achilles.");

    // TASK: Add a Port struct with fields: number (u16), protocol (String),
    //       state (String), service (Option<String>).
    //       Derive Serialize + Deserialize.
    //       Create a Vec<Port>, serialize to JSON, deserialize back,
    //       assert equality.
}
```

**Run it** (make sure you've added the dependencies):

```bash
cargo run --bin day2_serde
```

---

### Exercise 5 — Mutable references in practice

Create `~/Antigravity/Achilles/src/bin/day2_mut_refs.rs`:

```rust
// Day 2: Mutable references — modifying borrowed data
//
// Run with: cargo run --bin day2_mut_refs

fn main() {
    println!("=== Mutable References ===\n");

    // ----- PART A: Basic mutable reference -----
    let mut ports = vec![22, 80, 443];
    println!("Before: {:?}", ports);

    add_common_ports(&mut ports);
    println!("After:  {:?}", ports);

    // ----- PART B: The borrowing rules in action -----
    let mut target = String::from("10.10.10.100");

    // Multiple immutable borrows — fine
    let r1 = &target;
    let r2 = &target;
    println!("r1={}, r2={}", r1, r2);
    // r1 and r2 are no longer used after this point — their borrows end here

    // Mutable borrow — fine because r1 and r2 are no longer active
    let r3 = &mut target;
    r3.push_str(":8080");
    println!("r3={}", r3);

    // TASK: Uncomment the line below. Read the compiler error.
    //       It fails because r3 (mutable) and r1 (immutable) would coexist.
    // println!("r1={}", r1);

    // ----- PART C: Mutable reference to struct fields -----
    let mut scan_config = ScanConfig {
        target: String::from("10.10.10.100"),
        ports: vec![22, 80],
        timeout_secs: 30,
    };

    println!("\nBefore: {:?}", scan_config);
    configure_aggressive(&mut scan_config);
    println!("After:  {:?}", scan_config);

    // ----- PART D: Return a mutable reference -----
    let mut data = vec![1, 5, 3, 8, 2];
    println!("\nBefore sort: {:?}", data);

    // sort_and_return borrows mutably and returns the reference
    let sorted = sort_and_return(&mut data);
    println!("Sorted: {:?}", sorted);
    // After sorted is done being used, we can use data again
    println!("Data is still accessible: {:?}", data);
}

fn add_common_ports(ports: &mut Vec<u16>) {
    let common = [8080, 8443, 3000, 3306];
    for port in common {
        if !ports.contains(&port) {
            ports.push(port);
        }
    }
}

#[derive(Debug)]
struct ScanConfig {
    target: String,
    ports: Vec<u16>,
    timeout_secs: u32,
}

fn configure_aggressive(config: &mut ScanConfig) {
    config.ports = vec![1, 21, 22, 23, 25, 53, 80, 110, 135, 139,
                        143, 443, 445, 993, 995, 1723, 3306, 3389,
                        5432, 5900, 8080, 8443];
    config.timeout_secs = 10;
}

fn sort_and_return(data: &mut Vec<i32>) -> &Vec<i32> {
    data.sort();
    data  // Returns an immutable reference to the sorted data
}
```

**Run it:**

```bash
cargo run --bin day2_mut_refs
```

---

## Part 6: Key Concepts to Internalize

| Question | Your Answer Should Include |
|----------|--------------------------|
| What is a reference (`&T`)? | A pointer that borrows data without taking ownership. Nothing is freed when it goes out of scope. |
| What's the difference between `&T` and `&mut T`? | `&T` is read-only (many allowed). `&mut T` is read-write (only one allowed, no `&T` can coexist). |
| Why can't you have `&T` and `&mut T` at the same time? | Prevents data races — one writer + readers = unsafe. Compiler enforces exclusive write access. |
| What is `String` vs `&str`? | `String` = owned, heap, growable. `&str` = borrowed view, read-only. Function params should prefer `&str`. |
| How do you convert `String` to `&str`? | `&my_string` — free, just creates a reference |
| How do you convert `&str` to `String`? | `.to_string()` or `String::from(s)` — allocates new heap memory |
| What is `Result<T, E>`? | An enum: `Ok(T)` for success, `Err(E)` for failure. Must be handled — compiler enforces it. |
| What does the `?` operator do? | Unwraps `Ok` or returns `Err` from the current function. Shorthand for match + early return. |
| Why is `unwrap()` bad? | It panics (crashes) on `Err`. Use `?` or `match` in production code. |
| What is `serde`? | Rust's serialization framework. `#[derive(Serialize, Deserialize)]` auto-generates JSON conversion. |
| What is a lifetime? | The compiler's tracking of how long a reference is valid. Ensures no dangling pointers. |

---

## Completion Checklist

- [x] Rustlings: completed all exercises through `error_handling`
- [x] `day2_borrowing.rs`: Implemented `count_words` with `&str`, called it multiple times with same data
- [x] `day2_errors.rs`: Used `Result<T, E>` and `?` operator, handled success and failure cases
- [x] `day2_serde.rs`: Serialized/deserialized a HashMap and a struct, verified round-trip lossless
- [x] `day2_mut_refs.rs`: Used `&mut` references, understood the borrowing rules
- [x] I can explain the difference between `String` and `&str` and when to use each
- [x] I can explain why `?` is preferred over `unwrap()` in production code
- [x] I understand the borrowing rules: many `&T` OR one `&mut T`, never both
- [x] I've added `serde` and `serde_json` to `Cargo.toml`

---

## Lesson

> Rust forces you to handle errors at compile time. Python lets you ship a program that crashes at 3 AM. The `Result` type is not a nuisance — it is a guarantee that every error path is handled. The borrow checker is not a gatekeep — it is proof that your references are valid. When the compiler says no, it's protecting you from a bug you can't see yet.

---

**Next: Day 3 — Subprocess Management: `std::process::Command`, Capturing Output, Exit Codes →**
