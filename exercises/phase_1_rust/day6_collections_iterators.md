# Phase 1, Day 6 -- Vec, HashMap, Iterators, Closures, and Option<T>

> **Time budget:** 6-8 hours of focused work.
> **Prerequisite:** Day 5 complete. You can define traits, implement them
> for multiple structs, use generics with trait bounds, explain static vs
> dynamic dispatch, and build trait objects for heterogeneous collections.
> **Outcome:** You can manipulate collections with iterator chains, group
> data with HashMap, transform ADC objects using closures, and use
> Option<T> combinators to eliminate unnecessary match blocks.

> [!IMPORTANT]
> Data flows through Achilles as typed collections.
> Nmap produces `Vec<Host>`. Nuclei produces `Vec<Finding>`.
> The merge node receives these, deduplicates by IP, groups by
> severity, sorts by port number -- all with iterator chains.
> The transformer takes raw XML and produces a `HashMap<String,
> Vec<Port>>` grouping ports by host.
> Today you learn the tools to build those transformations.

---

## Why This Day Matters

Look at how data moves inside Achilles (from the architecture doc):

```rust
// Nuclei node reading hosts from nmap
fn execute(&self, input: &dyn NodeInput) -> Result<Vec<Finding>> {
    let hosts: Vec<Host> = input.get("nmap_scan");
    let urls: Vec<String> = hosts.iter()
        .flat_map(|h| h.web_urls())
        .collect();
    // Run nuclei against these URLs...
}
```

That `.iter().flat_map().collect()` chain is not optional syntax
sugar. It is how every node in the DAG transforms its input before
passing it downstream. Without iterator fluency, you cannot build
a single transformer.

---

## Part 1: Vec -- The Growable Array

You have already used `Vec` in Day 5 (the `Vec<Box<dyn Executable>>`).
Now learn it properly.

### Creating a Vec

```rust
// Empty, type inferred from later usage
let mut hosts: Vec<String> = Vec::new();

// From a literal list
let ports = vec![80, 443, 8080, 8443];

// Pre-allocated (no reallocation until capacity exceeded)
let mut results: Vec<String> = Vec::with_capacity(100);
```

### Core methods

```rust
let mut v = vec![1, 2, 3];

v.push(4);           // [1, 2, 3, 4]      -- append to end
v.pop();             // Some(4), v is [1, 2, 3] -- remove from end
v.insert(0, 99);     // [99, 1, 2, 3]     -- insert at index (shifts right)
v.remove(0);         // 99, v is [1, 2, 3] -- remove at index (shifts left)
v.len();             // 3                  -- number of elements
v.is_empty();        // false
v.contains(&2);      // true
v[1];                // 2                  -- index access (panics if OOB)
v.get(1);            // Some(&2)           -- safe access (no panic)
v.get(99);           // None               -- out of bounds returns None
```

### push/pop are O(1), insert/remove are O(n)

`push` and `pop` work at the **end** of the Vec -- no data moves.
`insert` and `remove` shift all elements after the index. Inserting
at index 0 in a 1000-element Vec moves 1000 elements. Use them
sparingly on large Vecs.

If you need to remove an element and do not care about order,
`swap_remove(index)` is O(1) -- it swaps the element with the last
one and pops.

### Sorting and deduplicating

```rust
let mut names = vec!["charlie", "alice", "bob", "alice"];

names.sort();        // ["alice", "alice", "bob", "charlie"]
names.dedup();       // ["alice", "bob", "charlie"]
```

`dedup()` only removes **consecutive** duplicates. It does NOT scan
the whole Vec. You MUST sort first, then dedup. If you skip the
sort, duplicates that are not adjacent survive.

For structs, use `sort_by_key`:

```rust
let mut ports: Vec<Port> = get_ports();
ports.sort_by_key(|p| p.number);  // sort by port number
ports.dedup_by_key(|p| p.number); // dedup by port number
```

### len() vs capacity()

```rust
let mut v = Vec::with_capacity(10);
v.push(1);
v.push(2);

v.len();       // 2   -- how many elements are stored
v.capacity();  // 10  -- how many elements can fit before reallocation
```

`Vec` is backed by a heap-allocated buffer. When you `push` past
capacity, the Vec allocates a new buffer (typically 2x the old size),
copies everything over, and frees the old buffer. This is why
`Vec::with_capacity(n)` exists -- if you know you will have 500 hosts,
pre-allocate to avoid repeated reallocations.

---

## Part 2: The Three Iterators

Every collection in Rust has three ways to iterate. This is critical.

### `.iter()` -- borrow the elements

```rust
let names = vec!["alice", "bob", "charlie"];

for name in names.iter() {
    // name is &&str (a reference to the element)
    // names is NOT consumed -- usable after the loop
    println!("{}", name);
}

println!("{:?}", names); // still works
```

### `.iter_mut()` -- mutably borrow the elements

```rust
let mut counts = vec![1, 2, 3];

for count in counts.iter_mut() {
    // count is &mut i32 (a mutable reference)
    *count *= 2;  // dereference to modify
}

// counts is now [2, 4, 6]
```

