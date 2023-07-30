use clap::Parser;

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
    } else {
        println!("Normal User.");
    }


    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}