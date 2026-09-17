// Building an simplified version of the ADCObject enum to understand the Achilles data contract pattern

use std::fmt;

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
}

#[derive(Debug, Clone)]
struct Finding {
    title: String,
    severity: String,
    host_ip: String,
}

#[derive(Debug)]
enum ADCObject {
    Host(Host),
    Port(Port),
    Finding(Finding),
}

impl fmt::Display for ADCObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Host(Host { ip, hostname }) => {
                let host = hostname.as_deref().unwrap_or("Unknown");
                write!(f, "Target details: ip/host -> {} ({})", ip, host)
            }
            Self::Port(Port {
                number,
                protocol,
                state,
            }) => {
                write!(
                    f,
                    "Port details:\n\tPort Number: {}\n\tProtocol: {}\n\tState: {}",
                    number, protocol, state
                )
            }
            Self::Finding(Finding {
                title,
                severity,
                host_ip,
            }) => {
                write!(
                    f,
                    "Findings: {}\n\tSeverity level: {}\n\tHost IP: {}",
                    title, severity, host_ip
                )
            }
        }
    }
}

fn process_objects(objects: &[ADCObject]) {
    let mut host_count = 0;
    let mut port_count = 0;
    let mut finding_count = 0;

    for obj in objects {
        match obj {
            ADCObject::Host(_) => host_count += 1,
            ADCObject::Port(_) => port_count += 1,
            ADCObject::Finding(_) => finding_count += 1,
        }
    }

    println!(
        "Hosts: {}, Ports: {}, Findings: {}",
        host_count, port_count, finding_count
    );
}

fn main() {
    let host_1 = ADCObject::Host(Host {
        ip: "192.168.12.4".to_string(),
        hostname: None,
    });
    let host_2 = ADCObject::Host(Host {
        ip: "10.255.155.81".to_string(),
        hostname: Some("HellFire - Phoenix".to_string()),
    });
    let port_1 = ADCObject::Port(Port {
        number: 80,
        protocol: "http".to_string(),
        state: "OPEN".to_string(),
    });
    let port_2 = ADCObject::Port(Port {
        number: 8080,
        protocol: "https".to_string(),
        state: "CLOSED".to_string(),
    });
    let port_3 = ADCObject::Port(Port {
        number: 22,
        protocol: "ssh".to_string(),
        state: "FILTERED".to_string(),
    });
    let findings_1 = ADCObject::Finding(Finding {
        title: "nmap scan".to_string(),
        severity: "LOW".to_string(),
        host_ip: "10.255.155.81".to_string(),
    });

    println!("\n=== Achilles Data Contract preview ===\n");
    let objects = vec![host_1, host_2, port_1, port_2, port_3, findings_1];
    process_objects(&objects);
    println!();

    println!("Host 1: {}", objects[0]);
    println!("Host 2: {}\n", objects[1]);
    println!("Port 1: {}", objects[2]);
    println!("Port 2: {}", objects[3]);
    println!("Port 3: {}\n", objects[4]);
    println!("Findings 1: {}", objects[5]);
}
