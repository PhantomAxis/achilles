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
    println!(
        "port: {} (u16, {} bytes on stack)",
        port,
        std::mem::size_of::<u16>()
    );
    println!(
        "timeout: {} (f64, {} bytes on stack)",
        timeout,
        std::mem::size_of::<f64>()
    );
    println!(
        "verbose: {} (bool, {} bytes on stack)",
        verbose,
        std::mem::size_of::<bool>()
    );

    // Pointer sizes (on your 64-bit system)
    println!("\nPointer size: {} bytes", std::mem::size_of::<&str>());

    // ----- Heap data -----
    let target = String::from("10.10.10.100");
    let ports = vec![22, 80, 443, 8080, 8443];

    // String and Vec store a POINTER on the stack, DATA on the heap.
    println!("\n=== Heap Data (stack part) ===");
    println!(
        "String struct size on stack: {} bytes",
        std::mem::size_of::<String>()
    );
    println!("  (pointer + length + capacity = 8 + 8 + 8 = 24 bytes on 64-bit)");
    println!(
        "Vec<u16> struct size on stack: {} bytes",
        std::mem::size_of::<Vec<u16>>()
    );

    println!("\n=== Heap Data (actual content) ===");
    println!(
        "target '{}': {} bytes of content on heap",
        target,
        target.len()
    );
    println!(
        "ports {:?}: {} elements × {} bytes = {} bytes on heap",
        ports,
        ports.len(),
        std::mem::size_of::<u16>(),
        ports.len() * std::mem::size_of::<u16>()
    );

    // ----- The difference matters -----
    println!("\n=== Copy vs Move ===");

    // Copy: stack data is cheap to duplicate
    let a: i32 = 42;
    let b = a; // Copies 4 bytes on the stack — instant
    println!("Copied i32: a={}, b={}", a, b);

    // Move: heap data transfer ownership (no data copied)
    let s1 = String::from("Achilles");
    let s2 = s1; // Only copies 24 bytes of stack metadata — the heap data doesn't move
                 // s1 is invalidated — but the actual string bytes are still at the same heap address
    println!("Moved String: s2={}", s2);
    // println!("s1={}", s1);  // ❌ s1 is invalid

    // Clone: explicit deep copy — allocates new heap memory
    let s3 = s2.clone();
    println!(
        "Cloned: s2={}, s3={} (two separate heap allocations)",
        s2, s3
    );

    // ----- Why this matters for Achilles -----
    println!("\n=== Achilles Context ===");
    println!("When nmap produces 10MB of XML output:");
    println!("  Move: transfers the 24-byte pointer — O(1), instant");
    println!("  Clone: copies 10MB of heap data — O(n), expensive");
    println!("  In Achilles pipeline: data moves between stages, never cloned unnecessarily");
}
