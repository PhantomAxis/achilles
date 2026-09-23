// Drill to gain familiarity in iterators, options

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Target {
    ip: String,
    hostname: Option<String>,
    os: Option<String>,
}

#[derive(Debug)]
struct ReconData {
    hosts: Vec<String>,
    ports: Vec<(String, u16, bool)>,
    findings: Vec<(String, String)>,
}

fn label_ports(ports: &[u16]) -> Vec<String> {
    ports.iter().map(|port| format!("port:{}", port)).collect()
}

fn privileged_ports(ports: &[u16]) -> Vec<u16> {
    ports.iter().filter(|&&port| port < 1024).copied().collect()
}

fn passing_labels(scores: &[i32]) -> Vec<String> {
    scores
        .iter()
        .filter(|&&score| score >= 70)
        .map(|score| format!("PASS:{}", score))
        .collect()
}

fn parse_ports(raw: &[&str]) -> Vec<u16> {
    raw.iter().filter_map(|s| s.parse::<u16>().ok()).collect()
}

fn expand_subnets(subnets: &[&str]) -> Vec<String> {
    subnets
        .iter()
        .flat_map(|subnet| (1..=3).map(move |i| format!("{}.{}", subnet, i)))
        .collect()
}

fn join_hostnames(hosts: &[&str]) -> String {
    hosts.iter().fold(String::new(), |acc, host| {
        if acc.is_empty() {
            acc + host
        } else {
            acc + ", " + host
        }
    })
}

fn count_levels(levels: &[&str]) -> HashMap<String, usize> {
    levels.iter().fold(HashMap::new(), |mut acc, level| {
        *acc.entry(level.to_string()).or_insert(0) += 1;
        acc
    })
}

fn greet(username: Option<&str>) -> String {
    username
        .map(|name| format!("Hello, {}!", name.to_uppercase()))
        .unwrap_or_else(|| "Hello, stranger!".to_string())
}

fn format_command(cmd: Option<&str>) -> String {
    cmd.map(|c| c.to_uppercase())
        .map(|cased| format!("CMD:{}", cased))
        .unwrap_or_else(|| "CMD:NOP".to_string())
}

fn find_ip(hostname: &str) -> Option<String> {
    match hostname {
        "web.htb" => Some("10.0.0.1".to_string()),
        "db.htb" => Some("10.0.0.2".to_string()),
        _ => None,
    }
}

fn find_service(ip: &str) -> Option<String> {
    match ip {
        "10.0.0.1" => Some("http".to_string()),
        _ => None,
    }
}

fn lookup_service(hostname: &str) -> Option<String> {
    find_ip(hostname).and_then(|ip| find_service(&ip))
}

fn unique_targets(nmap_ips: &[&str], subfinder_ips: &[&str]) -> Vec<String> {
    let mut uniques: Vec<String> = nmap_ips
        .iter()
        .chain(subfinder_ips)
        .map(|ip| ip.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    uniques.sort();
    uniques
}

fn even_indexed<'a>(items: &[&'a str]) -> Vec<&'a str> {
    items
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, item)| *item)
        .collect()
}

fn open_services(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            let splitted: Vec<&str> = line.split(':').collect();
            if splitted[2] == "OPEN" {
                Some(format!("{}:{}", splitted[0], splitted[1]))
            } else {
                None
            }
        })
        .collect()
}

fn group_ports(pairs: &[(&str, u16)]) -> HashMap<String, Vec<u16>> {
    let mut map = pairs.iter().fold(HashMap::new(), |mut acc, (host, port)| {
        acc.entry(host.to_string())
            .or_insert_with(Vec::new)
            .push(*port);
        acc
    });

    for ports in map.values_mut() {
        ports.sort();
        ports.dedup();
    }

    map
}

fn summary(target: &Target) -> String {
    let mut result = target.ip.clone();
    result += &target
        .hostname
        .as_deref()
        .map(|host| format!(" ({})", host))
        .unwrap_or_default();
    result += &target
        .os
        .as_deref()
        .map(|os| format!(" [{}]", os))
        .unwrap_or_default();

    result
}

fn drill_16() {
    let hosts = vec![
        "web.htb".to_string(),
        "db.htb".to_string(),
        "mail.htb".to_string(),
    ];

    for h in hosts.iter() {
        println!("borrowed: {}", h);
    }
    println!("hosts still alive: {:?}", hosts);

    let mut hosts2 = hosts.clone();
    for h in hosts2.iter_mut() {
        *h = h.to_uppercase();
    }
    println!("mutated: {:?}", hosts2);

    let hosts3 = hosts.clone();
    let collected: Vec<String> = hosts3.into_iter().map(|h| format!("[{}]", h)).collect();
    // println!("hosts3: {:?}", hosts3);
    println!("consumed and transformed: {:?}", collected);
}