### `.into_iter()` -- consume the collection

```rust
let names = vec!["alice".to_string(), "bob".to_string()];

for name in names.into_iter() {
    // name is String (owned, moved out of the Vec)
    println!("{}", name);
}

// println!("{:?}", names); // COMPILE ERROR -- names is consumed
```

### The `for` loop shorthand

```rust
for x in &v      { }  // same as v.iter()       -- borrows
for x in &mut v  { }  // same as v.iter_mut()    -- mutable borrows
for x in v       { }  // same as v.into_iter()   -- consumes
```

This is the `&tools` you used in Day 5's `run_all` --
`for tool in &tools` borrows the Vec, leaving it intact.

---

## Part 3: Iterator Adaptors -- The Pipeline

Iterators are **lazy**. Calling `.filter()` or `.map()` does NOT
execute anything. It builds a pipeline. Execution only happens when
you call a **consuming method** like `.collect()`, `.for_each()`,
`.count()`, or `.sum()`.

### .map() -- transform each element

```rust
let numbers = vec![1, 2, 3, 4];
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .collect();
// [2, 4, 6, 8]
```

`.map()` takes a closure. The closure receives each element and
returns the transformed version. The original Vec is untouched.

### .filter() -- keep matching elements

```rust
let ports = vec![22, 80, 443, 3306, 8080];
let web_ports: Vec<&i32> = ports.iter()
    .filter(|p| **p == 80 || **p == 443 || **p == 8080)
    .collect();
// [&80, &443, &8080]
```

`.filter()` takes a closure that returns `bool`. The closure receives
`&&i32` (a reference to a reference) because `.iter()` yields
references, and `.filter()` borrows those references. That is why
the double dereference `**p`. This is ugly but correct.

Cleaner with pattern matching:

```rust
let web_ports: Vec<&i32> = ports.iter()
    .filter(|&&p| p == 80 || p == 443 || p == 8080)
    .collect();
```

The `|&&p|` destructures through both layers of reference.

### .filter_map() -- filter and transform in one step

```rust
let values = vec![Some(1), None, Some(3), None, Some(5)];
let extracted: Vec<i32> = values.into_iter()
    .filter_map(|x| x)
    .collect();
// [1, 3, 5]
```

`.filter_map()` takes a closure that returns `Option<T>`. It keeps
the `Some` values (unwrapped) and discards `None`. This is the
idiomatic way to extract specific enum variants from a collection:

```rust
let hosts: Vec<&Host> = objects.iter()
    .filter_map(|obj| match obj {
        ADCObject::Host(h) => Some(h),
        _ => None,
    })
    .collect();
```

### .flat_map() -- map and flatten

```rust
let groups = vec![
    vec!["10.0.0.1", "10.0.0.2"],
    vec!["192.168.1.1"],
];
let all_ips: Vec<&&str> = groups.iter()
    .flat_map(|group| group.iter())
    .collect();
// ["10.0.0.1", "10.0.0.2", "192.168.1.1"]
```

`.flat_map()` is `.map()` followed by `.flatten()`. Each element
produces an iterator, and all those iterators are flattened into one.
This is the `hosts.iter().flat_map(|h| h.web_urls())` pattern from
the architecture doc.

### .enumerate() -- index + element

```rust
let tools = vec!["nmap", "nuclei", "ffuf"];
for (i, tool) in tools.iter().enumerate() {
    println!("[{}] {}", i, tool);
}
// [0] nmap
// [1] nuclei
// [2] ffuf
```

### .chain() -- concatenate iterators

```rust
let nmap_hosts = vec!["10.0.0.1", "10.0.0.2"];
let subfinder_hosts = vec!["10.0.0.3"];

let all: Vec<&&str> = nmap_hosts.iter()
    .chain(subfinder_hosts.iter())
    .collect();
// ["10.0.0.1", "10.0.0.2", "10.0.0.3"]
```

### .take() and .skip()

```rust
let data = vec![1, 2, 3, 4, 5];

let first_3: Vec<&i32> = data.iter().take(3).collect();
// [1, 2, 3]

let after_2: Vec<&i32> = data.iter().skip(2).collect();
// [3, 4, 5]
```

---

## Part 4: Consuming Methods

These drive the pipeline and produce a final result.

### .collect() -- the universal converter

`.collect()` consumes the iterator and builds a collection.
It can produce `Vec`, `String`, `HashMap`, `HashSet`, and more.
You must tell Rust what type to collect into.

```rust
// Into a Vec
let v: Vec<i32> = (1..=5).collect();

// Into a String
let s: String = vec!['h', 'e', 'l', 'l', 'o'].into_iter().collect();

// Using turbofish syntax (type on collect instead of variable)
let v = (1..=5).collect::<Vec<i32>>();
```

The turbofish `::<Type>` is needed when the compiler cannot infer
the collection type from context.

### .fold() -- accumulate a result

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum = numbers.iter().fold(0, |acc, x| acc + x);
// sum = 15
```

`.fold()` takes an initial value (0) and a closure that receives
the accumulator and each element. It processes every element and
returns the final accumulator.

`.sum()` is a shorthand for the common case: `numbers.iter().sum::<i32>()`

### .any() and .all()

```rust
let ports = vec![22, 80, 443];

