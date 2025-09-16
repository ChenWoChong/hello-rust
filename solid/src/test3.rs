use crate::test3::infrastructrue::{CsvLoader, CsvSaver, RecordLoader, RecordSaver};
use crate::test3::transfer::{RecordFilter, RecordLowercase, RecordTransfer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

// --- 数据模型 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserRecord {
    id: u32,
    name: String,
    email: String,
    // 从 CSV 读取时是字符串 "true"/"false"，希望能转成 bool
    is_active: bool,
    // 从 CSV 读取时是字符串，希望能转成 u64 时间戳
    last_login: u64,
}

mod contracts {
    use crate::test3::UserRecord;

    pub trait DataSource {
        fn load(&self) -> anyhow::Result<Box<dyn Iterator<Item = anyhow::Result<UserRecord>>>>;
    }

    pub trait Transform: Send + Sync {
        fn transform(
            &self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>>>,
        ) -> Box<dyn Iterator<Item = anyhow::Result<UserRecord>>>;
    }

    pub trait DataSink {
        fn save(
            &self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>>>,
        ) -> anyhow::Result<()>;
    }
}

mod implementation {
    use std::fs;
    use crate::test3::UserRecord;
    use crate::test3::contracts::DataSource;
    use csv::ErrorKind::Seek;

    pub struct CsvDataSource<'a> {
        path: &'a str,
    }
    impl<'a> CsvDataSource<'a> {
        pub fn new(path: &'a str) -> Self {
            Self { path }
        }
    }

    impl DataSource for CsvDataSource {
        fn load(&self) -> anyhow::Result<Box<dyn Iterator<Item = anyhow::Result<UserRecord>>>> {
            println!("正在从 CSV 文件 '{}' 读取数据...", self.path);
            let raw_data = fs::read_to_string(self.path)?;
            let mut reader = csv::Reader::from_reader(raw_data.as_bytes());

            reader.deserialize().into_iter().map(|result| {
                let record: UserRecord = result.into();
                record
            }).
        }
    }

    pub struct JsonFileSink<'a> {
        path: &'a str,
    }

    pub struct ActiveUserFilter;
    pub struct EmailLowercaseTransform;
}

mod infrastructrue {
    use crate::test3::UserRecord;
    use std::fs;

    pub trait RecordLoader {
        fn load(&self, path: &str) -> anyhow::Result<Vec<UserRecord>>;
    }

    pub struct CsvLoader;
    impl RecordLoader for CsvLoader {
        fn load(&self, path: &str) -> anyhow::Result<Vec<UserRecord>> {
            println!("正在从 CSV 文件 '{}' 读取数据...", path);
            let raw_data = fs::read_to_string(path)?;
            let mut reader = csv::Reader::from_reader(raw_data.as_bytes());

            let mut records = Vec::new();
            for result in reader.deserialize() {
                let record: UserRecord = result?;
                records.push(record);
            }
            println!("成功读取 {} 条记录。", records.len());

            Ok(records)
        }
    }

    pub struct DBLoader;
    impl RecordLoader for DBLoader {
        fn load(&self, path: &str) -> anyhow::Result<Vec<UserRecord>> {
            todo!()
        }
    }

    pub trait RecordSaver {
        fn save(&self, path: &str, records: Vec<UserRecord>) -> anyhow::Result<()>;
    }
    pub struct CsvSaver;
    impl RecordSaver for CsvSaver {
        fn save(&self, path: &str, records: Vec<UserRecord>) -> anyhow::Result<()> {
            let json_output = serde_json::to_string_pretty(&records)?;
            fs::write(path, json_output)?;
            Ok(())
        }
    }
}

mod transfer {
    use crate::test3::UserRecord;

    pub trait RecordTransfer {
        fn transfer(&self, records: Vec<UserRecord>) -> Vec<UserRecord>;
    }

    pub struct RecordFilter;
    impl RecordTransfer for RecordFilter {
        fn transfer(&self, records: Vec<UserRecord>) -> Vec<UserRecord> {
            records.into_iter().filter(|r| r.is_active).collect()
        }
    }

    pub struct RecordLowercase;
    impl RecordTransfer for RecordLowercase {
        fn transfer(&self, records: Vec<UserRecord>) -> Vec<UserRecord> {
            records
                .into_iter()
                .map(|mut r| {
                    r.email = r.email.to_lowercase();
                    r
                })
                .collect()
        }
    }
}

// --- 违反 SOLID 的“万能”流水线 ---
pub struct DataPipeline<'a> {
    loader: &'a dyn RecordLoader,
    saver: &'a dyn RecordSaver,
    transfers: &'a [&'a dyn RecordTransfer],
}

impl<'a> DataPipeline<'a> {
    pub fn new(
        loader: &'a dyn RecordLoader,
        saver: &'a dyn RecordSaver,
        transfers: &'a [&'a dyn RecordTransfer],
    ) -> Self {
        Self {
            loader,
            saver,
            transfers,
        }
    }

    pub fn load(&self, input_path: &str) -> Result<Vec<UserRecord>> {
        println!("start to load from {}", input_path);
        self.loader.load(input_path)
    }

    pub fn transfer(&self, records: Vec<UserRecord>) -> Vec<UserRecord> {
        let mut res = records;
        for &trans in self.transfers {
            res = trans.transfer(res);
        }
        res
    }

    /// 罪状一：一个方法包揽了 E、T、L 所有职责 (严重违反 SRP)
    pub fn process(&self, input_path: &str, output_path: &str) -> Result<()> {
        // --- 环节一：Extract (抽取) ---
        // 罪状二：只能从本地文件系统读取，且焊死了 CSV 格式 (违反 DIP)
        let records = self.load(input_path)?;

        // --- 环节二：Transform (转换) ---
        // 罪状三：所有的转换步骤和规则都硬编码在此处 (严重违反 OCP)
        println!("开始进行数据转换...");
        let transformed_records: Vec<UserRecord> = self.transfer(records);
        println!("转换后剩余 {} 条记录。", transformed_records.len());

        // --- 环节三：Load (加载) ---
        // 罪状四：只能写入到本地文件系统，且焊死了 JSON 格式 (违反 DIP)
        println!("正在将结果写入 JSON 文件 '{}'...", output_path);
        self.saver.save(output_path, transformed_records)?;
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

    let csv_loader = CsvLoader;
    let csv_saver = CsvSaver;
    let transfer_list: Vec<&dyn RecordTransfer> = vec![&RecordFilter, &RecordLowercase];
    let pipeline = DataPipeline::new(&csv_loader, &csv_saver, &transfer_list);

    pipeline.process("users.csv", "active_users.json")?;
    Ok(())
}
