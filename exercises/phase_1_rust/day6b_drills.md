# Day 6B: Drills -- Build the Muscle Memory

These are short, focused drills. Each one isolates ONE concept.
Do them in order. Each should take 5-10 minutes max.

Create `~/Antigravity/Achilles/src/bin/day6b_drills.rs` for all drills.
Put all functions in one file. Call each from `main()`.

---

## Drill 1: .map() -- Transform every element

Write a function that takes a `&[u16]` of port numbers and returns
`Vec<String>` where each port is formatted as `"port:{number}"`.

```rust
fn label_ports(ports: &[u16]) -> Vec<String>
```

Input: `[80, 443, 8080]`
Output: `["port:80", "port:443", "port:8080"]`

Hint: `.iter().map(|p| ...).collect()`

---

## Drill 2: .filter() -- Keep only matching elements

Write a function that takes `&[u16]` and returns only the
privileged ports (below 1024).

```rust
fn privileged_ports(ports: &[u16]) -> Vec<u16>
```

Input: `[22, 80, 3306, 443, 8080, 9090]`
Output: `[22, 80, 443]`

Hint: `.iter().filter(...)` -- remember, filter yields references.
You need `.copied()` before `.collect()` to convert `&u16` to `u16`.

---

## Drill 3: .filter().map() -- Two-step pipeline

Write a function that takes `&[i32]` of raw scores and returns
`Vec<String>` of labels ONLY for passing scores (>= 70).

```rust
fn passing_labels(scores: &[i32]) -> Vec<String>
```

Input: `[85, 42, 91, 67, 73, 55]`
Output: `["PASS:85", "PASS:91", "PASS:73"]`

Hint: `.iter().filter(...).map(...).collect()`

---

## Drill 4: .filter_map() -- Filter AND transform in one shot

Write a function that takes `&[&str]` of strings that MIGHT be
valid port numbers. Parse each one. Keep only the valid ports.
Discard anything that fails to parse.

```rust
fn parse_ports(raw: &[&str]) -> Vec<u16>
```

Input: `["80", "abc", "443", "", "8080", "99999"]`
Output: `[80, 443, 8080]`

Hint: `str::parse::<u16>()` returns `Result<u16, _>`.
`.ok()` converts `Result` to `Option` (Ok -> Some, Err -> None).
So `.filter_map(|s| s.parse::<u16>().ok())` does it in one shot.

---

## Drill 5: .flat_map() -- One-to-many flattening

You have a list of subnets. Each subnet expands to multiple IPs.

```rust
fn expand_subnets(subnets: &[&str]) -> Vec<String>
```

For simplicity, each "subnet" is just a base IP prefix like `"10.0.0"`.
Expand each to 3 IPs: `.1`, `.2`, `.3`.

Input: `["10.0.0", "192.168.1"]`
Output: `["10.0.0.1", "10.0.0.2", "10.0.0.3", "192.168.1.1", "192.168.1.2", "192.168.1.3"]`

Hint: `.iter().flat_map(|subnet| (1..=3).map(move |i| format!("{}.{}", subnet, i))).collect()`

Note the `move` on the inner closure. `subnet` is borrowed from the
outer iterator. The inner `.map()` returns a lazy iterator that uses
`subnet`. `move` copies the `&&str` reference into the inner closure
so it survives after the outer iteration moves on. This is the exact
pattern from Part 9, Example 4 in the Day 6 doc.

---

## Drill 6: .fold() -- Build a result from scratch

Write a function that takes `&[&str]` of hostnames and returns a
single comma-separated `String`.

```rust
fn join_hostnames(hosts: &[&str]) -> String
```

Input: `["web.htb", "db.htb", "mail.htb"]`
Output: `"web.htb, db.htb, mail.htb"`

Hint: Use `.iter().fold(String::new(), ...)`. Handle the first
element differently (no leading comma). You can check if `acc.is_empty()`.

Note: Rust has `.join()` on slices that does this directly:
`hosts.join(", ")`. This drill is just to practice fold.

---

## Drill 7: .fold() with HashMap -- Counting

Write a function that takes `&[&str]` of log levels and returns
a `HashMap<String, usize>` counting occurrences.

```rust
fn count_levels(levels: &[&str]) -> HashMap<String, usize>
```

Input: `["INFO", "ERROR", "INFO", "WARN", "ERROR", "INFO"]`
Output: `{"INFO": 3, "ERROR": 2, "WARN": 1}`

