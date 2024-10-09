use std::net::SocketAddr;
use clap::Parser;
use async_std::task;
use futures::future::join_all;
use async_port_scanner::Scanner;
use std::time::Duration;

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
    println!("{:?}", my_addrs);
}
