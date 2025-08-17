use anyhow::Result;
use bytes::{Buf, BytesMut};
use futures::{SinkExt, StreamExt};
use kv1::{CommandRequest, CommandResponse, Value}; // 引入你的 protobuf 类型
use prost::Message;
use std::io;
use tokio::net::TcpStream;
use tokio_util::codec::Framed; // 核心工具
use tokio_util::codec::{Decoder, Encoder}; // 确保你已引入 protobuf 生成的类型
use tracing::info;

pub struct ClientCodec {
    // 内部可以为空，因为它是一个无状态的编解码器
}

impl ClientCodec {
    pub fn new() -> Self {
        Self {}
    }
}

// 为客户端实现 Decoder，告诉 tokio-util 如何从字节流里“解码”出 CommandResponse
impl Decoder for ClientCodec {
    type Item = CommandResponse; // 客户端接收的是 Response
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // 这里的逻辑与服务端完全一致：先读4字节长度，再读消息体
        if src.len() < 4 {
            return Ok(None);
        }
        let len = src.get_u32() as usize;
        if src.len() < len {
            // 数据还不够长，等待下一次 read
            // 需要先将读过的长度信息“退回”，以免影响下次判断
            src.clear(); // 简单处理，实际可能需要更复杂的回退逻辑
            return Ok(None);
        }
        let data = src.split_to(len);
        CommandResponse::decode(&data[..])
            .map(Some)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

// 为客户端实现 Encoder, 告诉 tokio-util 如何将 CommandRequest “编码”成字节
impl Encoder<CommandRequest> for ClientCodec {
    type Error = io::Error; // 客户端发送的是 Request

    fn encode(&mut self, item: CommandRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // 逻辑也与服务端一致：先写4字节长度，再写消息体
        let encoded = item.encode_to_vec();
        dst.extend_from_slice(&(encoded.len() as u32).to_be_bytes());
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";

    // 1. 连接服务器，这部分不变
    let stream = TcpStream::connect(addr).await?;
    info!("Connected to {}", addr);

    // 2. 使用 Codec 包装原始 TcpStream，得到一个既是 Stream 又是 Sink 的 Framed 对象
    let mut client = Framed::new(stream, ClientCodec::new());

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
    client.send(cmd).await?;

    // 5. 等待并接收响应 (使用 StreamExt trait 提供的 .next() 方法)
    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:#?}", data);
    }

    Ok(())
}
