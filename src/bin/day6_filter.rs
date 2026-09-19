// Writing a function that takes Vec<String> of hostnames, removes duplicates, sorts and returns a Vec<String>

fn clean_hostnames(mut hostnames: Vec<String>) -> Vec<String> {
    hostnames.iter_mut().for_each(|h| *h = h.to_lowercase());
    hostnames.sort();
    hostnames.dedup();

    hostnames
}

fn main() {
    let hosts = vec![
        "target.htb".to_string(),
        "admin.target.htb".to_string(),
        "target.htb".to_string(),
        "mail.target.htb".to_string(),
        "admin.target.htb".to_string(),
        "dev.target.htb".to_string(),
        "mail.target.htb".to_string(),
    ];

    println!("Before: {} hostnames", hosts.len());
    let clean = clean_hostnames(hosts);
    println!("After: {} hostnames", clean.len());
    for host in &clean {
        println!("\t{}", host);
    }
}
