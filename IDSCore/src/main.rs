use std::net::SocketAddr;
use clap::Parser;
use async_std::task;
use async_port_scanner::Scanner;
use std::time::Duration;
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
        idscore_scan_ports();
    } else {
        println!("Normal User.");
    }
}

fn idscore_scan_ports() {
    let ps = Scanner::new(Duration::from_secs(2));

    let ftr = ps.run_batched("127.0.0.1".to_string(), 1, 65535, 10000);
    let my_addrs: Vec<SocketAddr> = task::block_on(async { ftr.await });

    // Extract only the ports from the addresses
    let mut ports: Vec<u16> = my_addrs.into_iter().map(|addr| addr.port()).collect();

    println!("{:?}", ports);
    ports.sort();

    idscore_log(ports);

}

fn idscore_log(ports: Vec<u16>) {
    for port in ports.iter() {
        port.to_string();
        if *port == 22 {
            println!("SSH Port Open");
            let cmd: String = r#"su -lp 2000 -c "cmd notification post -S bigtext -t 'IDS Magisk Module' 'SSH Port Alert' 'SSH Port is open /// Executing Emergency Response'""#.to_string();
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect("failed to execute process");
        } else if *port == 80 {
            println!("HTTP Port Open");
            Command::new("sh")
                .arg("-c")
                .arg("echo 'HTTP Port Open' >> /var/log/idscore.log")
                .output()
                .expect("failed to execute process");
        } else if *port == 443 {
            println!("HTTPS Port Open");
        } else {
            println!("Port {} is Open", port);
        }
    }
}
