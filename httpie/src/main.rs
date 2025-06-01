use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use reqwest::Url;
use std::str::FromStr;

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
    #[arg(value_parser = parse_url)]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    // match s.parse::<Url>() {
    //     Err(e) => Err(format!("Err parsing URL: {}", e)),
    //     Ok(_) => Ok(s.into()),
    // }
    let _ = s.parse::<Url>()?;
    Ok(s.into())
}

#[derive(Parser, Debug)]
struct Post {
    /// http request url
    #[arg(value_parser=parse_url)]
    url: String,
    /// http request body
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opt = Opts::parse();
    println!("Hello, world! {:?}", opt);
}
