// Day 2 Borrowing - word counter
//
// The function borrows text with &str - it reads without owning
// Run with: cargo run --bin day2_borrowing

use std::collections::HashMap;

/// Count the frequency of each word in the given text.
///
/// Takes &str (a borrow) - the caller keeps ownership of the original text.
/// Returns a HashMap that the caller OWNS (new data created inside the function).
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        // word is a &str - a slice of the original text
        // we need to own it to store in the HashMap, so we call .to_string()
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

    // We pass &nmap_output - borrowing not moving
    let word_counts = count_words(&nmap_output);

    // nmap_output is still valid - it was only borrowed
    println!("Original text length: {} chars", nmap_output.len());
    println!("\nWord frequencies:");

    // Collect into a Vec and sort for consistent output
    let mut sorted: Vec<_> = word_counts.iter().collect();
    sorted.sort_by_key(|(word, _)| *word);

    for (word, count) in &sorted {
        println!("  {:15} -> {}", word, count);
    }

    // TASK 1: why does count_words take &str instead of String?
    //         Write your own answer as a comment below.
    // Answer: We need to own the text since we are using it again and thus for that reason we are just lending the text and not giving them the ownership.

    // TASK 2: what would happen if count_words took String instead of &str?
    // Answer: Compilation error. Cuz we are not owning the text anymore and thus the nmap_output cannot be used again since ownership is transferred.

    // TASK 3: Call count_words TWICE with the same text.
    let _counts_again = count_words(nmap_output);
    println!("\nCalled count_words twice with the same text");

    // TASK 4: Call count_words with a string literal directly.
    let _literal_counts = count_words("hello world hello");
    println!("Called count_words with a literal");
}