Hint: Same pattern as `count_by_severity` from Exercise 3.
`fold(HashMap::new(), |mut acc, level| { ... acc })`

---

## Drill 8: Option .map() -- Transform inside the box

Write a function that takes an `Option<String>` username and returns
a greeting. If Some, greet them. If None, greet "stranger".

```rust
fn greet(username: Option<&str>) -> String
```

Input: `Some("phantom")` -> `"Hello, PHANTOM!"`
Input: `None` -> `"Hello, stranger!"`

Hint: `.map(|name| ...).unwrap_or_else(|| ...)`

Note: This one takes `Option<&str>` directly so you do NOT
need `as_deref()`. You would need `as_deref()` if the struct
field were `Option<String>` accessed through `&self`.

---

## Drill 9: Option .map() chain -- Multiple transforms

Write a function that processes an optional command string:
uppercase it, then prefix with `"CMD:"`, with fallback `"CMD:NOP"`.

```rust
fn format_command(cmd: Option<&str>) -> String
```

Input: `Some("scan")` -> `"CMD:SCAN"`
Input: `None` -> `"CMD:NOP"`

Hint: `.map(...).map(...).unwrap_or_else(...)` -- chain two maps.
First map uppercases. Second map prefixes.

---

## Drill 10: Option .and_then() -- Chain fallible lookups

You have two lookup functions. Chain them.

```rust
fn find_ip(hostname: &str) -> Option<String> {
    match hostname {
        "web.htb" => Some("10.0.0.1".to_string()),
        "db.htb" => Some("10.0.0.2".to_string()),
        _ => None,
    }
}

fn find_service(ip: &str) -> Option<String> {
    match ip {
        "10.0.0.1" => Some("http".to_string()),
        _ => None,
    }
}

fn lookup_service(hostname: &str) -> Option<String>
```

`lookup_service("web.htb")` -> `Some("http")`
`lookup_service("db.htb")` -> `None` (IP found but no service)
`lookup_service("unknown.htb")` -> `None` (no IP found)

Hint: `find_ip(hostname).and_then(|ip| find_service(&ip))`

`.and_then()` passes the unwrapped value to a function that ITSELF
returns Option. If `find_ip` returns None, `.and_then` short-circuits
and returns None without calling `find_service`.

---

## Drill 11: .chain() + .filter() -- Combine and process

You have two Vecs of IP addresses from different tools.
Combine them and return only the unique ones.

```rust
fn unique_targets(nmap_ips: &[&str], subfinder_ips: &[&str]) -> Vec<String>
```

Input: `["10.0.0.1", "10.0.0.2"]`, `["10.0.0.2", "10.0.0.3"]`
Output: `["10.0.0.1", "10.0.0.2", "10.0.0.3"]` (sorted, no duplicates)

Hint: `.chain()` to combine, collect into a `HashSet` to deduplicate,
then collect into a `Vec` and sort.

---

## Drill 12: .enumerate() + .filter() -- Indexed filtering

Write a function that returns every other element (even indices
only: 0, 2, 4, ...) from a slice.

```rust
fn even_indexed(items: &[&str]) -> Vec<&str>
```

Input: `["a", "b", "c", "d", "e"]`
Output: `["a", "c", "e"]`

Hint: `.iter().enumerate().filter(|(i, _)| ...).map(|(_, item)| ...).collect()`

---

## Drill 13: Full pipeline -- Putting it together

Write a single function that:
1. Takes `&[&str]` of raw scan lines like `"10.0.0.1:80:OPEN"`, `"10.0.0.2:22:CLOSED"`
2. Parses each line into (ip, port, state)
3. Keeps only OPEN ports
4. Returns `Vec<String>` formatted as `"ip:port"`

```rust
fn open_services(lines: &[&str]) -> Vec<String>
```

Input:
```rust
&["10.0.0.1:80:OPEN", "10.0.0.2:22:CLOSED", "10.0.0.1:443:OPEN", "10.0.0.3:8080:OPEN"]
```

Output: `["10.0.0.1:80", "10.0.0.1:443", "10.0.0.3:8080"]`

Hint: Split each line by `:`. Use `.filter_map()` to parse AND
filter in one step -- return `Some(formatted)` for OPEN,
`None` for everything else.