let has_http = ports.iter().any(|&p| p == 80);      // true
let all_above_20 = ports.iter().all(|&p| p > 20);   // true
```

These short-circuit -- `.any()` stops at the first true,
`.all()` stops at the first false.

### .find() -- first match

```rust
let ports = vec![22, 80, 443, 8080];

let first_web = ports.iter().find(|&&p| p >= 80 && p <= 443);
// Some(&80)
```

Returns `Option<&T>` -- `None` if no match.

### .count()

```rust
let ports = vec![22, 80, 443, 3306, 8080];
let privileged = ports.iter().filter(|&&p| p < 1024).count();
// 3
```

---

## Part 5: Closures -- Anonymous Functions

You have already used closures: `|x| x * 2` inside `.map()`.
Now understand them properly.

### Syntax

```rust
// Full annotation
let add = |a: i32, b: i32| -> i32 { a + b };

// Type-inferred (compiler figures it out from usage)
let add = |a, b| a + b;

// No parameters
let greet = || println!("hello");

// Multi-line body
let process = |x: i32| {
    let doubled = x * 2;
    let tripled = x * 3;
    doubled + tripled
};
```

### Capture modes

Closures capture variables from their enclosing scope.
Rust automatically chooses the cheapest capture mode that works.

```rust
let name = String::from("nmap");

// Captures by reference (&name) -- Fn
let print_name = || println!("{}", name);
print_name();
print_name();  // can call multiple times
println!("{}", name);  // name is still alive

// Captures by mutable reference (&mut count) -- FnMut
let mut count = 0;
let mut increment = || count += 1;
increment();
increment();
// count is now 2

// Captures by value (moves name into closure) -- FnOnce
// NOTE: it is the drop(name) that makes this FnOnce, NOT the move keyword.
// move only transfers ownership. If the closure only READS the owned
// value without consuming it, it would still be Fn (callable many times).
let name = String::from("nmap");
let consume = move || {
    println!("{}", name);
    drop(name);  // THIS is what makes it FnOnce -- name is consumed
};
consume();
// consume();  // ERROR -- closure consumed name, cannot call again
// println!("{}", name);  // ERROR -- name was moved into the closure
```

### The three traits

The compiler auto-implements these based on how the closure
uses its captures:

```
Fn       -- only reads captures (&T)
             can be called unlimited times
             .map(), .filter(), .for_each() use this

FnMut    -- mutates captures (&mut T)
             can be called multiple times
             .fold() uses this (accumulator is mutated)

FnOnce   -- consumes captures (T)
             can be called ONCE
             .unwrap_or_else() uses this
```

Hierarchy: `Fn` is a subset of `FnMut`, which is a subset of `FnOnce`.
Every `Fn` closure is also `FnMut` and `FnOnce`. But not the reverse.

For Day 6, you mostly use `Fn` closures (inside `.map()` and
`.filter()`). You do not need to annotate these traits -- the compiler
infers them. Just know they exist.

### The `move` keyword

`move` forces the closure to take ownership of all captured variables:

```rust
let name = String::from("nmap");
let closure = move || println!("{}", name);
// name is now OWNED by the closure
// println!("{}", name);  // ERROR -- name was moved
```

Why: when the closure outlives the current scope. Thread spawning
is the classic example:

```rust
let data = String::from("hello");
std::thread::spawn(move || {
    // This closure runs on a DIFFERENT thread
    // If it only borrowed &data, data might be dropped
    // before the thread finishes -- dangling reference
    // move forces ownership transfer -- data lives as long
    // as the closure does
    println!("{}", data);
});
```

You will use `move` closures heavily with Tokio on Day 10.

---

## Part 6: HashMap -- Key-Value Storage

```rust
use std::collections::HashMap;
```

HashMap is NOT in the prelude. You must import it.

### Creating and using

```rust
let mut scan_results: HashMap<String, Vec<u16>> = HashMap::new();

// Insert
scan_results.insert("10.0.0.1".to_string(), vec![22, 80, 443]);
scan_results.insert("10.0.0.2".to_string(), vec![8080]);

// Get -- returns Option<&V>
let ports = scan_results.get("10.0.0.1");
// Some(&[22, 80, 443])

let missing = scan_results.get("10.0.0.99");
// None

// Check existence
scan_results.contains_key("10.0.0.1");  // true

// Remove
scan_results.remove("10.0.0.2");

// Length
scan_results.len();  // 1
```

### Iterating

```rust
// Iterate over key-value pairs
for (ip, ports) in &scan_results {
    println!("{}: {:?}", ip, ports);
}

// Iterate over keys only
for ip in scan_results.keys() {
    println!("{}", ip);
}

