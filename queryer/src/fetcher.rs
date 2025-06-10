use anyhow::{Result, anyhow};
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => UrlFetcher(name).fetch().await,
        "file" => FileFetcher(name).fetch().await,
        _ => Err(anyhow!(
            "We only support http/https/file at this current moment"
        )),
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        tracing::info!("Fetching data from URL: {}", self.0);

        let client = reqwest::Client::new();

        // 使用代理
        // let client = reqwest::Client::builder()
        //     .proxy(reqwest::Proxy::all("http://127.0.0.1:7890")?)
        //     .timeout(std::time::Duration::from_secs(30))
        //     .build()?;

        let response = client.get(self.0).send().await?;
        tracing::info!("Response status: {}", response.status());

        let res = response.text().await?;
        Ok(res)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}