Watch out: `"10.0.0.1:80:OPEN".split(':')` splits on EVERY colon,
including the dots in the IP... wait, no. `:` only splits on colons,
dots are left alone. You will get `["10.0.0.1", "80", "OPEN"]`.
Use `.collect::<Vec<&str>>()` on the split, then index into it.

---

## Drill 14: HashMap entry -- Grouping with transformation

Write a function that takes `&[(&str, u16)]` of (hostname, port) pairs
and groups them into a HashMap where each hostname maps to a
SORTED, DEDUPLICATED Vec of its ports.

```rust
fn group_ports(pairs: &[(&str, u16)]) -> HashMap<String, Vec<u16>>
```

Input:
```rust
&[("web.htb", 80), ("db.htb", 3306), ("web.htb", 443), ("web.htb", 80), ("db.htb", 5432)]
```

Output:
```
{"web.htb": [80, 443], "db.htb": [3306, 5432]}
```

Hint: For loop with Entry API to group. Then iterate over
`.values_mut()` to sort and dedup each Vec in place.

---

## Drill 15: Option combinators on struct fields

```rust
struct Target {
    ip: String,
    hostname: Option<String>,
    os: Option<String>,
}
```

Write ONE function that produces a summary line:

```rust
fn summary(target: &Target) -> String
```

Rules:
- Start with the IP
- If hostname exists, append ` (hostname)` 
- If OS exists, append ` [os]`
- If neither, just the IP

Examples:
- `ip="1.1.1.1", hostname=Some("web"), os=Some("Linux")`
  -> `"1.1.1.1 (web) [Linux]"`
- `ip="2.2.2.2", hostname=None, os=Some("Windows")`
  -> `"2.2.2.2 [Windows]"`
- `ip="3.3.3.3", hostname=Some("db"), os=None`
  -> `"3.3.3.3 (db)"`
- `ip="4.4.4.4", hostname=None, os=None`
  -> `"4.4.4.4"`

Hint: Start with `let mut result = target.ip.clone()`.
Then use `.as_deref().map(...)` on each Optional field and
check if there is something to append. Or use `if let` --
this drill is about combining approaches pragmatically.
The "no match, no if let" constraint was only for Exercise 4.
Here, use whatever is cleanest.

---

---

## Drill 16: .iter() vs .into_iter() vs .iter_mut() -- Know the difference

Create a `Vec<String>` of 3 hostnames. Write three separate blocks
in main that demonstrate each iterator:

```rust
fn drill_16() {
    let hosts = vec!["web.htb".to_string(), "db.htb".to_string(), "mail.htb".to_string()];

    // Block A: .iter() -- borrows, original Vec survives
    for h in hosts.iter() {
        // h is &String
        println!("borrowed: {}", h);
    }
    println!("hosts still alive: {:?}", hosts); // this must compile

    // Block B: .iter_mut() -- mutable borrow, modify in place
    let mut hosts2 = hosts.clone();
    for h in hosts2.iter_mut() {
        // h is &mut String
        *h = h.to_uppercase(); // modify in place
    }
    println!("mutated: {:?}", hosts2); // ["WEB.HTB", "DB.HTB", "MAIL.HTB"]

    // Block C: .into_iter() -- consumes, original Vec is GONE
    let hosts3 = hosts.clone();
    let collected: Vec<String> = hosts3.into_iter()
        .map(|h| format!("[{}]", h))
        .collect();
    // println!("{:?}", hosts3);  // UNCOMMENT THIS AND IT WON'T COMPILE -- hosts3 consumed
    println!("consumed and transformed: {:?}", collected);
}
```

