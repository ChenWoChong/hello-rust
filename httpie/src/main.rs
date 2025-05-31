use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "1.0.0", author = "chenwochong")]
struct Opts {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

struct Get {
    url: String,
}

struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    let opt = Opts::parse();
    println!("Hello, world! {}, count {}", opt.name, opt.count);
}
