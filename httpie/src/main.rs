use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "1.0.0", author = "chenwochong")]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// get request
    Get(Get),
    /// post request
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    /// http request url
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    /// http request url
    url: String,
    /// http request body
    body: Vec<String>,
}

fn main() {
    let opt = Opts::parse();
    println!("Hello, world! {:?}", opt);
}