// Iterate over values only
for ports in scan_results.values() {
    println!("{:?}", ports);
}
```

**WARNING:** HashMap iteration order is NOT guaranteed. If you
iterate twice, the order may differ. If you need ordered keys,
use `BTreeMap` instead.

### The Entry API -- avoid double lookups

Common pattern: "if the key exists, update the value. If not,
insert a default."

Bad (double lookup):

```rust
if scan_results.contains_key("10.0.0.1") {
    scan_results.get_mut("10.0.0.1").unwrap().push(3306);
} else {
    scan_results.insert("10.0.0.1".to_string(), vec![3306]);
}
```

This hashes "10.0.0.1" **twice** -- once for `contains_key`,
once for `get_mut`/`insert`.

Good (Entry API, single lookup):

```rust
scan_results
    .entry("10.0.0.1".to_string())
    .or_insert_with(Vec::new)
    .push(3306);
```

`.entry()` looks up the key ONCE. If it exists, returns a mutable
reference to the value. If not, inserts the default from
`or_insert_with(Vec::new)` and returns a mutable reference to that.
Then `.push(3306)` appends to whichever value was returned.

Three variants:

```
.or_insert(default)          -- always evaluates default (eager)
.or_insert_with(|| default)  -- evaluates only if key missing (lazy)
.or_default()                -- uses Default::default() (lazy)
```

Use `or_insert_with` for anything that allocates (like `Vec::new()`).
Use `or_insert` for cheap values (like `0`).

### and_modify() -- update existing values

```rust
scan_results
    .entry("10.0.0.1".to_string())
    .and_modify(|ports| ports.push(3306))
    .or_insert_with(|| vec![3306]);
```

`.and_modify()` runs a closure on the existing value IF the key
exists. Chainable with `.or_insert*` for the missing case.

Counting pattern:

```rust
let mut word_count: HashMap<&str, usize> = HashMap::new();
for word in words {
    word_count
        .entry(word)
        .and_modify(|count| *count += 1)
        .or_insert(1);
}
```

### Collecting into a HashMap

```rust
let pairs = vec![("nmap", 22), ("httpx", 80), ("nuclei", 443)];

let map: HashMap<&str, i32> = pairs.into_iter().collect();
```

The iterator must produce `(Key, Value)` tuples. `.collect()` builds
the HashMap from those pairs.

---

## Part 7: Grouping with HashMap -- The Merge Pattern

This is exactly what the Achilles merge node does.

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
}

fn group_by_ip(hosts: Vec<Host>) -> HashMap<String, Vec<Host>> {
    let mut groups: HashMap<String, Vec<Host>> = HashMap::new();

    for host in hosts {
        groups
            .entry(host.ip.clone())
            .or_insert_with(Vec::new)
            .push(host);
    }

    groups
}
```

Why `host.ip.clone()`:

`.entry()` takes ownership of the key (needs to store it if
inserting). But `host.ip` is a field of `host`, and you cannot
partially move out of a struct. So you clone the IP to give
`.entry()` its own copy, then `.push(host)` moves the whole
host into the Vec.

The architecture doc at section 6.3 describes the merge node
doing exactly this -- receiving `Vec<ADCObject>` from multiple
upstream nodes and grouping them by IP address into a
`HashMap<String, Vec<ADCObject>>`.

---

## Part 8: Option<T> -- No Nulls, Ever

Rust has no `null`, no `nullptr`, no `NullPointerException`.
Instead: `Option<T>`.

```rust
enum Option<T> {
    Some(T),   // value exists
    None,      // value does not exist
}
```

You have used this. `Host.hostname` is `Option<String>` -- some hosts
have hostnames, some do not. No null. No crashes. The compiler forces
you to handle both cases.

### Basic extraction

Each line below shows a **separate alternative**, not sequential
code. `unwrap()` consumes the Option, so you would pick ONE of
these approaches per use site:

```rust
let hostname: Option<String> = Some("target.htb".to_string());

// match -- always works, always safe
match &hostname {
    Some(name) => println!("Host: {}", name),
    None => println!("No hostname"),
}

// if let -- when you only care about one case
if let Some(name) = &hostname {
    println!("Host: {}", name);
}

// unwrap -- panics if None. DO NOT use in production.
// let name = hostname.unwrap();  // "target.htb" -- CONSUMES hostname

// unwrap_or -- fallback value (also consumes)
// let name = hostname.unwrap_or("unknown".to_string());

// unwrap_or_else -- lazy fallback with closure (also consumes)
// let name = hostname.unwrap_or_else(|| "unknown".to_string());

// In practice, use as_deref() to avoid consuming:
let name = hostname.as_deref().unwrap_or("unknown");  // borrows, no consumption
```

### Combinators -- eliminate match blocks

These are the idiomatic way to work with Option. They replace
nested `match` statements with chainable method calls.

### .map() -- transform the inner value

```rust
let port: Option<u16> = Some(443);

let doubled: Option<u16> = port.map(|p| p * 2);
// Some(886)

let none_port: Option<u16> = None;
let doubled: Option<u16> = none_port.map(|p| p * 2);
// None -- closure never runs
```

`.map()` on Option: if Some, apply the closure and wrap the result
in Some. If None, return None. The closure never executes on None.

### .and_then() -- chain operations that return Option

