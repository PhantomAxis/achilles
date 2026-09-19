// Processing on Vec<ADCObject> using iterator chains.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Host {
    ip: String,
    hostname: Option<String>,
}

#[derive(Debug, Clone)]
struct Port {
    number: u16,
    protocol: String,
    state: String,
    host_ip: String,
}

#[derive(Debug, Clone)]
struct Finding {
    title: String,
    severity: String,
    host_ip: String,
}

#[derive(Debug, Clone)]
enum ADCObject {
    Host(Host),
    Port(Port),
    Finding(Finding),
}

fn extract_hosts(objects: &[ADCObject]) -> Vec<&Host> {
    objects
        .iter()
        .filter_map(|obj| match obj {
            ADCObject::Host(host) => Some(host),
            _ => None,
        })
        .collect()
}

fn unique_ips(objects: &[ADCObject]) -> Vec<String> {
    let ips: HashSet<&str> = objects
        .iter()
        .filter_map(|obj| match obj {
            ADCObject::Host(host) => Some(host.ip.as_str()),
            _ => None,
        })
        .collect();

    ips.iter().map(|ip| ip.to_string()).collect()
}

fn count_by_severity(objects: &[ADCObject]) -> HashMap<String, usize> {
    objects
        .iter()
        .fold(HashMap::new(), |mut acc, obj| match obj {
            ADCObject::Finding(finding) => {
                *acc.entry(finding.severity.clone()).or_insert(0) += 1;
                acc
            }
            _ => acc,
        })
}

fn ports_above(objects: &[ADCObject], threshold: u16) -> Vec<&Port> {
    objects
        .iter()
        .filter_map(|obj| match obj {
            ADCObject::Port(port) => {
                if port.number > threshold {
                    Some(port)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn open_targets(objects: &[ADCObject]) -> Vec<String> {
    objects
        .iter()
        .filter_map(|obj| match obj {
            ADCObject::Port(port) => {
                if port.state == "OPEN" {
                    Some(format!("{}:{}", port.host_ip, port.number))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

fn main() {
    let objects = vec![
        ADCObject::Host(Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web.target.htb".to_string()),
        }),
        ADCObject::Host(Host {
            ip: "10.0.0.2".to_string(),
            hostname: None,
        }),
        ADCObject::Host(Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("admin.target.htb".to_string()),
        }),
        ADCObject::Port(Port {
            number: 80,
            protocol: "tcp".to_string(),
            state: "OPEN".to_string(),
            host_ip: "10.0.0.1".to_string(),
        }),
        ADCObject::Port(Port {
            number: 443,
            protocol: "tcp".to_string(),
            state: "OPEN".to_string(),
            host_ip: "10.0.0.1".to_string(),
        }),
        ADCObject::Port(Port {
            number: 22,
            protocol: "tcp".to_string(),
            state: "CLOSED".to_string(),
            host_ip: "10.0.0.2".to_string(),
        }),
        ADCObject::Port(Port {
            number: 3306,
            protocol: "tcp".to_string(),
            state: "OPEN".to_string(),
            host_ip: "10.0.0.2".to_string(),
        }),
        ADCObject::Finding(Finding {
            title: "XSS Reflected".to_string(),
            severity: "High".to_string(),
            host_ip: "10.0.0.1".to_string(),
        }),
        ADCObject::Finding(Finding {
            title: "SQL Injection".to_string(),
            severity: "Critical".to_string(),
            host_ip: "10.0.0.1".to_string(),
        }),
        ADCObject::Finding(Finding {
            title: "Missing Headers".to_string(),
            severity: "Low".to_string(),
            host_ip: "10.0.0.2".to_string(),
        }),
        ADCObject::Finding(Finding {
            title: "Open Redirect".to_string(),
            severity: "High".to_string(),
            host_ip: "10.0.0.1".to_string(),
        }),
    ];

    println!("Hosts: {:?}", extract_hosts(&objects));
    println!("Unique IPs: {:?}", unique_ips(&objects));
    println!("Severity counts: {:?}", count_by_severity(&objects));
    println!("Ports above 100: {:?}", ports_above(&objects, 100));
    println!("Open targets: {:?}", open_targets(&objects));
}