YOUR TASK: Type this out (don't copy-paste). Run it. Then uncomment the
`hosts3` println and confirm the compiler error. Read the error message.
Then comment it back and move on.

The goal is to feel the difference in your fingers:
- `.iter()` = I'm just looking, don't take my stuff
- `.iter_mut()` = I'm changing things in place
- `.into_iter()` = take everything, the original is gone

---

## Drill 17: .any() and .all() -- Boolean questions

Write two functions:

```rust
// Does any host have a privileged port (< 1024)?
fn has_privileged(ports: &[u16]) -> bool

// Are ALL ports above 1024 (no privileged ports)?
fn all_unprivileged(ports: &[u16]) -> bool
```

Input: `[22, 80, 3306, 8080]`
- `has_privileged` -> `true` (22 and 80 are < 1024)
- `all_unprivileged` -> `false`

Input: `[3306, 8080, 9090]`
- `has_privileged` -> `false`
- `all_unprivileged` -> `true`

Hint: `.iter().any(|p| ...)` and `.iter().all(|p| ...)`
Both short-circuit -- `.any()` stops at first true, `.all()` stops at first false.

---

## Drill 18: .find() -- First match

Write a function that finds the first critical finding from a list.

```rust
fn first_critical(findings: &[(&str, &str)]) -> Option<&str>
// Each tuple is (title, severity)
```

Input: `[("XSS", "High"), ("SQLi", "Critical"), ("IDOR", "Critical")]`
Output: `Some("SQLi")`

Input: `[("Missing Headers", "Low"), ("Verbose Errors", "Medium")]`
Output: `None`

Hint: `.iter().find(|(_, severity)| ...).map(|(title, _)| ...)`

`.find()` returns `Option<&Item>`. Chain `.map()` on the Option
to extract just the title from the tuple.

---

## Drill 19: .count() -- How many match

Write a function that counts how many ports are open.

```rust
fn count_open(statuses: &[(&str, bool)]) -> usize
// Each tuple is (service_name, is_open)
```

Input: `[("http", true), ("ssh", false), ("https", true), ("ftp", false)]`
Output: `2`

Hint: `.iter().filter(|(_, open)| ...).count()`

`.count()` consumes the iterator and returns the total number of items.

---

## Drill 20: .take() and .skip() -- Slicing iterators

Write two functions:

```rust
// Return the first N hosts from a list
fn first_n(hosts: &[&str], n: usize) -> Vec<&str>

// Return all hosts AFTER skipping the first N
fn after_n(hosts: &[&str], n: usize) -> Vec<&str>
```

Input: `["a.htb", "b.htb", "c.htb", "d.htb", "e.htb"]`
- `first_n(hosts, 3)` -> `["a.htb", "b.htb", "c.htb"]`
- `after_n(hosts, 3)` -> `["d.htb", "e.htb"]`

Hint: `.iter().take(n).copied().collect()` and
`.iter().skip(n).copied().collect()`

`.copied()` converts `&&str` to `&str` (copies the reference, not the data).

---

## Drill 21: Closure as Fn -- Read-only capture

Write a function that takes a severity threshold string and returns
a CLOSURE that checks if a finding matches that severity.

```rust
fn severity_checker(threshold: &str) -> impl Fn(&str) -> bool + '_
```

Usage:
```rust
let is_critical = severity_checker("Critical");
assert!(is_critical("Critical"));
assert!(!is_critical("Low"));
```

Hint: `move |severity| severity == threshold`

The returned closure captures `threshold` by reference (borrows it).
It only READS the captured value. That makes it `Fn` -- callable
multiple times, because reading doesn't change or consume anything.

The `+ '_` in the return type means the returned closure borrows
from the input `threshold` with its lifetime. Without it, the
compiler would complain that the closure references `threshold`
which has a limited lifetime.

---

## Drill 22: Closure as FnMut -- Mutable capture

Write a function that returns a closure which acts as a counter.
Each call increments and returns the new count.

```rust
fn make_counter() -> impl FnMut() -> usize
```

Usage:
```rust
let mut counter = make_counter();
assert_eq!(counter(), 1);
assert_eq!(counter(), 2);
assert_eq!(counter(), 3);
```

Hint: The closure captures a `usize` by mutable reference
(actually by value with `move`, since the variable is created
inside `make_counter`). Each call mutates it.

```rust
fn make_counter() -> impl FnMut() -> usize {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}
```

Here `move` is mandatory because `count` is a local variable
inside `make_counter`. Without `move`, the closure would borrow
`count`, but `count` would be dropped when `make_counter` returns.
`move` transfers ownership of `count` into the closure so it
survives. The closure mutates the moved `count` on every call --
that makes it `FnMut`.

---

## Drill 23: Closure trait identification -- Which trait is this?

No code to write. Just answer in comments in your file.

For each closure, identify the trait (Fn, FnMut, or FnOnce) and WHY:

```rust
// A)
let name = String::from("phantom");
let print_it = || println!("{}", name);
// Trait: ???  Why: ???

// B)
let mut total = 0;
let mut add = |x: i32| total += x;
// Trait: ???  Why: ???

// C)
let data = vec![1, 2, 3];
let consume = || drop(data);
// Trait: ???  Why: ???

// D)
let name = String::from("phantom");
let consume = move || println!("{}", name);
// Trait: ???  Why: ???
```

Answers (write these as comments in your drill file AFTER you guess):
- A: `Fn` -- only reads `name` through a shared reference.
- B: `FnMut` -- mutates `total` through a mutable reference.
- C: `FnOnce` -- `drop(data)` consumes `data`. After one call, data is gone. Can't call again.
- D: `Fn` -- `move` transfers ownership, but the closure only READS `name` (println reads). `move` does NOT make it FnOnce. What makes it FnOnce is consuming the owned value (like `drop`). This one never consumes, so it stays `Fn`.

D is the trick question. Most people think `move` = FnOnce. It doesn't.

---

## Drill 24: .sum() and .min() and .max() -- Numeric consumers

Write three functions:

```rust
// Total all port numbers
fn total_ports(ports: &[u16]) -> u16

// Find the lowest port
fn lowest_port(ports: &[u16]) -> Option<u16>

// Find the highest port
fn highest_port(ports: &[u16]) -> Option<u16>
```

Input: `[80, 443, 22, 8080]`
- `total_ports` -> `8625`
- `lowest_port` -> `Some(22)`
- `highest_port` -> `Some(8080)`

Input: `[]` (empty slice)
- `total_ports` -> `0`
- `lowest_port` -> `None`
- `highest_port` -> `None`

Hint:
- `.iter().copied().sum()` -- `.sum()` needs owned values, `.copied()` converts `&u16` to `u16`
- `.iter().copied().min()` -- returns `Option<u16>` because empty iterators have no min
- `.iter().copied().max()` -- same

`.sum()` requires a type annotation on the binding or turbofish:
`let total: u16 = ports.iter().copied().sum();`

---

## Drill 25: .zip() -- Parallel iteration

You have a list of hostnames and a list of IPs. Pair them together.

```rust
fn pair_hosts(names: &[&str], ips: &[&str]) -> Vec<String>
```

Input: `["web", "db", "mail"]`, `["10.0.0.1", "10.0.0.2", "10.0.0.3"]`
Output: `["web -> 10.0.0.1", "db -> 10.0.0.2", "mail -> 10.0.0.3"]`

Hint: `names.iter().zip(ips.iter()).map(|(name, ip)| ...).collect()`

`.zip()` stops when the SHORTER iterator runs out. If names has 3
items and ips has 2, you get 2 pairs. No panic, no padding.

---

## Drill 26: .inspect() -- Debug without breaking the chain

Write a function that filters ports, but uses `.inspect()` to
print each element as it flows through the pipeline.

```rust
fn debug_filter(ports: &[u16]) -> Vec<u16>
```

```rust
fn debug_filter(ports: &[u16]) -> Vec<u16> {
    ports.iter()
        .copied()
        .inspect(|p| println!("  checking port: {}", p))
        .filter(|p| *p >= 1024)
        .inspect(|p| println!("  kept port: {}", p))
        .collect()
}
```

Input: `[22, 80, 3306, 8080]`
Output prints:
```
  checking port: 22
  checking port: 80
  checking port: 3306
  kept port: 3306
  checking port: 8080
  kept port: 8080
```
Returns: `[3306, 8080]`

`.inspect()` does NOT change the elements. It runs a side-effect
(like printing) and passes the element through untouched. Use it
to debug iterator chains without adding temporary `.collect()` calls.

YOUR TASK: Type it, run it, see the output flow. Then remove
the `.inspect()` calls and confirm the result is the same.

---

## Drill 27: .position() -- Find the index

Write a function that finds the index of the first occurrence
of a specific port.

```rust
fn find_port_index(ports: &[u16], target: u16) -> Option<usize>
```

Input: `[22, 80, 443, 8080]`, target: `443`
Output: `Some(2)`

Input: `[22, 80, 443, 8080]`, target: `9090`
Output: `None`

Hint: `.iter().position(|p| *p == target)`

`.position()` returns `Option<usize>` -- the index of the first
matching element, or None if nothing matches. Like `.find()` but
returns the INDEX instead of the VALUE.

---

## Drill 28: .for_each() -- Side effects without collect

Write a function that prints each host with a number prefix.
Return nothing (the purpose is the side effect).

```rust
fn print_numbered(hosts: &[&str])
```

Input: `["web.htb", "db.htb", "mail.htb"]`
Output prints:
```
[1] web.htb
[2] db.htb
[3] mail.htb
```

Hint: `.iter().enumerate().for_each(|(i, host)| println!(...))`

`.for_each()` is a consuming method like `.collect()`, but instead
of building a collection, it runs a closure on every element and
returns `()`. Use it when you want the side effects (printing,
logging) but don't need a result.

---

## Drill 29: .copied() vs .cloned() -- Know the difference

Write two functions that demonstrate each:

```rust
// .copied() -- for Copy types (i32, u16, bool, &str, etc.)
fn copy_ports(ports: &[u16]) -> Vec<u16> {
    ports.iter().copied().collect()
    // .copied() converts &u16 -> u16 by copying the bits
    // Only works on types that implement Copy (cheap, stack-only)
}

// .cloned() -- for Clone types (String, Vec, etc.)
fn clone_names(names: &[String]) -> Vec<String> {
    names.iter().cloned().collect()
    // .cloned() converts &String -> String by calling .clone()
    // Works on types that implement Clone (may heap-allocate)
}
```

YOUR TASK: Type both. Then try switching them -- use `.cloned()`
where `.copied()` is, and vice versa.

- `.cloned()` works on Copy types too (Clone is a supertrait of Copy).
  So `.cloned()` on `&u16` works fine. It's just less precise.
- `.copied()` does NOT work on non-Copy types. `.copied()` on
  `&String` fails -- String isn't Copy.

Rule: use `.copied()` for primitives and references.
Use `.cloned()` for String/Vec/anything that heap-allocates.

---

## Drill 30: HashMap .get() and .contains_key()

Write a function that takes a HashMap of IP -> hostname mappings
and a list of IPs to look up. Return the results.

```rust
fn lookup_hosts(
    dns: &HashMap<String, String>,
    queries: &[&str]
) -> Vec<String>
```

For each query IP:
- If found in the map, return `"ip -> hostname"`
- If not found, return `"ip -> NOT FOUND"`

Input map: `{"10.0.0.1": "web.htb", "10.0.0.2": "db.htb"}`
Input queries: `["10.0.0.1", "10.0.0.3", "10.0.0.2"]`
Output: `["10.0.0.1 -> web.htb", "10.0.0.3 -> NOT FOUND", "10.0.0.2 -> db.htb"]`

Hint: `queries.iter().map(|ip| { ... }).collect()`

Inside the map closure, use `.get()`:
- `dns.get(*ip)` returns `Option<&String>`
- Chain `.map()` on the Option or use `match`/`if let`

Note: `.get()` takes a reference to the key type. Since the HashMap
key is `String`, you can pass `&str` because `HashMap<String, V>`
implements `get<Q>` where `String: Borrow<Q>` and `&str` qualifies.

---

## Drill 31: HashMap .and_modify() -- Update existing entries

Write a function that counts word frequencies, but if a word
is already seen, also track when it was last seen (by index).

```rust
fn word_stats(words: &[&str]) -> HashMap<String, (usize, usize)>
// Value is (count, last_seen_index)
```

Input: `["scan", "report", "scan", "alert", "scan"]`
Output: `{"scan": (3, 4), "report": (1, 1), "alert": (1, 3)}`

Hint: Use `.enumerate()` to get the index, then Entry API:

```rust
.entry(word.to_string())
.and_modify(|(count, last)| { *count += 1; *last = index; })
.or_insert((1, index));
```

`.and_modify()` runs a closure on the value IF the key exists.
`.or_insert()` inserts a default IF the key is new.
Chained together: modify existing OR insert new.

---

## Drill 32: Option .is_some() / .is_none() / .or()

Write three functions:

```rust
// Does this target have a hostname?
fn has_hostname(hostname: &Option<String>) -> bool

// Use primary hostname, or fall back to secondary
fn best_hostname(primary: Option<&str>, fallback: Option<&str>) -> Option<String>

// Only keep hostnames that end with ".htb"
fn htb_only(hostname: Option<&str>) -> Option<&str>
```

Examples:
- `has_hostname(&Some("web.htb".to_string()))` -> `true`
- `has_hostname(&None)` -> `false`
- `best_hostname(Some("web.htb"), Some("backup.htb"))` -> `Some("web.htb")`
- `best_hostname(None, Some("backup.htb"))` -> `Some("backup.htb")`
- `best_hostname(None, None)` -> `None`
- `htb_only(Some("web.htb"))` -> `Some("web.htb")`
- `htb_only(Some("google.com"))` -> `None`
- `htb_only(None)` -> `None`

Hints:
- `has_hostname`: just `.is_some()`. Returns bool.
- `best_hostname`: `.or()` returns the first Some it finds.
  `primary.map(|s| s.to_string()).or(fallback.map(|s| s.to_string()))`
  Or: `primary.or(fallback).map(|s| s.to_string())`
- `htb_only`: `.filter(|h| h.ends_with(".htb"))`. Option has a
  `.filter()` method that returns None if the predicate fails.

---

## Drill 33: Integration -- The recon summary

Final boss. Combine EVERYTHING into one function.

```rust
struct ReconData {
    hosts: Vec<String>,
    ports: Vec<(String, u16, bool)>,  // (ip, port_number, is_open)
    findings: Vec<(String, String)>,  // (title, severity)
}

fn recon_summary(data: &ReconData) -> String
```

The function must produce a summary string using ONLY iterator methods:

```
=== Recon Summary ===
Hosts: 3 total, 2 unique
Open ports: 22, 80, 443 (3 total)
Highest open port: 443
Findings: 4 total
  Critical: 1
  High: 2
  Low: 1
Has critical findings: YES
First critical: SQL Injection
```

Requirements (use each method at least once):
- `.iter()`, `.filter()`, `.map()`, `.collect()` for open ports
- `.count()` for totals
- `.max()` for highest port
- `.fold()` or Entry API for severity counting
- `.any()` for "has critical"
- `.find()` for "first critical"
- `.chain()` somewhere (combine data from two sources)
- HashSet for unique host count
- `.join(", ")` for comma-separated port list

Test data:
```rust
let data = ReconData {
    hosts: vec!["10.0.0.1", "10.0.0.2", "10.0.0.1"]
        .into_iter().map(String::from).collect(),
    ports: vec![
        ("10.0.0.1".into(), 22, true),
        ("10.0.0.1".into(), 80, true),
        ("10.0.0.1".into(), 443, true),
        ("10.0.0.2".into(), 3306, false),
    ],
    findings: vec![
        ("XSS Reflected".into(), "High".into()),
        ("SQL Injection".into(), "Critical".into()),
        ("Missing Headers".into(), "Low".into()),
        ("Open Redirect".into(), "High".into()),
    ],
};
```

This drill uses 10+ methods in one function. If you can write this
without looking at hints, Day 6 is conquered.

---

## Checklist

- [ ] Drill 1: .map() port labels
- [ ] Drill 2: .filter() privileged ports
- [ ] Drill 3: .filter().map() passing labels
- [ ] Drill 4: .filter_map() parse ports
- [ ] Drill 5: .flat_map() subnet expansion
- [ ] Drill 6: .fold() join hostnames
- [ ] Drill 7: .fold() count levels
- [ ] Drill 8: Option .map() greeting
- [ ] Drill 9: Option .map() chain command
- [ ] Drill 10: Option .and_then() lookup chain
- [ ] Drill 11: .chain() unique targets
- [ ] Drill 12: .enumerate() even indexed
- [ ] Drill 13: Full pipeline open services
- [ ] Drill 14: HashMap entry group ports
- [ ] Drill 15: Option combinators summary
- [ ] Drill 16: .iter() vs .into_iter() vs .iter_mut()
- [ ] Drill 17: .any() and .all()
- [ ] Drill 18: .find() first match
- [ ] Drill 19: .count()
- [ ] Drill 20: .take() and .skip()
- [ ] Drill 21: Closure as Fn
- [ ] Drill 22: Closure as FnMut
- [ ] Drill 23: Closure trait identification
- [ ] Drill 24: .sum() .min() .max()
- [ ] Drill 25: .zip() parallel iteration
- [ ] Drill 26: .inspect() debug chains
- [ ] Drill 27: .position() find index
- [ ] Drill 28: .for_each() side effects
- [ ] Drill 29: .copied() vs .cloned()
- [ ] Drill 30: HashMap .get() .contains_key()
- [ ] Drill 31: HashMap .and_modify()
- [ ] Drill 32: Option .is_some() .is_none() .or() .filter()
- [ ] Drill 33: Integration -- recon summary (FINAL BOSS)

---

> "Repetition is the mother of skill. You don't understand iterators
> by reading about them. You understand them by writing fifty chains
> until your fingers type .iter().filter_map().collect() without
> your brain needing to think about it."

