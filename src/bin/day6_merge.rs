// Simulating Achilles merge nodes

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
    ports: Vec<u16>,
    source: String,
}

fn merge_hosts(hosts: Vec<Host>) -> Vec<Host> {
    let mapped = hosts.into_iter().fold(HashMap::new(), |mut acc, host| {
        acc.entry(host.ip.clone())
            .or_insert_with(Vec::new)
            .push(host);
        acc
    });

    mapped
        .into_iter()
        .map(|(ip, host)| Host {
            ip,
            hostname: host.iter().filter_map(|h| h.hostname.clone()).next(),
            // My trick
            // .find(|h| h.hostname.is_some())
            // .map(|h| h.hostname.as_deref().unwrap().to_string()),
            ports: {
                let mut merged_ports: Vec<u16> =
                    host.iter().flat_map(|h| h.ports.iter().copied()).collect();
                merged_ports.sort();
                merged_ports.dedup();
                merged_ports
            },
            source: {
                let mut sources: Vec<String> = host.iter().map(|h| h.source.clone()).collect();
                sources.sort();
                sources.dedup();
                if sources.len() > 1 {
                    format!("merged ({})", sources.join(", "))
                } else {
                    sources[0].clone()
                }
            },
        })
        .collect()
}

fn generate_report(hosts: &[Host]) -> String {
    let mut result = String::new();
    for host in hosts {
        result += &format!(
            "\n[{}] {}\n\
                            \tPorts: {}\n\
                            \tSource: {}\n",
            host.ip,
            host.hostname.as_deref().unwrap_or_default(),
            if !host.ports.is_empty() {
                host.ports
                    .iter()
                    .copied()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            } else {
                "(none)".to_string()
            },
            host.source
        );
    }

    result
}

fn main() {
    let nmap_hosts = vec![
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: None,
            ports: vec![22, 80, 443],
            source: "nmap".to_string(),
        },
        Host {
            ip: "10.0.0.2".to_string(),
            hostname: None,
            ports: vec![80, 8080],
            source: "nmap".to_string(),
        },
    ];

    let subfinder_hosts = vec![
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web.target.htb".to_string()),
            ports: vec![],
            source: "subfinder".to_string(),
        },
        Host {
            ip: "10.0.0.3".to_string(),
            hostname: Some("db.target.htb".to_string()),
            ports: vec![],
            source: "subfinder".to_string(),
        },
    ];

    println!("=== Merge Node Output ===");
    let all_hosts = nmap_hosts.into_iter().chain(subfinder_hosts).collect();
    let merged = merge_hosts(all_hosts);
    let report = generate_report(&merged);
    println!("{}", report);
}
