use anyhow::Result;
use queryer::query;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let http_url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let output = "owid-covid-latest.csv";

    let body = reqwest::get(http_url).await?.text().await?;

    fs::write(output, body)?;
    let url = format!("file://{}", output);

    // 使用 sql 从 URL 里获取数据
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC",
        url
    );
    let df1 = query(sql).await?;
    println!("{:?}", df1);
    fs::remove_file(output)?;
    
    Ok(())
}
