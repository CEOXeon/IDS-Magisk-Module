use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

#[allow(non_snake_case)]
fn main() {
    // println!("Hello, world!");
    clap();
}



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn clap() {
    let args = Args::parse();

    if args.name == "Censored" || args.name == "CEOXeon" {
        println!("Hi, Admin.");
    } else if args.name == "MOD" || args.name == "Money" {
        println!("Hi, the M's are meeting.");
    }
      else if args.name == "port" {
        port_scanner()
    } else {
        println!("Normal User.");
    }


    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

fn is_port_open(host: &str, port: u16) -> bool {
    match format!("{}:{}", host, port).to_socket_addrs() {
        Ok(addresses) => {
            for address in addresses {
                if let Ok(_) = TcpStream::connect_timeout(&address, Duration::from_nanos(1)) {
                    return true;
                }
            }
            false
        }
        Err(_) => false,
    }
}

fn scan_ports(host: &str, start_port: u16, end_port: u16) -> Vec<u16> {
    let mut open_ports = Vec::new();

    for port in start_port..=end_port {
        if is_port_open(host, port) {
            open_ports.push(port);
        }
    }

    open_ports
}

fn port_scanner() {
    let host = "127.0.0.1";
    let start_port = 1;
    let end_port = 16384;

    let open_ports = scan_ports(host, start_port, end_port);

    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        println!("Open ports:");
        for port in open_ports {
            println!("Port {} is open", port);
        }
    }
}