```rust
fn find_port(ip: &str) -> Option<u16> {
    if ip == "10.0.0.1" { Some(80) } else { None }
}

fn port_to_service(port: u16) -> Option<String> {
    match port {
        80 => Some("http".to_string()),
        443 => Some("https".to_string()),
        _ => None,
    }
}

let service = find_port("10.0.0.1")
    .and_then(port_to_service);
// Some("http")

let service = find_port("10.0.0.99")
    .and_then(port_to_service);
// None -- find_port returned None, port_to_service never called
```

`.and_then()` is `.map()` but the closure itself returns an Option.
It prevents `Option<Option<T>>` nesting.

Think of it as: "if the previous step succeeded, try this next step.
If any step returns None, the whole chain is None."

### .as_ref() and .as_deref() -- borrow inside Option

You already used `.as_deref()` in Day 5:

```rust
let hostname: Option<String> = Some("target.htb".to_string());

// as_ref: &Option<String> -> Option<&String>
let ref_hostname: Option<&String> = hostname.as_ref();

// as_deref: &Option<String> -> Option<&str>
let deref_hostname: Option<&str> = hostname.as_deref();

// Why: unwrap_or needs a matching type
hostname.as_deref().unwrap_or("unknown")  // zero allocation
// vs
hostname.clone().unwrap_or("unknown".to_string())  // allocates
```

`.as_deref()` converts `Option<String>` to `Option<&str>` by
applying `Deref` (String -> &str). Now `unwrap_or("unknown")` works
with a simple `&str` literal -- no `.to_string()` needed.

### .is_some() and .is_none() -- boolean checks

```rust
let hostname: Option<String> = Some("target.htb".to_string());

if hostname.is_some() {
    println!("has hostname");
}

if hostname.is_none() {
    println!("no hostname");
}
```

Prefer `if let Some(name) = ...` over `is_some()` + `unwrap()`.
`if let` gives you the value. `is_some()` only gives you a bool.

---

## Part 9: Putting It All Together -- Iterator Chains in Practice

### Example 1: Filter hosts with open HTTP ports

```rust
let hosts: Vec<Host> = get_scan_results();

let http_hosts: Vec<&Host> = hosts.iter()
    .filter(|h| h.ports.iter().any(|p| p.number == 80 || p.number == 443))
    .collect();
```

Read it as: "from all hosts, keep only those that have any port
with number 80 or 443."

### Example 2: Extract all unique IPs from scan results

```rust
use std::collections::HashSet;

let all_ips: HashSet<&str> = hosts.iter()
    .map(|h| h.ip.as_str())
    .collect();
```

Collecting into a `HashSet` automatically deduplicates.

### Example 3: Count findings by severity

```rust
let mut severity_counts: HashMap<&str, usize> = HashMap::new();

for finding in &findings {
    *severity_counts
        .entry(finding.severity.as_str())
        .or_insert(0) += 1;
}

// {"Critical": 2, "High": 5, "Medium": 12, "Low": 3}
```

This uses the Entry API: get the counter for this severity (or
insert 0), then dereference the mutable reference and increment.

### Example 4: Transform Vec<Host> to Vec<String> of "ip:port"

```rust
let targets: Vec<String> = hosts.iter()
    .flat_map(|h| {
        h.ports.iter().map(move |p| {
            format!("{}:{}", h.ip, p.number)
        })
    })
    .collect();
// ["10.0.0.1:80", "10.0.0.1:443", "10.0.0.2:8080"]
```

`.flat_map()` because each host produces multiple "ip:port" strings.
Without `flat_map`, you would get `Vec<Vec<String>>` -- nested.

---

## Part 10: Lifetimes -- A Brief Introduction

You will encounter lifetimes in today's Rustlings exercises.
This is NOT a deep dive -- that comes later. But you need enough
to solve the exercises.

### What is a lifetime?

A lifetime is the compiler's way of tracking "how long does this
reference stay valid?"

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}
```

This does NOT compile. The compiler asks: "the return type is `&str`.
Which input does it come from? `a` or `b`? I need to know so I can
guarantee the returned reference is valid."

### Lifetime annotations

You answer with lifetime annotations:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

`'a` is a lifetime parameter. It says: "the return value lives at
least as long as both `a` and `b`." The compiler uses this to ensure
you do not use the returned reference after either input is dropped.

### The rules (lifetime elision)

Most of the time you do not write lifetimes. The compiler infers
them. These are the elision rules:

1. Each input reference gets its own lifetime
2. If there is exactly one input lifetime, it is assigned to all outputs
3. If there is `&self` or `&mut self`, its lifetime is assigned to
   all outputs

```rust
fn first_word(s: &str) -> &str { ... }
// Compiler infers: fn first_word<'a>(s: &'a str) -> &'a str
// Rule 2: one input, so output gets the same lifetime

fn name(&self) -> &str { ... }
// Compiler infers: fn name<'a>(&'a self) -> &'a str
// Rule 3: &self, so output gets self's lifetime
```

When the rules cannot infer (like `longest` with two inputs),
you must annotate manually.

### Do not overthink lifetimes today

For Day 6, just know:
- Lifetimes exist to prevent dangling references
- The compiler usually infers them
- When it cannot, you annotate with `'a`
- The Rustlings exercises will teach the mechanics

