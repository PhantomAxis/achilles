// Writing functions that use option combinators instead of match blocks

#[derive(Debug)]
struct ScanResult {
    ip: String,
    hostname: Option<String>,
    os: Option<String>,
    service: Option<String>,
}

fn display_target(result: &ScanResult) -> String {
    result
        .hostname
        .as_deref()
        .map(|host| format!("{} ({})", host, result.ip))
        .unwrap_or_else(|| result.ip.clone())
}

fn get_os_label(result: &ScanResult) -> String {
    result
        .os
        .as_deref()
        .map(|os| os.to_uppercase())
        .unwrap_or_else(|| "Unknown OS".to_string())
}

fn service_description(result: &ScanResult) -> String {
    result
        .service
        .as_deref()
        .map(|service| format!("{} on {}", service, result.ip))
        .unwrap_or_else(|| format!("no service on {}", result.ip))
}

fn short_label(result: &ScanResult) -> String {
    result
        .hostname
        .as_deref()
        .map(|host| host.to_uppercase())
        .map(|cased| cased.chars().take(10).collect::<String>())
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn main() {
    println!("\n=== Option Combinators ===\n");
    let target_1 = ScanResult {
        ip: "10.0.0.1".to_string(),
        hostname: Some("web.target.htb".to_string()),
        os: Some("linux 5.4".to_string()),
        service: Some("http".to_string()),
    };
    let target_2 = ScanResult {
        ip: "10.0.0.2".to_string(),
        hostname: None,
        os: None,
        service: None,
    };

    println!("Target 1: {}", display_target(&target_1));
    println!("Target 2: {}", display_target(&target_2));
    println!("OS 1: {}", get_os_label(&target_1));
    println!("OS 2: {}", get_os_label(&target_2));
    println!("Service 1: {}", service_description(&target_1));
    println!("Service 2: {}", service_description(&target_2));
    println!("Label 1: {}", short_label(&target_1));
    println!("Label 2: {}", short_label(&target_2));
}
