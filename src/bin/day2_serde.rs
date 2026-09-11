// Day 2: Serde — serialization and deserialization
//
// This is how Achilles will convert between Rust types and JSON.
// Every ADC type (Host, Port, Finding) will derive Serialize + Deserialize.
//
// Run with: cargo run --bin day2_serde

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simplified version of the Achilles Host type.
/// The derive macros auto-generate serialization code.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Host {
    id: String,
    ip: String,
    hostnames: Vec<String>,
    open_ports: Vec<u16>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Port {
    number: u16,
    protocol: String,
    state: String,
    service: Option<String>,
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    println!("=== Serde: Rust ↔ JSON ===\n");

    // ----- PART A: Serialize a HashMap to JSON -----
    let text = "PORT STATE SERVICE\n22/tcp open ssh\n80/tcp open http\n22/tcp open ssh";
    let word_counts = count_words(text);

    let json = serde_json::to_string_pretty(&word_counts).expect("Failed to serialize");
    println!("HashMap → JSON:\n{}\n", json);

    // Deserialize it back
    let deserialized: HashMap<String, usize> =
        serde_json::from_str(&json).expect("Failed to deserialize");

    // Assert round-trip is lossless
    assert_eq!(word_counts, deserialized);
    println!("✅ Round-trip HashMap → JSON → HashMap: lossless\n");

    // ----- PART B: Serialize a struct -----
    let host = Host {
        id: "host-001".to_string(),
        ip: "10.10.10.100".to_string(),
        hostnames: vec!["target.local".to_string(), "metasploitable.lab".to_string()],
        open_ports: vec![22, 80, 443, 3306, 5432],
    };

    let host_json = serde_json::to_string_pretty(&host).expect("Failed to serialize Host");
    println!("Host → JSON:\n{}\n", host_json);

    // Deserialize back to a Host struct
    let host_restored: Host = serde_json::from_str(&host_json).expect("Failed to deserialize Host");

    assert_eq!(host, host_restored);
    println!("✅ Round-trip Host → JSON → Host: lossless\n");

    // ----- PART C: Handle invalid JSON -----
    let bad_json = r#"{"id": "host-002", "ip": 12345}"#; // ip should be a string, not a number

    match serde_json::from_str::<Host>(bad_json) {
        Ok(h) => println!("Parsed: {:?}", h),
        Err(e) => {
            println!("❌ Failed to parse bad JSON: {}", e);
            println!("   (This is good — serde caught the type mismatch at runtime)");
        }
    }

    // ----- PART D: Serialize to file, read back -----
    let path = "/tmp/achilles_host.json";
    let json_bytes = serde_json::to_string_pretty(&host).unwrap();
    std::fs::write(path, &json_bytes).expect("Failed to write file");
    println!("\n✅ Wrote Host to {}", path);

    let file_contents = std::fs::read_to_string(path).expect("Failed to read file");
    let file_host: Host = serde_json::from_str(&file_contents).expect("Failed to parse file");
    assert_eq!(host, file_host);
    println!("✅ Read Host back from file: lossless round-trip\n");

    // Clean up
    let _ = std::fs::remove_file(path);

    // ----- WHY THIS MATTERS -----
    println!("=== Achilles Context ===");
    println!("Every ADC type (Host, Port, Finding, Credential) will derive");
    println!("Serialize + Deserialize. This means:");
    println!("  • nmap XML → parsed → Host struct → JSON → passed to next node");
    println!("  • Workflow state saved to disk as JSON");
    println!("  • API responses serialized automatically");
    println!("Serde is the foundation of all data flow in Achilles.");

    // TASK: Add a Port struct with fields: number (u16), protocol (String),
    //       state (String), service (Option<String>).
    //       Derive Serialize + Deserialize.
    //       Create a Vec<Port>, serialize to JSON, deserialize back,
    //       assert equality.

    let datas = vec![
        Port {
            number: 80,
            protocol: "tcp".to_string(),
            state: "open".to_string(),
            service: Some("http".to_string()),
        },
        Port {
            number: 443,
            protocol: "tcp".to_string(),
            state: "open".to_string(),
            service: None,
        },
    ];

    let json = serde_json::to_string_pretty(&datas).expect("Failed to create json");
    println!("Json created successfully:\n\n {}\n", json);

    let deserialized_datas: Vec<Port> =
        serde_json::from_str(&json).expect("Failed to convert to Vec");
    assert_eq!(datas, deserialized_datas);
    println!("Successfully converted");
}
