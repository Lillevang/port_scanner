use std::net::{Ipv4Addr, Ipv6Addr};

pub fn validate_ip_or_hostname(ip_or_hostname: &str) -> Result<(), &'static str> {
    if ip_or_hostname.parse::<Ipv4Addr>().is_ok() {
        return Ok(());
    }

    if ip_or_hostname.parse::<Ipv6Addr>().is_ok() {
        return Ok(());
    }

    if is_valid_hostname(ip_or_hostname) {
        return Ok(());
    }

    Err("Invalid IP address or hostname")
}

fn is_valid_hostname(hostname: &str) -> bool {
    // Check length (must be 1-253 characters)
    if hostname.len() < 1 || hostname.len() > 253 {
        return false;
    }

    // Reject purely numeric hostnames that resemble an IP
    if hostname.chars().all(|c| c.is_digit(10) || c == '.') {
        return false;
    }

    // Check valid characters (alphanumeric, hyphen, dot) and format
    let valid_hostname = hostname
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '.')
        && !hostname.starts_with('-')
        && !hostname.ends_with('-')
        && !hostname.contains("..");

    valid_hostname
}

pub fn parse_ports(ports: &str) -> Result<Vec<u16>, &'static str> {
    let mut port_list = Vec::new();

    // Split the input by commas to handle list of ports
    for part in ports.split(',') {
        let part = part.trim();

        // Check if it's a range (e.g., 20-80)
        if let Some((start, end)) = part.split_once('-') {
            let start: u16 = start.parse().map_err(|_| "invalid start of range")?;
            let end: u16 = end.parse().map_err(|_| "Invalid end of range")?;
            if start > end {
                return Err("Start of range is greater than end");
            }
            port_list.extend(start..=end);
        } else {
            // Handle single ports
            let port: u16 = part.parse().map_err(|_| "Invalid Port")?;
            port_list.push(port)
        }
    }

    // Ensure all ports are valid (between 1 and 65535)
    if port_list.iter().any(|&port| port < 1) {
        return Err("Port out of range");
    }

    Ok(port_list)
}
