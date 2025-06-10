use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    // 创建配置了代理的 client
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("http://127.0.0.1:7890").expect("Failed to set HTTP proxy"))
        .proxy(reqwest::Proxy::https("http://127.0.0.1:7890").expect("Failed to set HTTPS proxy"))
        .build()
        .expect("Failed to build client");

    println!("Sending request to {}", url);
    let data = client.get(url).send().await?.text().await?;

    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;
    // let filtered = df.filter(&df.column("new_deaths")?.gt(500)?)?;
    // println!(
    //     "{:?}",
    //     filtered.select([
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ])
    // );
    Ok(())
}
