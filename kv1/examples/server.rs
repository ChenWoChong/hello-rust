use anyhow::Result;
use bytes::{Buf, BytesMut};
use futures::{SinkExt, StreamExt};
use prost::Message; // 来自 prost 库
use std::io;
use tokio::net::TcpListener;
use tokio_util::codec::{Decoder, Encoder, Framed}; // 核心工具
use tracing::info;

// 假设这是你的 protobuf 结构体
use kv1::{CommandRequest, CommandResponse, MemTable, Service};

// --- 步骤 1: 创建你自己的编解码器 ---
pub struct ProstCodec<In, Out> {
    // ... 内部状态 ...
    _in: std::marker::PhantomData<In>,
    _out: std::marker::PhantomData<Out>,
}

impl<In, Out> ProstCodec<In, Out> {
    pub fn new() -> Self {
        Self {
            _in: std::marker::PhantomData,
            _out: std::marker::PhantomData,
        }
    }
}

// 实现 Decoder, 告诉 tokio-util 如何从字节流里“解码”出你的 CommandRequest
impl Decoder for ProstCodec<CommandRequest, CommandResponse> {
    type Item = CommandRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // 这里的逻辑是：先读 4 字节的长度头，再根据长度读消息体
        if src.len() < 4 {
            return Ok(None);
        }
        let len = src.get_u32() as usize;
        if src.len() < len {
            // 数据还不够，等下次
            src.clear(); // 清理错误的数据或者更复杂的回退逻辑
            return Ok(None);
        }
        let data = src.split_to(len);
        CommandRequest::decode(&data[..])
            .map(Some)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

// 实现 Encoder, 告诉 tokio-util 如何将你的 CommandResponse “编码”成字节
impl Encoder<CommandResponse> for ProstCodec<CommandRequest, CommandResponse> {
    type Error = io::Error;

    fn encode(&mut self, item: CommandResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // 这里的逻辑是：先写 4 字节的长度头，再写消息体
        let encoded = item.encode_to_vec();
        dst.extend_from_slice(&(encoded.len() as u32).to_be_bytes());
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service = Service::new(MemTable::new());

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);

        let svc = service.clone();

        tokio::spawn(async move {
            // --- 步骤 2: 将 stream 和 codec 包装成 Framed 对象 ---
            let mut framed =
                Framed::new(stream, ProstCodec::<CommandRequest, CommandResponse>::new());

            // --- 步骤 3: 现在 framed 就是一个标准的 Stream + Sink, 可以直接使用 .next() 和 .send() ---
            while let Some(Ok(cmd)) = framed.next().await {
                info!("Got a command: {:?}", cmd);

                let resp = svc.execute(cmd);

                if let Err(e) = framed.send(resp).await {
                    info!("Failed to send response: {:?}", e);
                }
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
