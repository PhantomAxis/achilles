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
    config.ports = vec![
        1, 21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445, 993, 995, 1723, 3306, 3389, 5432,
        5900, 8080, 8443,
    ];
    config.timeout_secs = 10;
}

fn sort_and_return(data: &mut Vec<i32>) -> &Vec<i32> {
    data.sort();
    data // Returns an immutable reference to the sorted data
}
