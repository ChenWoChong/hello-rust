use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use prost::Message; // 来自 prost 库
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec}; // 核心工具
use tracing::info;

// 假设这是你的 protobuf 结构体
use kv1::{CommandRequest, MemTable, Service, ServiceInner};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service = ServiceInner::new(MemTable::new()).into();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            // --- 步骤 2: 将 stream 和 codec 包装成 Framed 对象 ---
            let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

            // --- 步骤 3: 现在 framed 就是一个标准的 Stream + Sink, 可以直接使用 .next() 和 .send() ---
            while let Some(Ok(data)) = framed.next().await {
                let cmd = CommandRequest::decode(data).unwrap();
                info!("Got a command: {:?}", cmd);

                let resp = svc.execute(cmd);

                if let Err(e) = framed.send(Bytes::from(resp.encode_to_vec())).await {
                    info!("Failed to send response: {:?}", e);
                }
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
