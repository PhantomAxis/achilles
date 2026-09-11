# Phase 1, Day 1 — Variables, Ownership, Moves, and the Stack vs Heap

> **Time budget:** 3–5 hours of focused work.
> **Prerequisite:** Phase 0b complete. Rust toolchain installed, `cargo run` works, Emacs configured with rustic/rust-mode.
> **Outcome:** You understand Rust's ownership system — the single most important concept in the language. You've felt the borrow checker 
> reject your code, understood *why*, and fixed it.

> [!IMPORTANT]
> These first two days are dedicated entirely to Rust's ownership model. Do **not** skip ahead to subprocesses. The borrow checker must click first. 
> Every compiler error from today onward is the compiler *saving you from a bug*. Learn to read them, not fight them.

---

## Why This Day Matters

In Python, you write `a = [1, 2, 3]` then `b = a`. Both `a` and `b` point to the same list. Modify `a[0]` and `b[0]` changes too. This is fine for scripts. 
It is catastrophic for systems software.

Achilles will manage concurrent subprocess I/O — multiple tool outputs arriving simultaneously, being parsed and passed between pipeline stages. 
If two threads hold a reference to the same data and one modifies it while the other reads it, you get a **data race** — the program silently produces 
wrong results or crashes. C and C++ allow this. Python's GIL hides it (mostly). Rust makes it **impossible at compile time**.

The mechanism that makes this possible is **ownership**. Every value in Rust has exactly one owner. When the owner goes out of scope, the value is 
dropped (freed). Two variables cannot own the same heap data. This sounds restrictive — it is. And it eliminates entire categories of bugs that have 
  * plagued systems programming for 50 years.

---

## Part 1: The Mental Model — Stack vs Heap

Before you can understand ownership, you need to understand where data lives.

### The Stack

The stack is fast, automatic memory. It works like a stack of plates — last in, first out.

```
┌──────────────────────────────────────────────────────┐
│                    THE STACK                           │
├──────────────────────────────────────────────────────┤
│                                                        │
│  Every function call pushes a "frame" onto the stack.  │
│  Local variables live inside the frame.                │
│  When the function returns, the frame is popped.       │
│                                                        │
│  fn main() {                                           │
│      let x = 42;        ← x lives on main's frame     │
│      let y = 3.14;      ← y lives on main's frame     │
│      add(x, y);                                        │
│  }                       ← frame popped, x and y gone  │
│                                                        │
│  fn add(a: i32, b: f64) {                              │
│      let sum = a as f64 + b; ← a, b, sum on add's     │
│  }                            frame ← popped on return │
│                                                        │
│  Stack layout (grows downward):                        │
│  ┌─────────────┐                                       │
│  │ main's frame│  x = 42, y = 3.14                     │
│  ├─────────────┤                                       │
│  │ add's frame │  a = 42, b = 3.14, sum = 45.14        │
│  └─────────────┘                                       │
│                                                        │
│  Properties:                                           │
│  • Allocation: instant (just move the stack pointer)   │
│  • Deallocation: instant (pop the frame)               │
│  • Size: must be known at compile time                 │
│  • Access: extremely fast (CPU cache-friendly)         │
│                                                        │
└──────────────────────────────────────────────────────┘
```

**What lives on the stack:** integers, floats, booleans, chars, fixed-size arrays, tuples of stack types, and **pointers** to heap data.

### The Heap

The heap is flexible, manual(ish) memory. It's for data whose size isn't known at compile time, or that needs to outlive the function that created it.