---

## Part 11: Exercises

> [!IMPORTANT]
> These exercises are task-based.
> The code blocks show expected output and hints, NOT code to copy.
> You write the implementation from scratch.
> Run `cargo clippy` and `cargo fmt` before committing.
> Add `#![allow(dead_code)]` at the top of each exercise file
> to suppress warnings about unused struct fields.

---

### Exercise 1 -- Filter and sort hostnames

Create `~/Antigravity/Achilles/src/bin/day6_filter.rs`.

**Task:** Write a function that takes a `Vec<String>` of hostnames,
removes duplicates, sorts alphabetically, and returns `Vec<String>`.
No manual `for`/`while` loops -- use Vec methods (`.sort()`, `.dedup()`).

1. Define the function:

```rust
fn clean_hostnames(hostnames: Vec<String>) -> Vec<String>
```

2. The function must:
   - Remove duplicate hostnames
   - Sort alphabetically (case-insensitive sort: "Foo.com" and
     "foo.com" should be adjacent, with lowercase first)
   - Return the cleaned list

3. In `main()`, test with:

```rust
let hosts = vec![
    "target.htb".to_string(),
    "admin.target.htb".to_string(),
    "target.htb".to_string(),
    "mail.target.htb".to_string(),
    "admin.target.htb".to_string(),
    "dev.target.htb".to_string(),
    "mail.target.htb".to_string(),
];
```

**Expected output:**

```
Before: 7 hostnames
After: 4 hostnames
  admin.target.htb
  dev.target.htb
  mail.target.htb
  target.htb
```

**Hints:**
- `.sort()` sorts in place (mutates the Vec)
- `.dedup()` removes consecutive duplicates (sort first)
- You need a `mut` binding since sort/dedup modify in place
- Consider using `.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()))`
  for case-insensitive sorting

**Run:** `cargo run --bin day6_filter`

---

### Exercise 2 -- Group hosts by IP with HashMap

Create `~/Antigravity/Achilles/src/bin/day6_group.rs`.

**Task:** Write a function that takes `Vec<Host>` and groups them
into `HashMap<String, Vec<Host>>` keyed by IP address.

1. You will need this import:

```rust
use std::collections::HashMap;
```

2. Define the Host struct:

```rust
#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
    source: String,
}
```

3. Define the grouping function:

```rust
fn group_by_ip(hosts: Vec<Host>) -> HashMap<String, Vec<Host>>
```

4. In `main()`, create test data simulating what happens when
   multiple tools discover the same hosts:

```rust
let hosts = vec![
    Host { ip: "10.0.0.1".to_string(), hostname: Some("web.target.htb".to_string()), source: "subfinder".to_string() },
    Host { ip: "10.0.0.2".to_string(), hostname: None, source: "nmap".to_string() },
    Host { ip: "10.0.0.1".to_string(), hostname: None, source: "nmap".to_string() },
    Host { ip: "10.0.0.3".to_string(), hostname: Some("db.target.htb".to_string()), source: "subfinder".to_string() },
    Host { ip: "10.0.0.2".to_string(), hostname: Some("api.target.htb".to_string()), source: "subfinder".to_string() },
];
```

5. After grouping, print each IP and its associated hosts.

**Expected output:**

```
=== Hosts grouped by IP ===

10.0.0.1 (2 entries):
  - hostname: web.target.htb, source: subfinder
  - hostname: Unknown, source: nmap

10.0.0.2 (2 entries):
  - hostname: Unknown, source: nmap
  - hostname: api.target.htb, source: subfinder

10.0.0.3 (1 entries):
  - hostname: db.target.htb, source: subfinder
```

Note: HashMap iteration order is NOT guaranteed -- your IPs may
print in a different order. That is correct behavior.

**Hints:**
- Use the Entry API: `.entry(key).or_insert_with(Vec::new).push(host)`
- Clone the IP for the key: `host.ip.clone()`
- Use `.as_deref().unwrap_or("Unknown")` for hostname display

**Run:** `cargo run --bin day6_group`

---

### Exercise 3 -- Iterator chains on ADCObjects

Create `~/Antigravity/Achilles/src/bin/day6_iterators.rs`.

**Task:** Write multiple functions that process `Vec<ADCObject>`
using iterator chains. NO manual loops (no `for`, no `while`,
no `loop`). Every transformation must use iterator methods.

1. You will need these imports:

```rust
use std::collections::HashMap;
use std::collections::HashSet;
```

2. Reuse your ADCObject enum from Day 5:

```rust
#[derive(Debug, Clone)]
struct Host { ip: String, hostname: Option<String> }

#[derive(Debug, Clone)]
struct Port { number: u16, protocol: String, state: String, host_ip: String }

#[derive(Debug, Clone)]
struct Finding { title: String, severity: String, host_ip: String }

#[derive(Debug, Clone)]
enum ADCObject {
    Host(Host),
    Port(Port),
    Finding(Finding),
}
```

3. Write these functions (ALL using iterator chains, no loops):

