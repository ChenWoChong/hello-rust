use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use kv1::{CommandRequest, CommandResponse, Value}; // 引入你的 protobuf 类型
use prost::Message;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec}; // 核心工具
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    // 1. 连接服务器，这部分不变
    let stream = TcpStream::connect(addr).await?;
    info!("Connected to {}", addr);

    // 2. 使用 Codec 包装原始 TcpStream，得到一个既是 Stream 又是 Sink 的 Framed 对象
    let mut client = Framed::new(stream, LengthDelimitedCodec::new());

    // 3. 创建一个命令
    let cmd = CommandRequest::new_hset(
        "table1",
        "hello",
        Value {
            value: Some(kv1::value::Value::String("world".into())),
        },
    );

    // 4. 发送命令 (使用 SinkExt trait 提供的 .send() 方法)
    info!("Sending command: {:?}", cmd);
    client.send(Bytes::from(cmd.encode_to_vec())).await?;

    // 5. 等待并接收响应 (使用 StreamExt trait 提供的 .next() 方法)
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", data);
    }

    let cmd = CommandRequest::new_hset(
        "table1",
        "hello",
        Value {
            value: Some(kv1::value::Value::String("world_new".into())),
        },
    );
    info!("Sending command: {:?}", cmd);
    client.send(Bytes::from(cmd.encode_to_vec())).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", CommandResponse::decode(data)?);
    }

    let cmd = CommandRequest::new_hset(
        "table1",
        "hello1",
        Value {
            value: Some(kv1::value::Value::String("world1".into())),
        },
    );
    info!("Sending command: {:?}", cmd);
    client.send(Bytes::from(cmd.encode_to_vec())).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", CommandResponse::decode(data)?);
    }

    let cmd = CommandRequest::new_hset(
        "table1",
        "hello2",
        Value {
            value: Some(kv1::value::Value::String("world2".into())),
        },
    );
    info!("Sending command: {:?}", cmd);
    client.send(Bytes::from(cmd.encode_to_vec())).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", CommandResponse::decode(data)?);
    }

    // h_m_get
    let cmd = CommandRequest::new_hmget("table1", ["hello", "hello1", "hello2", "hello3"]);
    info!("Sending command: {:?}", cmd);
    client.send(Bytes::from(cmd.encode_to_vec())).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", CommandResponse::decode(data)?);
    }

    Ok(())
}
