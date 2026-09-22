# Day 6 Quick Reference -- Every Method, One Example Each

This is NOT a teaching doc. This is a lookup card.
Keep it open while doing the drills. Glance, don't read.

---

## The Three Iterators

```rust
let v = vec!["a".to_string(), "b".to_string()];

// .iter() borrows → yields &String
for s in v.iter() { }        // s: &String, v still alive after

// .iter_mut() mutably borrows → yields &mut String
for s in v.iter_mut() { }    // s: &mut String, can modify in place

// .into_iter() consumes → yields String
for s in v.into_iter() { }   // s: String (owned), v is GONE
```

Shorthand: `for s in &v` = `.iter()`, `for s in &mut v` = `.iter_mut()`, `for s in v` = `.into_iter()`

---

## Iterator Adaptors (lazy -- nothing happens until consumed)

### .map() -- Transform each element
```rust
[1, 2, 3].iter().map(|x| x * 10)
// yields: 10, 20, 30
```

### .filter() -- Keep elements matching a condition
```rust
[1, 2, 3, 4].iter().filter(|x| **x > 2)
// yields: &3, &4
// Note: filter gives you &&T (reference to reference). Use **x or |&&x|.
```

### .filter_map() -- Filter AND transform in one shot
```rust
["1", "abc", "3"].iter().filter_map(|s| s.parse::<i32>().ok())
// yields: 1, 3
// Returns Some(val) to keep, None to discard.
```

### .flat_map() -- One-to-many, then flatten
```rust
["hi", "yo"].iter().flat_map(|s| s.chars())
// yields: 'h', 'i', 'y', 'o'
// Each element produces multiple items. All items are flattened into one stream.
```

### .enumerate() -- Attach index (starting at 0)
```rust
["a", "b", "c"].iter().enumerate()
// yields: (0, &"a"), (1, &"b"), (2, &"c")
```

### .chain() -- Concatenate two iterators
```rust
[1, 2].iter().chain([3, 4].iter())
// yields: &1, &2, &3, &4
```

### .take(n) -- First n elements
```rust
[10, 20, 30, 40].iter().take(2)
// yields: &10, &20
```

### .skip(n) -- Discard first n elements
```rust
[10, 20, 30, 40].iter().skip(2)
// yields: &30, &40
```

### .zip() -- Pair elements from two iterators
```rust
["a", "b"].iter().zip([1, 2].iter())
// yields: (&"a", &1), (&"b", &2)
// Stops at shorter iterator.
```

### .inspect() -- Peek without changing (debug tool)
```rust
[1, 2, 3].iter().inspect(|x| println!("saw: {}", x)).collect::<Vec<_>>()
// prints: saw: 1, saw: 2, saw: 3
// elements pass through unchanged
```

### .copied() -- Convert &T to T (Copy types only)
```rust
[1, 2, 3].iter().copied()
// yields: 1, 2, 3 (owned i32, not &i32)
// Only works if T: Copy (integers, bools, &str, etc.)
```

### .cloned() -- Convert &T to T (Clone types)
```rust
vec!["a".to_string()].iter().cloned()
// yields: String (owned, heap-allocated clone)
// Works on any T: Clone. Use for String, Vec, etc.
```

---

## Consuming Methods (drive the iterator, produce a result)

### .collect() -- Build a collection
```rust
let v: Vec<i32> = [1, 2, 3].iter().copied().collect();
let s: HashSet<i32> = [1, 2, 2, 3].iter().copied().collect();
let m: HashMap<&str, i32> = [("a", 1), ("b", 2)].iter().copied().collect();
// Type annotation tells collect WHAT to build.
```

### .fold() -- Accumulate with initial value
```rust
[1, 2, 3].iter().fold(0, |acc, x| acc + x)
// 0 + 1 = 1, 1 + 2 = 3, 3 + 3 = 6 → result: 6
```

### .sum() -- Add all elements
```rust
let total: i32 = [1, 2, 3].iter().copied().sum();
// result: 6. Needs type annotation.
```

### .min() / .max() -- Smallest / largest
```rust
[3, 1, 2].iter().min()   // Some(&1)
[3, 1, 2].iter().max()   // Some(&3)
// Returns Option because empty iterator has no min/max.
```

### .count() -- How many elements
```rust
[1, 2, 3].iter().filter(|x| **x > 1).count()
// result: 2
```

### .any() -- Does ANY element match? (short-circuits)
```rust
[1, 2, 3].iter().any(|x| *x > 2)
// result: true (stops at 3)
```

### .all() -- Do ALL elements match? (short-circuits)
```rust
[1, 2, 3].iter().all(|x| *x > 0)
// result: true
```

### .find() -- First element matching a condition
```rust
[1, 2, 3].iter().find(|x| **x > 1)
// result: Some(&2)
```

### .position() -- Index of first match
```rust
["a", "b", "c"].iter().position(|x| *x == "b")
// result: Some(1)
```