```
┌──────────────────────────────────────────────────────┐
│                    THE HEAP                            │
├──────────────────────────────────────────────────────┤
│                                                        │
│  let name = String::from("Achilles");                  │
│                                                        │
│  Stack:                    Heap:                        │
│  ┌──────────────┐         ┌───────────────────┐        │
│  │ name         │────────→│ A c h i l l e s   │        │
│  │  ptr: 0x7f.. │         └───────────────────┘        │
│  │  len: 8      │                                      │
│  │  capacity: 8 │                                      │
│  └──────────────┘                                      │
│                                                        │
│  The variable `name` is on the STACK. It contains:     │
│  • A pointer to heap memory                            │
│  • The length (8 bytes of actual content)               │
│  • The capacity (8 bytes allocated)                    │
│                                                        │
│  The actual string data ("Achilles") is on the HEAP.   │
│                                                        │
│  Properties:                                           │
│  • Allocation: slower (OS must find free space)        │
│  • Deallocation: must be explicit (in Rust: automatic  │
│    when the owner goes out of scope)                   │
│  • Size: can be dynamic (grow, shrink)                 │
│  • Access: slower (pointer indirection, cache misses)  │
│                                                        │
└──────────────────────────────────────────────────────┘
```

**What lives on the heap:** `String`, `Vec<T>`, `Box<T>`, `HashMap<K, V>` — anything that can grow at runtime.

### The Critical Insight

In C, you allocate heap memory with `malloc()` and free it with `free()`. Forget to free → memory leak. Free twice → crash. Use after free → security vulnerability (CVE-level stuff).

