// Day 1: Ownership — understanding moves
//
// Run with: cargo run --bin day1_ownership

fn main() {
    // ----- PART A: The move trap -----
    let target = String::from("10.10.10.100");

    // This function "consumes" target — takes ownership
    validate_target(&target);

    // Uncomment the line below. Read the compiler
    println!("Scanning target: {}", target);

    // TASK: Make this work by using ONE of the three techniques:
    //   1. Return ownership from validate_target
    //   2. Clone target before passing
    //   3. (Preview of Day 2) Pass a reference with &
    //
    // Try all three. Understand the trade-offs.

    // ----- PART B: Copy types vs Move types -----
    let port: u16 = 8080;
    let port_copy = port; // Copy — both are valid
    println!("Original port: {}", port); // ✅ Works
    println!("Copied port: {}", port_copy); // ✅ Works

    let service = String::from("http");
    let service_moved = service; // Move — service is now invalid
                                 // println!("Service: {}", service); // ❌ Uncomment to see the error
    println!("Moved service: {}", service_moved);

    // ----- PART C: Ownership with Vec -----
    let ports = vec![22, 80, 443, 8080, 8443];

    // TASK: Call print_ports with ports, then try to use ports again.
    //       Fix it using clone or by returning ownership.
    print_ports(ports.clone());
    println!("Port count: {}", ports.len()); // ❌ Uncomment to see the error

    // ----- PART D: Scope and automatic drop -----
    {
        let scan_output = String::from("PORT   STATE SERVICE\n22/tcp open  ssh\n80/tcp open  http");
        println!("Inside scope:\n{}", scan_output);
    } // scan_output is dropped here — memory freed

    // println!("{}", scan_output);  // ❌ Uncomment — scan_output doesn't exist

    // ----- PART E: Multiple moves -----
    let host = String::from("scanme.nmap.org");
    let host2 = host; // Move 1: host → host2
    let host3 = host2; // Move 2: host2 → host3
                       // Both host and host2 are now invalid. Only host3 owns the data.
    println!("Final owner: {}", host3);
}

fn validate_target(target: &String) {
    if target.is_empty() {
        println!("ERROR: empty target");
    } else {
        println!("Target '{}' is valid ({} chars)", target, target.len());
    }
} // target is dropped here

fn print_ports(ports: Vec<u16>) {
    print!("Ports: ");
    for port in &ports {
        print!("{} ", port);
    }
    println!();
} // ports is dropped here
