use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use colored::Colorize;
use mime::{APPLICATION_JSON, Mime};
use reqwest::{Client, Response, Url, header};
use std::{collections::HashMap, str::FromStr};

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser, Debug)]
#[command(version = "1.0.0", author = "ChenWoChong")]
#[command(styles = CLAP_STYLING)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Get request
    Get(Get),
    /// Post request
    Post(Post),
}

#[derive(Parser, Debug)]
pub struct Get {
    /// http request url
    #[arg(value_parser = parse_url)]
    pub url: String,
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
pub struct Post {
    /// http request url
    #[arg(value_parser=parse_url)]
    pub url: String,
    /// http request body
    #[arg(value_parser=parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s.red()));
        Ok(Self {
            k: split.next().ok_or_else(err)?.to_string(),
            v: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status)
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        _ => {
            println!("{}", body)
        }
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    print_body(get_content_type(&resp), &resp.text().await?);
    Ok(())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

pub async fn run_request(opt: Opts) -> Result<()> {
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    let result = match opt.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("https://www.baidu.com").is_ok());
        assert!(parse_url("https://httpie.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=b").unwrap(),
            KvPair {
                k: "a".to_string(),
                v: "b".to_string(),
            }
        );
        assert_eq!(
            parse_kv_pair("a=").unwrap(),
            KvPair {
                k: "a".to_string(),
                v: "".to_string(),
            }
        )
    }
}
