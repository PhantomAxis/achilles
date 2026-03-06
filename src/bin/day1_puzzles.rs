// Day 1: Ownership puzzles
//
// For each section: PREDICT whether it compiles, then uncomment and verify.
// Run with: cargo run --bin day1_puzzles

fn main() {
    // ----- Puzzle 1: Does this compile? -----
    let tools = vec!["nmap", "subfinder", "httpx"];
    let first = tools[0]; // What type is first? &str — Copy!
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
    data = String::from("new scan result"); // Re-assigning data — is this allowed?
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
    result // Ownership moves to caller — no copy, no clone
}
