#[allow(non_snake_case)]
use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    // println!("Hello, world!");
    clap();
}



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    name: String,

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
        port_scanner_experimental()
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
    let target_port_1: u16 = 5555; // ADB 
    let target_port_2: u16 = 6666; // IRC
    let target_port_3: u16 = 23;   // Telnet
    let target_port_4: u16 = 1900; // UPnP

    let host = "127.0.0.1";
    let start_port = 1;
    let end_port = 16384;

    let open_ports = scan_ports(host, start_port, end_port);

    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        println!("Open ports:");
        for port in &open_ports {
            println!("Port {} is open", port);
        };
        for port in &open_ports {
            if port.to_string() == target_port_1.to_string() {
                println!("Found a match: {}", port);
            }
        };
        for port in &open_ports {
            if port.to_string() == target_port_2.to_string() {
                println!("Found a match: {}", port);
            }
        };  
        for port in &open_ports {
            if port.to_string() == target_port_3.to_string() {
                println!("Found a match: {}", port);
            }
        };
        for port in &open_ports {
            if port.to_string() == target_port_4.to_string() {
                println!("Found a match: {}", port);
            }
        };
    }
}

fn port_scanner_experimental() {
    let mut target_ports: HashMap<u16, Vec<&str>> = HashMap::new();
    target_ports.insert(5555, vec!["ls", "ls"]);
    target_ports.insert(6666, vec!["ls"]);
    target_ports.insert(23, vec!["ls"]);
    target_ports.insert(1900, vec!["ls"]);
    

    let host = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;

    let open_ports = scan_ports(host, start_port, end_port);

    if open_ports.is_empty() {
        println!("No open ports found.");
    } else {
        println!("Open ports:");
        for port in &open_ports {
            println!("Port {} is open", port);

            if let Some(commands) = target_ports.get(port) {
                println!("Commands for port {}:", port);
                for command in commands {
                    println!(" - {}", command);
                    
                    for command in commands {
                        println!(" - Executing: {}", command);
                        let output = Command::new("sh ")
                            .arg("-c")
                            .arg(command)
                            .output();

                        match output {
                            Ok(output) => {
                                println!("   Output: {:?}", output.stdout);
                            }
                            Err(err) => {
                                eprintln!("   Error executing command: {}", err);
                            }
                        }
                    }
                }
            }
        }
    }
}


