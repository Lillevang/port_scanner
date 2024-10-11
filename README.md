# ⚡ Fast Port Scanner 🛠️

A blazing-fast asynchronous TCP port scanner built with Rust. This tool lets you scan ports efficiently, detect services running on open ports, and configure various scanning parameters like timeout and concurrency.

## Features 🚀

- Asynchronous Scanning for ultra-fast port scanning 🔥
- Service Detection for well-known ports (e.g., SSH, HTTP) 🔍
- Configurable timeout and concurrency settings ⚙️
- Supports custom port ranges and multi-port scanning 🎯

## Getting Started 🛠️

### Prerequisites

To build and run this project, you need:

- Rust installed on your system
- Cargo (included with Rust)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/your-username/port_scanner.git
cd port_scanner
```

2. Build the project:

```bash
cargo build --release
```

3. Run the port scanner:

```
cargo run --release -- --ip <TARGET_IP> --ports <PORT_RANGE> [OPTIONS]
```

## Example Usage

Scan a host for open ports 22, 80, and 443 with a 2-second timeout:

```bash
cargo run --release -- --ip 192.168.1.1 --ports 22,80,443 --timeout 2
```

Scan a range of ports:

```bash
cargo run --release -- --ip 192.168.1.1 --ports 1-30000
```

## Options

| Option    | Description                                        | Example               |
| --------- | -------------------------------------------------- | --------------------- |
| --ip      | Target IP or hostname to scan (required)           | --ip 192.168.1.1      |
| --ports   | Comma-separated list or range of ports (required)  | --ports 22,80,443-445 |
| --timeout | Timeout for each port scan in seconds (default: 1) | --timeout 2           |


### Output Example 📋

```bash
Port 22 is open (SSH)
Port 80 is open (HTTP)
Port 443 is open (HTTPS)
Open ports: [22, 80, 443]
Scanned 1024 ports
Scan completed in: 1.347s 
```

