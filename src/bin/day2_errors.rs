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
    let contents = read_file(path)?; // ? propagates file errors

    let mut counts = HashMap::new();
    for word in contents.split_whitespace() {
        let count = counts.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    Ok(counts)
}

fn concat_files(path1: &str, path2: &str) -> Result<String, io::Error> {
    let contents_of_file_1 = fs::read_to_string(path1)?;
    let contents_of_file_2 = fs::read_to_string(path2)?;

    let concatenated_file = contents_of_file_1 + &contents_of_file_2;

    Ok(concatenated_file)
}

fn main() {
    println!("=== Error Handling with Result<T, E> ===\n");

    // ----- SUCCESS CASE -----
    // Create a test file to read
    let test_path = "/tmp/achilles_day2_test.txt";
    fs::write(
        test_path,
        "PORT STATE SERVICE\n22/tcp open ssh\n80/tcp open http",
    )
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

    let path_1 = "/tmp/hello_phantom.txt";
    fs::write(path_1, "Hey Phantom, how is it going?\n").expect("Failed to create test file");

    let path_2 = "/tmp/bye_phantom.txt";
    fs::write(path_2, "Bye Phantom, see you tomorrow").expect("Failed to create test file");

    match concat_files(path_1, path_2) {
        Ok(contents) => println!(
            "File concatenated successfully.\nContents:\n\n{}\n",
            contents
        ),
        Err(e) => println!("Error: {}", e),
    }

    match concat_files(path_1, "~/hello.txt") {
        Ok(contents) => println!(
            "File concatenated successfully.\nContents:\n\n{}\n",
            contents
        ),
        Err(e) => println!("Error: {}", e),
    }

    match concat_files("~/null.txt", path_2) {
        Ok(contents) => println!(
            "File concatenated successfully.\nContents:\n\n{}\n",
            contents
        ),
        Err(e) => println!("Error: {}", e),
    }
}
