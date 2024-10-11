mod config;
mod scanner;

use clap::Parser;
use config::args::{parse_ports, validate_ip_or_hostname};
use scanner::tcp::scan_ports;
use tokio;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value = "1-1024")]
    ports: String,

    #[arg(short, long, default_value_t = 1)]
    timeout: u8,

    #[arg(short, long, default_value_t = 4)]
    concurrency: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(err) = validate_ip_or_hostname(&args.ip) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let port_list = match parse_ports(&args.ports) {
        Ok(port_list) => port_list,
        Err(err) => {
            eprintln!("Error parsing ports: {}", err);
            std::process::exit(1);
        }
    };

    scan_ports(args.ip, port_list, args.timeout as u64).await;
}
