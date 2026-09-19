// Writing a function that takes Vec<Host> and groups them into HashMap<String, Vec<Host>> keyed by IP address

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
    source: String,
}

fn group_by_ip(hosts: Vec<Host>) -> HashMap<String, Vec<Host>> {
    let mut groups = HashMap::new();

    for host in hosts {
        groups
            .entry(host.ip.clone())
            .or_insert_with(Vec::new)
            .push(host);
    }

    groups
}

fn main() {
    let hosts = vec![
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web.target.htb".to_string()),
            source: "subfinder".to_string(),
        },
        Host {
            ip: "10.0.0.2".to_string(),
            hostname: None,
            source: "nmap".to_string(),
        },
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: None,
            source: "nmap".to_string(),
        },
        Host {
            ip: "10.0.0.3".to_string(),
            hostname: Some("db.target.htb".to_string()),
            source: "subfinder".to_string(),
        },
        Host {
            ip: "10.0.0.2".to_string(),
            hostname: Some("api.target.htb".to_string()),
            source: "subfinder".to_string(),
        },
    ];

    let groups = group_by_ip(hosts);

    println!("\n=== Hosts grouped by IP===\n");
    for (key, value) in &groups {
        println!("{} ({} entries)", key, value.len());
        for host in value {
            println!(
                "\t- hostname: {}, source: {}",
                host.hostname.as_deref().unwrap_or("Unknown"),
                host.source
            );
        }
        println!();
    }
}
