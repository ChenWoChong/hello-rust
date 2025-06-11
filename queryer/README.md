由于老师的代码依赖太老了，无法直接运行，调了一周总算调通，分享一下我的修改点


### 依赖版本
其他用最新版本没问题, 这两个必须注意，踩了好久的坑
1. polars 升级为 0.33.2，老版有 bug 会直接崩溃
2. sqlparser 必须维持在 0.10.0 ，新版本接口变化太大
```toml
[dependencies]
polars = { version = "0.33.2", features = ["json", "lazy"] }
sqlparser = "0.10.0"
```

### 网络代理
如果无法直接反问 GitHub，则需要走代理;  能直接访问不需要修改
```rust
// 不使用代理
let client = reqwest::Client::new();

// 使用代理
// let client = reqwest::Client::builder()
//     .proxy(reqwest::Proxy::all("http://127.0.0.1:7890")?)
//     .timeout(std::time::Duration::from_secs(30))
//     .build()?;
```


### examples/covid.rs 修改

```rust
use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    // 创建配置了代理的 client
    // let client = reqwest::Client::builder()
    //     .proxy(reqwest::Proxy::http("http://127.0.0.1:7890").expect("Failed to set HTTP proxy"))
    //     .proxy(reqwest::Proxy::https("http://127.0.0.1:7890").expect("Failed to set HTTPS proxy"))
    //     .build()
    //     .expect("Failed to build client");
    let client = reqwest::Client::new();

    println!("Sending request to {}", url);
    let data = client.get(url).send().await?.text().await?;

    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;
    let filtered = df.filter(&df.column("new_deaths")?.gt(500)?)?;
    println!(
        "{:?}",
        filtered.select([
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ])
    );
    Ok(())
}

```

### src/convert.rs
1. IsNull 和 IsNotNull 
```rust

SqlExpr::IsNull(expr) => Ok(Self::IsNull(Box::new(Expression(expr).try_into()?))),
SqlExpr::IsNotNull(expr) => Ok(Self::IsNotNull(Box::new(Expression(expr).try_into()?))),
SqlExpr::Identifier(id) => Ok(Self::Column(Arc::new(id.value))),

// 改为：

impl TryFrom<Expression> for Expr {
    type Error = anyhow::Error;
    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::BinaryOp { left, op, right } => Ok(Expr::BinaryExpr {
                left: Box::new(Expression(left).try_into()?),
                op: Operation(op).try_into()?,
                right: Box::new(Expression(right).try_into()?),
            }),
            SqlExpr::Wildcard => Ok(Self::Wildcard),
            SqlExpr::IsNull(expr) => Ok(Self::is_null(Expression(expr).try_into()?)),
            SqlExpr::IsNotNull(expr) => Ok(Self::is_not_null(Expression(expr).try_into()?)),
            SqlExpr::Identifier(id) => Ok(Self::Column(Arc::from(id.value))),
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expr {:#?} is not supported", v)),
        }
    }
}
```
2. 使用 arc::from
```rust
Box::new(Expr::Column(Arc::new(id.to_string()))),
Arc::new(alias.to_string()),

// 改为：

Box::new(Expr::Column(Arc::from(id.to_string()))),
Arc::from(alias.to_string()),
```

### src/lib.rs
```rust
    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX))
    }

// 改为：

    filtered = order_by.into_iter().fold(filtered, |acc, (col, desc)| {
        acc.sort(
            &col,
            SortOptions {
                descending: desc,
                nulls_last: true,
                multithreaded: false,
                maintain_order: false,
            },
        )
    });

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX) as IdxSize);
    }
```

### 示例运行方式
```bash
# 基本的 covid example(我改名为 covid_simple)
cargo run -p queryer --example covid_simple

# covid http example
cargo run -p queryer --example covid

# covid local example
cargo run -p queryer --example local
```