In Rust, the compiler tracks ownership. When the owner of heap data goes out of scope, Rust automatically calls `drop()` (Rust's destructor). 
No `free()`. No garbage collector. **Zero-cost memory safety.**

---

## Part 2: Ownership Rules

Rust's ownership system has three rules. Memorize them:

```
┌──────────────────────────────────────────────────────┐
│              THE THREE RULES OF OWNERSHIP              │
├──────────────────────────────────────────────────────┤
│                                                        │
│  1. Each value in Rust has exactly ONE owner.           │
│                                                        │
│  2. There can only be ONE owner at a time.              │
│                                                        │
│  3. When the owner goes out of scope, the value         │
│     is dropped (memory freed).                         │
│                                                        │
└──────────────────────────────────────────────────────┘
```

### What "move" means

```rust
fn main() {
    let s1 = String::from("Achilles");
    let s2 = s1;       // s1 is MOVED to s2

    println!("{}", s1); // ❌ COMPILE ERROR: value used after move
}
```

**What happened:** `s1` was a `String` — it owned heap data. When you wrote `let s2 = s1`, ownership **moved** from `s1` to `s2`. 
After the move, `s1` is invalid. It's not `null` — it doesn't exist. The compiler won't let you use it.

```
Before move:               After move:
Stack:     Heap:            Stack:      Heap:
┌────┐    ┌──────────┐     ┌────┐
│ s1 │───→│ Achilles │     │ s1 │  (INVALID — moved)
└────┘    └──────────┘     └────┘
                            ┌────┐    ┌──────────┐
                            │ s2 │───→│ Achilles │
                            └────┘    └──────────┘
```

There is ONE copy of the string on the heap. Ownership moved from `s1` to `s2`. No double-free is possible because only `s2` will call `drop()`.

### Copy vs Move

**Stack types** (integers, booleans, floats, chars) implement the `Copy` trait. They are cheap to copy (just copy the bits), so assignment copies rather than moves:

```rust
let x = 42;
let y = x;      // x is COPIED, not moved
println!("{x}"); // ✅ Works fine — x still valid
```

**Heap types** (`String`, `Vec<T>`, etc.) do NOT implement `Copy`. They are *moved* by default because copying heap data is expensive 
(you'd need to allocate new heap memory and copy all the bytes).

| Type | Default behavior on assignment | Why |
|------|-------------------------------|-----|
| `i32`, `f64`, `bool`, `char` | **Copy** (bitwise copy) | Small, fixed-size, cheap to duplicate |
| `String`, `Vec<T>`, `Box<T>` | **Move** (transfer ownership) | Heap-allocated, expensive to duplicate |

If you *want* to copy a heap type, you must explicitly call `.clone()`:

```rust
let s1 = String::from("Achilles");
let s2 = s1.clone();    // Explicit deep copy — allocates new heap memory
println!("{s1}");        // ✅ Works — s1 was cloned, not moved
```

`.clone()` is explicit because it's expensive — it allocates new heap memory and copies all the data. Rust makes you opt-in to expensive operations.

---

## Part 3: Ownership and Functions

When you pass a value to a function, it's either **moved** or **copied**, just like assignment:

```rust
fn main() {
    let name = String::from("Achilles");
    print_name(name);           // name is MOVED into the function

    println!("{}", name);       // ❌ COMPILE ERROR: value used after move
}

fn print_name(n: String) {     // n now owns the String
    println!("Name: {}", n);
}                               // n goes out of scope, String is dropped
```

The function took ownership. After the call, `name` is invalid. The String was dropped when `print_name` returned.

**To keep using the value after the function call, you have three options:**

**Option 1: Return ownership back**

```rust
fn main() {
    let name = String::from("Achilles");
    let name = print_and_return(name);  // Moved in, returned back
    println!("{}", name);               // ✅ Works — we got it back
}

fn print_and_return(n: String) -> String {
    println!("Name: {}", n);
    n   // Return ownership to the caller
}
```

This works but is cumbersome. You'll learn a better way on Day 2: **borrowing** (`&`).

**Option 2: Clone before passing**

```rust
fn main() {
    let name = String::from("Achilles");
    print_name(name.clone());   // Pass a clone, keep the original
    println!("{}", name);       // ✅ Works — we still own the original
}
```

Works, but wasteful — two copies of the string exist on the heap.

**Option 3: Borrow (Day 2)**

```rust
fn main() {
    let name = String::from("Achilles");
    print_name(&name);          // Lend it — don't give it away
    println!("{}", name);       // ✅ Works — name was borrowed, not moved
}

fn print_name(n: &String) {    // n is a REFERENCE — it borrows, doesn't own
    println!("Name: {}", n);
}                               // n goes out of scope, but it didn't own the data
                                // so nothing is dropped
```

This is the right approach — covered in depth on Day 2.

---

## Part 4: Scope and Drop

When a variable goes out of scope, Rust drops it:

```rust
fn main() {
    {
        let s = String::from("temporary");
        println!("{}", s);    // ✅ s is valid here
    }                          // s goes out of scope → drop() called → heap memory freed

    // println!("{}", s);     // ❌ s doesn't exist here
}
```

This applies to everything: `String`, `Vec<T>`, file handles, network connections, mutex locks. When the owner goes out of scope, the resource 
is released. This is called **RAII** (Resource Acquisition Is Initialization) and it's one of Rust's superpowers.

**Why this matters for Achilles:** When a subprocess node finishes execution and its output is processed, the output buffer goes out of scope and is 
automatically freed. No memory leaks. No manual cleanup. The compiler guarantees it.

---

## Part 5: Exercises

### Exercise 1 — Install and start Rustlings

Rustlings is an official Rust learning tool — small exercises where you fix broken code to make it compile.

```bash
cargo install rustlings
rustlings init
cd rustlings
```

Run the interactive mode:

```bash
rustlings
```

This presents exercises one at a time. Fix each one and press Enter to move to the next.

**Complete all exercises through `move_semantics`:**

The exercise categories you'll go through, in order:

| Category | What you learn | Count |
|----------|---------------|-------|
| `intro` | How rustlings works | 1 |
| `variables` | `let`, `mut`, shadowing, type annotations | 6 |
| `functions` | Parameters, return values, statements vs expressions | 5 |
| `if` | Conditionals, `if` as an expression | 3 |
| `primitive_types` | Tuples, arrays, slices, indexing | 6 |
| `vecs` | `Vec<T>` creation, indexing, iteration | 2 |
| `move_semantics` | **The core of today** — ownership, moves, references | 6 |

**After each exercise, write one sentence** in a scratch file (`/tmp/rustlings_notes.md`) explaining what the compiler was preventing and why. For example:

```markdown
## variables1
The compiler prevented using an uninitialized variable.
Accessing uninitialized memory is undefined behavior in C — Rust catches it at compile time.

## move_semantics1
The compiler prevented using a Vec after it was moved into a function.
The function took ownership — the original variable is no longer valid.
```

This forces you to think about *why* the compiler complained, not just how to make it compile.

---

### Exercise 2 — Ownership in action: The classic move trap

Create a new file in your Achilles project for experimentation:

```bash
mkdir -p ~/Antigravity/Achilles/src/bin
```

Create `~/Antigravity/Achilles/src/bin/day1_ownership.rs`:

```rust
// Day 1: Ownership — understanding moves
//
// Run with: cargo run --bin day1_ownership

fn main() {
    // ----- PART A: The move trap -----
    let target = String::from("10.10.10.100");
    
    // This function "consumes" target — takes ownership
    validate_target(target);
    
    // Uncomment the line below. Read the compiler error.
    // println!("Scanning target: {}", target);
    
    // TASK: Make this work by using ONE of the three techniques:
    //   1. Return ownership from validate_target
    //   2. Clone target before passing
    //   3. (Preview of Day 2) Pass a reference with &
    //
    // Try all three. Understand the trade-offs.
    
    
    // ----- PART B: Copy types vs Move types -----
    let port: u16 = 8080;
    let port_copy = port;      // Copy — both are valid
    println!("Original port: {}", port);     // ✅ Works
    println!("Copied port: {}", port_copy);  // ✅ Works
    
    let service = String::from("http");
    let service_moved = service;   // Move — service is now invalid
    // println!("Service: {}", service);  // ❌ Uncomment to see the error
    println!("Moved service: {}", service_moved);
    
    
    // ----- PART C: Ownership with Vec -----
    let ports = vec![22, 80, 443, 8080, 8443];
    
    // TASK: Call print_ports with ports, then try to use ports again.
    //       Fix it using clone or by returning ownership.
    print_ports(ports);
    // println!("Port count: {}", ports.len()); // ❌ Uncomment to see the error
    
    
    // ----- PART D: Scope and automatic drop -----
    {
        let scan_output = String::from("PORT   STATE SERVICE\n22/tcp open  ssh\n80/tcp open  http");
        println!("Inside scope:\n{}", scan_output);
    }  // scan_output is dropped here — memory freed
    
    // println!("{}", scan_output);  // ❌ Uncomment — scan_output doesn't exist
    
    
    // ----- PART E: Multiple moves -----
    let host = String::from("scanme.nmap.org");
    let host2 = host;          // Move 1: host → host2
    let host3 = host2;         // Move 2: host2 → host3
    // Both host and host2 are now invalid. Only host3 owns the data.
    println!("Final owner: {}", host3);
}

fn validate_target(target: String) {
    if target.is_empty() {
        println!("ERROR: empty target");
    } else {
        println!("Target '{}' is valid ({} chars)", target, target.len());
    }
}  // target is dropped here

fn print_ports(ports: Vec<u16>) {
    print!("Ports: ");
    for port in &ports {
        print!("{} ", port);
    }
    println!();
}  // ports is dropped here
```

**Run it:**

```bash
cargo run --bin day1_ownership
```

**Tasks (do each one):**

1. Uncomment the marked lines one at a time. Read each compiler error **carefully** — Rust's errors are the best error messages in any language. They tell 
you exactly what went wrong and often suggest the fix.

2. Fix Part A using all three techniques (one at a time):
   - Return ownership: change `validate_target` to return `String`
   - Clone: use `target.clone()`
   - Reference: change to `fn validate_target(target: &String)` and pass `&target`

3. Fix Part C so you can call `print_ports` AND use `ports` afterward.

---

### Exercise 3 — Stack vs Heap: See it yourself

Create `~/Antigravity/Achilles/src/bin/day1_memory.rs`:

```rust
// Day 1: Stack vs Heap — observing where data lives
//
// Run with: cargo run --bin day1_memory

fn main() {
    // ----- Stack data -----
    let port: u16 = 443;
    let timeout: f64 = 30.0;
    let verbose: bool = true;
    
    // These variables live on the stack.
    // Their size is known at compile time:
    //   u16  = 2 bytes
    //   f64  = 8 bytes
    //   bool = 1 byte
    println!("=== Stack Data ===");
    println!("port: {} (u16, {} bytes on stack)", port, std::mem::size_of::<u16>());
    println!("timeout: {} (f64, {} bytes on stack)", timeout, std::mem::size_of::<f64>());
    println!("verbose: {} (bool, {} bytes on stack)", verbose, std::mem::size_of::<bool>());
    
    // Pointer sizes (on your 64-bit system)
    println!("\nPointer size: {} bytes", std::mem::size_of::<&str>());
    
    // ----- Heap data -----
    let target = String::from("10.10.10.100");
    let ports = vec![22, 80, 443, 8080, 8443];
    
    // String and Vec store a POINTER on the stack, DATA on the heap.
    println!("\n=== Heap Data (stack part) ===");
    println!("String struct size on stack: {} bytes", std::mem::size_of::<String>());
    println!("  (pointer + length + capacity = 8 + 8 + 8 = 24 bytes on 64-bit)");
    println!("Vec<u16> struct size on stack: {} bytes", std::mem::size_of::<Vec<u16>>());
    
    println!("\n=== Heap Data (actual content) ===");
    println!("target '{}': {} bytes of content on heap", target, target.len());
    println!("ports {:?}: {} elements × {} bytes = {} bytes on heap",
        ports, ports.len(), std::mem::size_of::<u16>(),
        ports.len() * std::mem::size_of::<u16>());
    
    // ----- The difference matters -----
    println!("\n=== Copy vs Move ===");
    
    // Copy: stack data is cheap to duplicate
    let a: i32 = 42;
    let b = a;     // Copies 4 bytes on the stack — instant
    println!("Copied i32: a={}, b={}", a, b);
    
    // Move: heap data transfer ownership (no data copied)
    let s1 = String::from("Achilles");
    let s2 = s1;   // Only copies 24 bytes of stack metadata — the heap data doesn't move
    // s1 is invalidated — but the actual string bytes are still at the same heap address
    println!("Moved String: s2={}", s2);
    // println!("s1={}", s1);  // ❌ s1 is invalid
    
    // Clone: explicit deep copy — allocates new heap memory
    let s3 = s2.clone();
    println!("Cloned: s2={}, s3={} (two separate heap allocations)", s2, s3);
    
    // ----- Why this matters for Achilles -----
    println!("\n=== Achilles Context ===");
    println!("When nmap produces 10MB of XML output:");
    println!("  Move: transfers the 24-byte pointer — O(1), instant");
    println!("  Clone: copies 10MB of heap data — O(n), expensive");
    println!("  In Achilles pipeline: data moves between stages, never cloned unnecessarily");
}
```

**Run it:**

```bash
cargo run --bin day1_memory
```

Study the output. Understand:
- Stack types are small, fixed-size, and copied on assignment
- Heap types store a pointer on the stack and data on the heap
- Move transfers the pointer (24 bytes), not the data (could be megabytes)
- Clone explicitly copies everything — use it only when you need two independent copies

---

### Exercise 4 — Ownership puzzles: Predict then verify

Create `~/Antigravity/Achilles/src/bin/day1_puzzles.rs`:

For each puzzle, **predict** whether it compiles before running it. Then verify.

```rust
// Day 1: Ownership puzzles
//
// For each section: PREDICT whether it compiles, then uncomment and verify.
// Run with: cargo run --bin day1_puzzles

fn main() {
    // ----- Puzzle 1: Does this compile? -----
    let tools = vec!["nmap", "subfinder", "httpx"];
    let first = tools[0];      // What type is first? &str — Copy!
    println!("tools: {:?}", tools);
    println!("first: {}", first);
    // ANSWER: Yes! tools[0] returns a &str (a reference to a string literal).
    // &str implements Copy, so tools is not moved.

    // ----- Puzzle 2: Does this compile? -----
    let tools = vec![
        String::from("nmap"),
        String::from("subfinder"),
        String::from("httpx"),
    ];
    // let first = tools[0];   // Uncomment — does it compile?
    // PREDICT:
    // ANSWER: No! tools[0] would try to move a String out of the Vec.
    //         You can't move an element out of a container without
    //         leaving a "hole". Use &tools[0] or tools[0].clone() instead.

    // ----- Puzzle 3: Does this compile? -----
    let mut data = String::from("scan result");
    let data2 = data;
    data = String::from("new scan result");  // Re-assigning data — is this allowed?
    println!("data: {}", data);
    println!("data2: {}", data2);
    // PREDICT:
    // ANSWER: Yes! After data was moved, we assigned a NEW String to data.
    //         data now owns a completely different String. data2 owns the original.

    // ----- Puzzle 4: Does this compile? -----
    let names = vec![String::from("alpha"), String::from("bravo")];
    for name in names {
        println!("{}", name);
    }
    // println!("{:?}", names);  // Uncomment — does it compile?
    // PREDICT:
    // ANSWER: No! `for name in names` moves each element out of names.
    //         After the loop, names is consumed (moved).
    //         Use `for name in &names` to borrow instead.

    // ----- Puzzle 5: Does this compile? -----
    let scan_output = get_output();
    println!("Got output: {} bytes", scan_output.len());
    // PREDICT:
    // ANSWER: Yes! get_output() creates a String and returns it.
    //         Ownership moves from the function to the caller.
    //         This is how functions "give" data to their callers.
}

fn get_output() -> String {
    let result = String::from("PORT   STATE SERVICE\n22/tcp open  ssh");
    result  // Ownership moves to caller — no copy, no clone
}
```

**Run it:**

```bash
cargo run --bin day1_puzzles
```

Uncomment each marked line one at a time. Predict. Verify. Read the error messages.

---

## Part 6: Key Concepts to Internalize

| Question | Your Answer Should Include |
|----------|--------------------------|
| What are Rust's three ownership rules? | 1. Each value has one owner. 2. Only one owner at a time. 3. Value is dropped when owner goes out of scope. |
| What is a "move"? | Transfer of ownership from one variable to another. The source variable becomes invalid. |
| What's the difference between move and copy? | Copy duplicates the bits (stack types). Move transfers ownership without copying heap data. |
| Which types are Copy? | Primitive types: integers, floats, bool, char, tuples/arrays of Copy types, `&T` references |
| Which types are Move? | Heap-allocated types: `String`, `Vec<T>`, `Box<T>`, `HashMap<K,V>` — anything with heap data |
| What does `.clone()` do? | Explicitly creates a deep copy — allocates new heap memory and copies all data. Opt-in expensive. |
| What is `drop()`? | Rust's destructor — called automatically when an owner goes out of scope. Frees heap memory. |
| What is RAII? | Resource Acquisition Is Initialization — resources are freed when their owner goes out of scope |
| What does a `String` look like in memory? | 24 bytes on the stack (pointer + length + capacity) pointing to variable-length data on the heap |
| Why does Achilles need ownership? | Concurrent subprocess I/O — ownership prevents data races at compile time, no runtime cost |

---

## Completion Checklist

- [X] Rustlings: completed all exercises through `move_semantics` (variables, functions, if, primitive_types, vecs, move_semantics)
- [X] I wrote one sentence per exercise explaining what the compiler prevented
- [X] `day1_ownership.rs`: Fixed all ownership errors using return, clone, and reference techniques
- [X] `day1_memory.rs`: Can explain the difference between stack and heap, and know what `String` looks like in memory
- [X] `day1_puzzles.rs`: Predicted correctly for at least 3 of 5 puzzles
- [X] I can explain ownership, move, copy, clone, and drop from memory
- [X] I understand why `let s2 = s1` invalidates `s1` for heap types but not stack types
- [X] I've read at least 5 Rust compiler error messages carefully — they are documentation, not noise

---

## Lesson

> Rust's ownership system is not a tax. It's a guarantee. Every bug it prevents — use-after-free, double-free, data race — is a bug that in C would be a 
> CVE, and in Python would be a silent 3 AM crash. The compiler is your most rigorous reviewer. When it rejects your code, **it's right.** Your job is to understand why.

---

**Next: Day 2 — Borrowing, References, Lifetimes, `String` vs `&str`, `Result<T, E>`, and the `?` Operator →**