fn has_privileged(ports: &[u16]) -> bool {
    ports.iter().any(|&port| port < 1024)
}

fn all_unprivileged(ports: &[u16]) -> bool {
    ports.iter().all(|&port| port >= 1024)
}

fn first_critical<'a>(findings: &[(&'a str, &'a str)]) -> Option<&'a str> {
    findings
        .iter()
        .find(|(_, severity)| *severity == "Critical")
        .map(|(title, _)| *title)
}

fn count_open(statuses: &[(&str, bool)]) -> usize {
    statuses.iter().filter(|(_, isopen)| *isopen).count()
}

fn first_n<'a>(hosts: &[&'a str], n: usize) -> Vec<&'a str> {
    hosts.iter().take(n).copied().collect()
}

fn after_n<'a>(hosts: &[&'a str], n: usize) -> Vec<&'a str> {
    hosts.iter().skip(n).copied().collect()
}

fn total_ports(ports: &[u16]) -> u16 {
    ports.iter().copied().sum()
}

fn lowest_port(ports: &[u16]) -> Option<u16> {
    ports.iter().copied().min()
}

fn highest_port(ports: &[u16]) -> Option<u16> {
    ports.iter().copied().max()
}

fn pair_hosts(names: &[&str], ips: &[&str]) -> Vec<String> {
    names
        .iter()
        .zip(ips.iter())
        .map(|(name, ip)| format!("{} -> {}", name, ip))
        .collect()
}

fn debug_filter(ports: &[u16]) -> Vec<u16> {
    ports
        .iter()
        .copied()
        .inspect(|p| println!(" checking port: {}", p))
        .filter(|p| *p >= 1024)
        .inspect(|p| println!(" kept port: {}", p))
        .collect()
}

fn find_port_index(ports: &[u16], target: u16) -> Option<usize> {
    ports.iter().position(|port| *port == target)
}

fn print_numbered(hosts: &[&str]) {
    hosts
        .iter()
        .enumerate()
        .for_each(|(i, host)| println!("\t[{}] {}", i + 1, host));
}

fn copy_ports(ports: &[u16]) -> Vec<u16> {
    ports.iter().copied().collect()
}

fn clone_names(names: &[String]) -> Vec<String> {
    names.iter().cloned().collect()
}

fn lookup_hosts(dns: &HashMap<String, String>, queries: &[&str]) -> Vec<String> {
    queries
        .iter()
        .map(|&query| {
            dns.get(query)
                .map(|hostname| format!("{} -> {}", query, hostname))
                .unwrap_or_else(|| format!("{} -> NOT FOUND", query))
        })
        .collect()
}

fn word_stats(words: &[&str]) -> HashMap<String, (usize, usize)> {
    words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (index, word)| {
            acc.entry(word.to_string())
                .and_modify(|(count, last)| {
                    *count += 1;
                    *last = index;
                })
                .or_insert((1, index));

            acc
        })
}

fn has_hostname(hostname: &Option<String>) -> bool {
    hostname.is_some()
}

fn best_hostname(primary: Option<&str>, fallback: Option<&str>) -> Option<String> {
    primary.or(fallback).map(|s| s.to_string())
}

fn htb_only(hostname: Option<&str>) -> Option<&str> {
    hostname.filter(|s| s.ends_with(".htb"))
}

fn recon_summary(data: &ReconData) -> String {
    let host_total = data.hosts.len();
    let unique_hosts = data.hosts.iter().collect::<HashSet<&String>>().len();
    let open_ports: Vec<u16> = data
        .ports
        .iter()
        .filter_map(
            |(_, port, is_open)| {
                if *is_open {
                    Some(*port)
                } else {
                    None
                }
            },
        )
        .collect();
    let highest_open_port = open_ports.iter().copied().max().unwrap_or_default();
    let total_open_ports = open_ports.iter().count();
    let total_findings = data.findings.len();
    let severity = data
        .findings
        .iter()
        .fold(HashMap::new(), |mut acc, (_, severity)| {
            *acc.entry(severity.clone()).or_insert(0) += 1;
            acc
        });
    let has_criticals = if data
        .findings
        .iter()
        .any(|(_, severity)| severity == "Critical")
    {
        "YES"
    } else {
        "NO"
    };

    let first_critical = data
        .findings
        .iter()
        .find(|(_, severity)| severity == "Critical")
        .map(|(title, _)| title.to_string())
        .unwrap_or_else(|| "None".to_string());

    format!(
        "=== Recon Summary ===\n\
             Hosts: {} total, {} unique\n\
             Open ports: {} ({} total)\n\
             Highest open port: {}\n\
             Findings: {} total\n\
             \tCritical: {}\n\
             \tHigh: {}\n\
             \tLow: {}\n\
             Has critical findings: {}\n\
             First critical: {}",
        host_total,
        unique_hosts,
        open_ports
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        total_open_ports,
        highest_open_port,
        total_findings,
        severity.get("Critical").map(|n| *n).unwrap_or_default(),
        severity.get("High").map(|n| *n).unwrap_or_default(),
        severity.get("Low").map(|n| *n).unwrap_or_default(),
        has_criticals,
        first_critical
    )
}