### .for_each() -- Run side effect on each element
```rust
[1, 2, 3].iter().for_each(|x| println!("{}", x));
// prints 1, 2, 3. Returns ().
```

### .last() -- Final element
```rust
[1, 2, 3].iter().last()
// result: Some(&3). Consumes entire iterator to get there.
```

---

## HashMap Methods

### Create and insert
```rust
let mut m = HashMap::new();
m.insert("key".to_string(), 42);     // returns Option<old_value>
```

### Read
```rust
m.get("key")           // Option<&V> -- None if missing
m.contains_key("key")  // bool
m["key"]               // panics if missing, avoid this
```

### Entry API (read-or-insert in one shot)
```rust
// Insert default if missing, get mutable ref either way
m.entry("key".to_string()).or_insert(0);

// Insert with closure (lazy -- only runs if key missing)
m.entry("key".to_string()).or_insert_with(Vec::new);

// Modify existing value, or insert default
m.entry("key".to_string())
    .and_modify(|v| *v += 1)
    .or_insert(1);
```

### Iterate
```rust
for (key, value) in &m { }           // borrows: key is &String, value is &V
for (key, value) in &mut m { }       // mutable: value is &mut V
for (key, value) in m { }            // consumes: key is String, value is V

m.keys()        // iterator over &String
m.values()      // iterator over &V
m.values_mut()  // iterator over &mut V
```

---

## Option<T> Methods

### Check
```rust
opt.is_some()       // bool
opt.is_none()       // bool
```

### Extract (unsafe -- panics on None)
```rust
opt.unwrap()        // T or PANIC. Avoid in production.
```

### Extract (safe -- with fallback)
```rust
opt.unwrap_or(default)              // T. Eager: default computed always.
opt.unwrap_or_else(|| compute())    // T. Lazy: closure runs only on None.
opt.unwrap_or_default()             // T. Uses Default trait (0, "", etc.)
```

### Transform inside the Option
```rust
opt.map(|v| v * 2)            // Some(10) → Some(20), None → None
opt.filter(|v| *v > 5)        // Some(10) → Some(10), Some(3) → None, None → None
```

### Chain fallible operations
```rust
opt.and_then(|v| another_option(v))
// Some(x) → calls another_option(x), returns whatever IT returns
// None → returns None, never calls the function
// Use when your transform itself returns Option
```

### Combine Options
```rust
opt1.or(opt2)                  // First Some wins. opt1 if Some, else opt2.
opt1.or_else(|| compute())     // Lazy version.
```

### Borrow inside the Option (avoid moving)
```rust
// When you have &Option<String> and need Option<&str>:
opt.as_ref()        // Option<String> → Option<&String>
opt.as_deref()      // Option<String> → Option<&str>
// as_deref = as_ref + deref. Converts String → &str inside the Option.
```

---

## .map() vs .and_then() -- The ONE difference

Both unwrap Some, pass value to closure, skip None.
The difference is what they do with the closure's return value.

```rust
let x: Option<i32> = Some(5);

// .map() WRAPS the result in Some
x.map(|v| v * 2)              // closure returns 10 → .map() wraps → Some(10)

// .and_then() returns the result AS-IS
x.and_then(|v| Some(v * 2))   // closure returns Some(10) → .and_then() passes through → Some(10)
x.and_then(|v| None)           // closure returns None → .and_then() passes through → None
```

Use `.map()` when your closure returns a plain value.
Use `.and_then()` when your closure returns an Option.

---

## Closure Capture Rules

What determines the closure trait is HOW it uses captured variables:

```
Reads captured variable    → Fn     (can call unlimited times)
Mutates captured variable  → FnMut  (can call unlimited times, needs &mut)
Consumes captured variable → FnOnce (can call only ONCE, value is gone after)
```

`move` does NOT determine the trait. `move` only controls WHETHER
ownership transfers. The trait is determined by what the closure
DOES with the captured value.

```rust
// move + read = Fn
let s = String::from("hi");
let c = move || println!("{}", s);    // Fn: reads s

// move + mutate = FnMut
let mut n = 0;
let c = move || { n += 1; };         // FnMut: mutates n

// move + consume = FnOnce
let v = vec![1, 2];
let c = move || drop(v);             // FnOnce: drops v (consumed)
```

---

## The Dereference Cheat Sheet

```
Reading (method calls, operators)  → auto-deref, no * needed
Writing (assignment through &mut)  → manual * required

&String  → String   → str     (auto-deref chain for method calls)
&Vec<T>  → Vec<T>   → [T]     (auto-deref chain for method calls)
&Box<T>  → Box<T>   → T       (auto-deref chain for method calls)
```

`as_str()` = explicit conversion: `&String → &str`
`as_deref()` on Option = `as_ref()` + `deref()`: `Option<String> → Option<&str>`
`as_ref()` on Option = `Option<T> → Option<&T>`

---

> Glance at this card. Don't memorize it. Use it while drilling.
> After 33 drills, you won't need it anymore.
