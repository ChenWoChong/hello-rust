use anyhow::Result;
use httpie::*;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opts {
        subcmd: SubCommand::Post(Post {
            url: "https://httpbin.org/post".to_string(),
            body: vec![
                KvPair {
                    k: "name".to_string(),
                    v: "Rust".to_string(),
                },
                KvPair {
                    k: "language".to_string(),
                    v: "Rust".to_string(),
                },
                KvPair {
                    k: "framework".to_string(),
                    v: "Actix".to_string(),
                },
            ],
        }),
    };
    run_request(opt).await?;
    Ok(())
}