fn main() {
    let input_1 = vec![80, 443, 8080];
    println!("Drill 1 Output: {:?}", label_ports(&input_1));

    let input_2 = vec![20, 80, 3306, 443, 8080, 9090];
    println!("Drill 2 Output: {:?}", privileged_ports(&input_2));

    let input_3 = vec![85, 42, 91, 67, 73, 55];
    println!("Drill 3 Output: {:?}", passing_labels(&input_3));

    println!(
        "Drill 4 Output: {:?}",
        parse_ports(&["80", "abc", "443", "", "8080", "99999"])
    );

    println!(
        "Drill 5 Output: {:?}",
        expand_subnets(&["10.0.0", "192.168.1"])
    );

    println!(
        "Drill 6 Output: {:?}",
        join_hostnames(&["web.htb", "db.htb", "mail.htb"])
    );

    println!(
        "Drill 7 Output: {:?}",
        count_levels(&["INFO", "ERROR", "INFO", "WARN", "ERROR", "INFO"])
    );

    println!("Drill 8 Output:");
    println!("\tSome(phantom): {}", greet(Some("phantom")));
    println!("\tNone: {}", greet(None));

    println!("Drill 9 Output:");
    println!("\tSome(scan): {}", format_command(Some("scan")));
    println!("\tNone: {}", format_command(None));

    println!("Drill 10 Output:");
    println!("\tPassing web.htb: {:?}", lookup_service("web.htb"));
    println!("\tPassing db.htb: {:?}", lookup_service("db.htb"));

    println!(
        "Drill 11 Output: {:?}",
        unique_targets(&["10.0.0.1", "10.0.0.2"], &["10.0.0.2", "10.0.0.3"])
    );

    println!(
        "Drill 12 Output: {:?}",
        even_indexed(&["a", "b", "c", "d", "e"])
    );

    println!(
        "Drill 13 Output: {:?}",
        open_services(&[
            "10.0.0.1:80:OPEN",
            "10.0.0.2:22:CLOSED",
            "10.0.0.1:443:OPEN",
            "10.0.0.3:8080:OPEN"
        ])
    );

    println!(
        "Drill 14 Output: {:?}",
        group_ports(&[
            ("web.htb", 80),
            ("db.htb", 3306),
            ("web.htb", 443),
            ("web.htb", 80),
            ("db.htb", 5432)
        ])
    );

    println!(
        "Drill 15 Output: \n\t{}\n\t{}\n\t{}\n\t{}",
        summary(&Target {
            ip: "1.1.1.1".to_string(),
            hostname: Some("web".to_string()),
            os: Some("Linux".to_string())
        }),
        summary(&Target {
            ip: "2.2.2.2".to_string(),
            hostname: None,
            os: Some("Windows".to_string())
        }),
        summary(&Target {
            ip: "3.3.3.3".to_string(),
            hostname: Some("db".to_string()),
            os: None
        }),
        summary(&Target {
            ip: "1.1.1.1".to_string(),
            hostname: None,
            os: None
        })
    );

    println!("Drill 16 Output: ");
    drill_16();

    let input_17_a = [20, 80, 3306, 8080];
    let input_17_b = [3306, 8080, 9090];
    println!(
        "Drill 17 Output: \n\tinput_17_a:\n\t\tis any: {:?}\n\t\tis all: {:?}\n\tinput_17_a:\n\t\tis any: {:?}\n\t\tis all: {:?}",
        has_privileged(&input_17_a),
        all_unprivileged(&input_17_a),
        has_privileged(&input_17_b),
        all_unprivileged(&input_17_b)
    );

    let input_18_a = [("XSS", "High"), ("SQLi", "Critical"), ("IDOR", "Critical")];
    let input_18_b = [("Missing Headers", "Low"), ("Verbose Errors", "Medium")];
    println!(
        "Drill 18 Output: \n\t{:?}: {:?}\n\t{:?}: {:?}",
        input_18_a,
        first_critical(&input_18_a),
        input_18_b,
        first_critical(&input_18_b)
    );

    println!(
        "Drill 19 Output: {}",
        count_open(&[
            ("http", true),
            ("ssh", false),
            ("https", true),
            ("ftp", false)
        ])
    );

    let hosts = ["a.htb", "b.htb", "c.htb", "d.htb", "e.htb"];
    println!(
        "Drill 20 Output: \n\tfirst_n(hosts, 3): {:?}\n\tafter_n(hosts, 3): {:?}",
        first_n(&hosts, 3),
        after_n(&hosts, 3)
    );

    let ports = [80, 443, 22, 8080];
    println!(
        "Drill 24 Output:\n\t{:?}:\n\t\ttotal_ports -> {}\n\t\tlowest_port -> {:?}\n\t\thighest_port -> {:?}\n\t[]:\n\t\ttotal_ports -> {}\n\t\tlowest_port -> {:?}\n\t\thighest_port -> {:?}",
        ports,
        total_ports(&ports),
        lowest_port(&ports),
        highest_port(&ports),
        total_ports(&[]),
        lowest_port(&[]),
        highest_port(&[])
    );

    println!(
        "Drill 25 Output: {:?}",
        pair_hosts(
            &["web", "db", "mail"],
            &["10.0.0.1", "10.0.0.2", "10.0.0.3"]
        )
    );

    println!("Drill 26 Output:");
    println!("final ports{:?}", debug_filter(&[22, 80, 3306, 8080]));

    println!(
        "Drill 27 Output: \n\ttarget -> 443: {:?}\n\ttarget -> 9090: {:?}",
        find_port_index(&ports, 443),
        find_port_index(&ports, 9090)
    );

    println!("Drill 28 Output:");
    print_numbered(&["web.htb", "db.htb", "mail.htb"]);

    println!(
        "Drill 29 Output: \n\tcopied on ports: {:?}\n\tcloned on names: {:?}",
        copy_ports(&ports),
        clone_names(&[
            "Phantom".to_string(),
            "Loves".to_string(),
            "Hacking".to_string()
        ])
    );

    println!(
        "Drill 30 Output: {:?}",
        lookup_hosts(
            &HashMap::from([
                ("10.0.0.1".to_string(), "web.htb".to_string()),
                ("10.0.0.2".to_string(), "db.htb".to_string())
            ]),
            &["10.0.0.1", "10.0.0.3", "10.0.0.2"]
        )
    );

    println!(
        "Drill 31 Output: {:?}",
        word_stats(&["scan", "report", "scan", "alert", "scan"])
    );

    println!("Drill 32 Output:");
    println!(
        "\thas_hostname(&Some(\"web.htb\".to_string())) -> {:?}",
        has_hostname(&Some("web.htb".to_string()))
    );
    println!("\thas_hostname(&None) -> {:?}", has_hostname(&None));
    println!(
        "\tbest_hostname(Some(\"web.htb\"), Some(\"backup.htb\")) -> {:?}",
        best_hostname(Some("web.htb"), Some("backup.htb"))
    );
    println!(
        "\tbest_hostname(None, Some(\"backup.htb\")) -> {:?}",
        best_hostname(None, Some("backup.htb"))
    );
    println!(
        "\tbest_hostname(None, None) -> {:?}",
        best_hostname(None, None)
    );
    println!(
        "\thtb_only(Some(\"web.htb\")) -> {:?}",
        htb_only(Some("web.htb"))
    );
    println!(
        "\thtb_only(Some(\"google.com\")) -> {:?}",
        htb_only(Some("google.com"))
    );
    println!("\thtb_only(None) -> {:?}", htb_only(None));

    let data = ReconData {
        hosts: vec!["10.0.0.1", "10.0.0.2", "10.0.0.1"]
            .into_iter()
            .map(String::from)
            .collect(),
        ports: vec![
            ("10.0.0.1".into(), 22, true),
            ("10.0.0.1".into(), 80, true),
            ("10.0.0.1".into(), 443, true),
            ("10.0.0.2".into(), 3306, false),
        ],
        findings: vec![
            ("XSS Reflected".into(), "High".into()),
            ("SQL Injection".into(), "Critical".into()),
            ("Missing Headers".into(), "Low".into()),
            ("Open Redirect".into(), "High".into()),
        ],
    };
    println!("Drill 33 Output:\n\n{}", recon_summary(&data));
}
