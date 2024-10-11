use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

pub async fn scan_port(ip: &str, port: u16, timeout_duration: u64) -> bool {
    let address = format!("{}:{}", ip, port);
    let timeout_duration = Duration::from_secs(timeout_duration);

    match timeout(timeout_duration, TcpStream::connect(&address)).await {
        Ok(Ok(_)) => {
            println!("Port {} is open", port);
            true
        }
        _ => false,
    }
}

pub async fn scan_ports(ip: String, ports: Vec<u16>, timeout_duration: u64) {
    let mut tasks = vec![];
    let mut open_ports = vec![];
    let port_len = ports.len();

    for port in ports {
        let ip = ip.clone();
        // Spawn a task for each port scan
        let task = tokio::spawn(async move {
            if scan_port(&ip, port, timeout_duration).await {
                Some(port)
            } else {
                None
            }
        });
        tasks.push(task);
    }

    // Await all tasks to finish
    for task in tasks {
        if let Ok(Some(port)) = task.await {
            open_ports.push(port);
        }
    }

    println!("Open ports: {:?}", open_ports);
    println!("Scanned {} ports", port_len);
}
