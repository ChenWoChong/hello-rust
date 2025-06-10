use anyhow::Result;
use clap::Parser;
use std::process::exit;

use httpie::*;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opts::parse();
    println!("opt: {:#?}", opt);

    match run_request(opt).await {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("err: {}", e);
            exit(1);
        }
    }
}
