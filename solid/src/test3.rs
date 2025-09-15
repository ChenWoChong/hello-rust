use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

// --- 数据模型 ---
#[derive(Debug, Deserialize, Serialize)]
struct UserRecord {
    id: u32,
    name: String,
    email: String,
    // 从 CSV 读取时是字符串 "true"/"false"，希望能转成 bool
    is_active: bool,
    // 从 CSV 读取时是字符串，希望能转成 u64 时间戳
    last_login: u64,
}

// --- 违反 SOLID 的“万能”流水线 ---
pub struct DataPipeline;

impl DataPipeline {
    pub fn new() -> Self {
        Self
    }

    /// 罪状一：一个方法包揽了 E、T、L 所有职责 (严重违反 SRP)
    pub fn process(&self, input_path: &str, output_path: &str) -> Result<()> {
        // --- 环节一：Extract (抽取) ---
        // 罪状二：只能从本地文件系统读取，且焊死了 CSV 格式 (违反 DIP)
        println!("正在从 CSV 文件 '{}' 读取数据...", input_path);
        let raw_data = fs::read_to_string(input_path)?;
        let mut reader = csv::Reader::from_reader(raw_data.as_bytes());

        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: UserRecord = result?;
            records.push(record);
        }
        println!("成功读取 {} 条记录。", records.len());

        // --- 环节二：Transform (转换) ---
        // 罪状三：所有的转换步骤和规则都硬编码在此处 (严重违反 OCP)
        println!("开始进行数据转换...");
        let transformed_records: Vec<UserRecord> = records
            .into_iter()
            // 规则1：过滤掉不活跃的用户
            .filter(|r| r.is_active)
            // 规则2：将所有邮箱地址转为小写
            .map(|mut r| {
                r.email = r.email.to_lowercase();
                r
            })
            .collect();
        println!("转换后剩余 {} 条记录。", transformed_records.len());

        // --- 环节三：Load (加载) ---
        // 罪状四：只能写入到本地文件系统，且焊死了 JSON 格式 (违反 DIP)
        println!("正在将结果写入 JSON 文件 '{}'...", output_path);
        let json_output = serde_json::to_string_pretty(&transformed_records)?;
        fs::write(output_path, json_output)?;

        println!("处理完成！");
        Ok(())
    }
}

// --- 使用示例 ---
fn main() -> Result<()> {
    // 准备一个临时的 CSV 文件
    let csv_data = "id,name,email,is_active,last_login\n\
                    1,Alice,Alice@EXAMPLE.COM,true,1678886400\n\
                    2,Bob,bob@example.com,false,1678886401\n\
                    3,Charlie,CHARLIE@EXAMPLE.COM,true,1678886402";
    fs::write("users.csv", csv_data)?;

    let pipeline = DataPipeline::new();
    pipeline.process("users.csv", "active_users.json")?;

    Ok(())
}
