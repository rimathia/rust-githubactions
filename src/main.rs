use rust_githubactions::saturating_add;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    println!("100 + 200 = {}", saturating_add(100, 200));
    for _ in 0..args.count {
        println!("Hello {}", args.name);
    }
}