```rust
// Extract all Host objects from a mixed Vec<ADCObject>
fn extract_hosts(objects: &[ADCObject]) -> Vec<&Host>

// Get all unique IPs from ADCObjects (hosts only)
fn unique_ips(objects: &[ADCObject]) -> Vec<String>

// Count findings by severity -> HashMap<String, usize>
fn count_by_severity(objects: &[ADCObject]) -> HashMap<String, usize>

// Get all ports above a threshold
fn ports_above(objects: &[ADCObject], threshold: u16) -> Vec<&Port>

// Get "ip:port" strings for all open ports
fn open_targets(objects: &[ADCObject]) -> Vec<String>
```

4. In `main()`, create a Vec with a mix of all types and call
each function, printing the results.

**Hints:**
- `extract_hosts`: `.iter().filter_map(|obj| match obj { ADCObject::Host(h) => Some(h), _ => None })`
- `unique_ips`: extract hosts, map to ip, collect into `HashSet`, convert to Vec
- `count_by_severity`: use `.fold()` with a HashMap accumulator
- `ports_above`: filter_map to extract Ports, then filter by number
- `open_targets`: filter_map for Ports, filter state == "OPEN", map to `format!("{}:{}", p.host_ip, p.number)`

**Run:** `cargo run --bin day6_iterators`

---

### Exercise 4 -- Option combinators

Create `~/Antigravity/Achilles/src/bin/day6_options.rs`.

**Task:** Write functions that use Option combinators instead of
match blocks.

1. Define:

```rust
#[derive(Debug)]
struct ScanResult {
    ip: String,
    hostname: Option<String>,
    os: Option<String>,
    service: Option<String>,
}
```

2. Write these functions using ONLY Option combinators (no match,
no if let, no unwrap):

```rust
// Return "hostname (ip)" if hostname exists, else just "ip"
fn display_target(result: &ScanResult) -> String

// Return the OS in uppercase if it exists, else "Unknown OS"
fn get_os_label(result: &ScanResult) -> String

// Return "service on ip" if both service and ip exist
// (ip always exists, service might not)
fn service_description(result: &ScanResult) -> String

// Chain: hostname -> uppercase -> first 10 chars -> with fallback
fn short_label(result: &ScanResult) -> String
```

3. In `main()`, test with multiple ScanResults (some with all
fields, some with None values).

**Expected output:**

```
=== Option Combinators ===

Target 1: web.target.htb (10.0.0.1)
Target 2: 10.0.0.2
OS 1: LINUX 5.4
OS 2: Unknown OS
Service 1: http on 10.0.0.1
Service 2: no service on 10.0.0.2
Label 1: WEB.TARGET
Label 2: UNKNOWN
```

**Hints:**
- `display_target`: use `.as_deref().map(|h| format!(...)).unwrap_or(...)`
- `get_os_label`: use `.as_deref().map(|os| os.to_uppercase()).unwrap_or_else(...)`
- `service_description`: use `.as_deref().map(...).unwrap_or(...)`
- `short_label`: chain `.as_deref().map().map().unwrap_or_else()`

**Run:** `cargo run --bin day6_options`

---

### Exercise 5 -- The merge node preview

Create `~/Antigravity/Achilles/src/bin/day6_merge.rs`.

**Task:** Simulate what the Achilles merge node does: receive
scan results from multiple tools, deduplicate hosts by IP, merge
their data, and produce a summary report.

1. Define:

```rust
#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
    ports: Vec<u16>,
    source: String,
}
```

2. Write:

```rust
// Merge hosts with the same IP into one host.
// Combined host should have:
// - all unique ports from all hosts with that IP (sorted, deduplicated)
// - hostname from the first host that HAS one (first Some wins)
// - source: if only one source, keep it as-is. If multiple distinct
//   sources, set to "merged (tool1, tool2, ...)" in order of appearance.
fn merge_hosts(hosts: Vec<Host>) -> Vec<Host>
```

3. Write:

```rust
// Generate a summary report string
fn generate_report(hosts: &[Host]) -> String
```

4. In `main()`, simulate output from nmap and subfinder:

```rust
let nmap_hosts = vec![
    Host { ip: "10.0.0.1".to_string(), hostname: None, ports: vec![22, 80, 443], source: "nmap".to_string() },
    Host { ip: "10.0.0.2".to_string(), hostname: None, ports: vec![80, 8080], source: "nmap".to_string() },
];

let subfinder_hosts = vec![
    Host { ip: "10.0.0.1".to_string(), hostname: Some("web.target.htb".to_string()), ports: vec![], source: "subfinder".to_string() },
    Host { ip: "10.0.0.3".to_string(), hostname: Some("db.target.htb".to_string()), ports: vec![], source: "subfinder".to_string() },
];
```

5. Combine both Vecs, merge, and generate the report.

**Expected output:**

```
=== Merge Node Output ===

3 unique hosts after merge:

[10.0.0.1] web.target.htb
  Ports: 22, 80, 443
  Source: merged (nmap, subfinder)

[10.0.0.2]
  Ports: 80, 8080
  Source: nmap

[10.0.0.3] db.target.htb
  Ports: (none)
  Source: subfinder
```

**Hints:**
- Use `.chain()` to combine the two Vecs
- Group by IP using HashMap (Exercise 2 pattern)
- For each group, fold/reduce the hosts into a merged host
- Use `.sort()` and `.dedup()` on the merged ports Vec
- Use `.into_values()` to extract the merged hosts from the HashMap

**Run:** `cargo run --bin day6_merge`

---

## Part 12: Rustlings

Complete these Rustlings sections:

```
lifetimes1, lifetimes2, lifetimes3
iterators1, iterators2, iterators3, iterators4, iterators5
```

Lifetimes reinforce Part 10. Iterators reinforce Parts 3-4.
They are short -- 45-60 minutes total.

Run: `rustlings`

---

## Part 13: Key Concepts to Internalize

**1. What is `Vec<T>`?**
A heap-allocated, growable array. Elements are contiguous in memory.
push/pop are O(1) amortized. Random access via index is O(1).

**2. What are the three iterator types?**
`.iter()` borrows (&T), `.iter_mut()` mutably borrows (&mut T),
`.into_iter()` consumes (T). Choose based on whether you need
the collection after iteration.

**3. What does "lazy" mean for iterators?**
`.map()` and `.filter()` build a pipeline description. No work
happens until a consuming method (.collect(), .sum(), .for_each())
drives the pipeline.

**4. What is zero-cost abstraction?**
Iterator chains compile to the same machine code as hand-written
loops. The compiler inlines the closures and eliminates the
abstraction overhead. You pay nothing for the expressiveness.

**5. What is a closure?**
An anonymous function that captures variables from its enclosing
scope. `|x| x * 2` is a closure. Rust infers the capture mode
(borrow, mutable borrow, or move).

**6. What is the `move` keyword on a closure?**
Forces the closure to take ownership of all captured variables.
Required when the closure outlives the scope where variables
were defined (thread spawning, async tasks).

**7. What is `HashMap<K, V>`?**
A hash table mapping keys to values. O(1) average lookup, insert,
and remove. NOT ordered. Import from `std::collections::HashMap`.

**8. What is the Entry API?**
`map.entry(key).or_insert(default)` -- single lookup to get or
insert. Avoids the contains_key + insert double-lookup pattern.

**9. What is `Option<T>`?**
Rust's replacement for null. `Some(T)` = value exists. `None` =
value does not exist. The compiler forces you to handle both cases.

**10. What are Option combinators?**
Methods like `.map()`, `.and_then()`, `.unwrap_or()`, `.as_deref()`
that transform or extract Option values without match blocks.
They make code cleaner and more chainable.

**11. What is `.flat_map()`?**
`.map()` where each element produces multiple items (an iterator).
`.flat_map()` flattens all those iterators into a single stream.
Used when one host produces multiple ports, or one scan produces
multiple findings.

**12. What is `.collect()`?**
A consuming method that builds a collection from an iterator.
Can produce Vec, HashMap, HashSet, String, and more. Requires a
type annotation or turbofish to know what to collect into.

**13. What is a lifetime (`'a`)?**
The compiler's annotation for how long a reference is valid.
Usually inferred. When the compiler cannot infer (multiple inputs,
one output), you annotate manually.

**14. What is `as_deref()`?**
Converts `Option<String>` to `Option<&str>` by applying the Deref
trait. Lets you work with `&str` instead of `String` inside Option,
avoiding unnecessary clones and allocations.

---

## Completion Checklist

- [X] Read and understand Parts 1-10
      (Vec, three iterators, adaptors, consuming methods, closures,
      HashMap, Entry API, grouping, Option combinators, lifetimes)
- [X] Exercise 1 -- Filter and sort hostnames (day6_filter.rs)
- [X] Exercise 2 -- Group hosts by IP (day6_group.rs)
- [X] Exercise 3 -- Iterator chains on ADCObjects (day6_iterators.rs)
- [X] Exercise 4 -- Option combinators (day6_options.rs)
- [X] Exercise 5 -- Merge node preview (day6_merge.rs)
- [X] Complete Rustlings: lifetimes1-3, iterators1-5
- [X] Run `cargo clippy` and fix all warnings
- [X] Run `cargo fmt` on all files
- [X] Can chain .iter().filter().map().collect() fluently
- [X] Can use Entry API for grouping operations
- [X] Can use Option combinators instead of match blocks
- [X] Can explain the difference between .iter(), .iter_mut(), .into_iter()

---

## Lesson

> The engine does not loop through hosts manually.
> It does not write `for i in 0..hosts.len()`.
> It writes:
>
>   hosts.iter()
>     .filter(|h| h.ports.iter().any(|p| p.state == "open"))
>     .flat_map(|h| h.urls())
>     .collect()
>
> Three lines. Filter. Transform. Collect.
> No indices. No off-by-one errors. No bounds checking.
> The compiler verifies the types at every step.
> The optimizer removes every abstraction layer.
>
> Iterator chains are not syntax sugar.
> They are the language Achilles thinks in.

---

**Next: Day 7 -- ADC Type Definitions, Serde, and JSON Serialization ->**
