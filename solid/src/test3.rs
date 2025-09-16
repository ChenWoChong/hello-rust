use crate::test3::infrastructure::{CsvLoader, CsvSaver, RecordLoader, RecordSaver};
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

    pub trait DataSource: Send + Sync {
        fn load(
            &self,
        ) -> anyhow::Result<Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send>>;
    }

    pub trait Transform: Send + Sync {
        fn transform<'a>(
            &'a self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>,
        ) -> Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>;
    }

    pub trait DataSink: Send + Sync {
        fn save<'a>(
            &'a self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>,
        ) -> anyhow::Result<()>;
    }
}

mod implementation {
    use crate::test3::UserRecord;
    use crate::test3::contracts::{DataSink, DataSource, Transform};
    use anyhow::anyhow;
    use std::fs;

    pub struct CsvDataSource<'a> {
        path: &'a str,
    }
    impl<'a> CsvDataSource<'a> {
        pub fn new(path: &'a str) -> Self {
            Self { path }
        }
    }

    impl<'a> DataSource for CsvDataSource<'a> {
        fn load(
            &self,
        ) -> anyhow::Result<Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send>> {
            println!("正在从 CSV 文件 '{}' 读取数据...", self.path);
            let file = fs::File::open(self.path)?;
            let reader = csv::Reader::from_reader(file);

            // csv::Reader::deserialize 本身就返回一个迭代器
            let iter = reader.into_deserialize().map(|res| {
                // 将 csv::Error 转换为我们统一的 anyhow::Error
                res.map_err(|e| anyhow!(e))
            });

            // 将迭代器装箱返回
            Ok(Box::new(iter))
        }
    }

    pub struct JsonFileSink<'a> {
        path: &'a str,
    }

    impl<'a> JsonFileSink<'a> {
        pub fn new(path: &'a str) -> Self {
            Self { path }
        }
    }

    impl<'a> DataSink for JsonFileSink<'a> {
        fn save<'b>(
            &'b self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'b>,
        ) -> anyhow::Result<()> {
            println!("正在将结果写入 JSON 文件 '{}'...", self.path);
            let records: Vec<UserRecord> = stream.collect::<anyhow::Result<Vec<_>>>()?;
            let json_output = serde_json::to_string_pretty(&records)?;
            fs::write(self.path, json_output)?;
            Ok(())
        }
    }

    pub struct ActiveUserFilter;
    impl Transform for ActiveUserFilter {
        fn transform<'a>(
            &'a self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>,
        ) -> Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a> {
            println!("应用“活跃用户”过滤器...");
            let transformed_stream = stream.filter(|res| match res {
                Ok(record) => record.is_active,
                Err(_) => true,
            });
            Box::new(transformed_stream)
        }
    }
    pub struct EmailLowercaseTransform;
    impl Transform for EmailLowercaseTransform {
        fn transform<'a>(
            &'a self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>,
        ) -> Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a> {
            println!("应用“邮箱小写”转换...");
            let transformed_stream = stream.map(|res| {
                res.map(|mut record| {
                    record.email = record.email.to_lowercase();
                    record
                })
            });
            Box::new(transformed_stream)
        }
    }

    pub struct NameAnonymizeTransform;
    impl NameAnonymizeTransform {
        fn anonymize(name: &str) -> String {
            if name.is_empty() {
                return "".to_string();
            }
            let first_char = name.chars().next().unwrap();
            format!("{}***", first_char)
        }
    }

    impl Transform for NameAnonymizeTransform {
        fn transform<'a>(
            &'a self,
            stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a>,
        ) -> Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send + 'a> {
            println!("应用“姓名匿名化”转换...");
            let transformed_stream = stream.map(|res| {
                res.map(|mut record| {
                    record.name = Self::anonymize(&record.name);
                    record
                })
            });
            Box::new(transformed_stream)
        }
    }
}

mod pipeline {
    use crate::test3::UserRecord;
    use crate::test3::contracts::{DataSink, DataSource, Transform};
    use std::iter::FusedIterator;
    use std::marker::PhantomData;

    pub struct PipelineBuilder<'a> {
        source: Option<&'a dyn DataSource>,
        transforms: Vec<&'a (dyn Transform + 'a)>,
        sink: Option<&'a dyn DataSink>,
        _maker: PhantomData<&'a ()>,
    }

    impl<'a> PipelineBuilder<'a> {
        pub fn new(source: &'a dyn DataSource) -> Self {
            Self {
                source: Some(source),
                transforms: Vec::new(),
                sink: None,
                _maker: PhantomData,
            }
        }

        pub fn add_transform(mut self, transform: &'a dyn Transform) -> Self {
            self.transforms.push(transform);
            self
        }

        pub fn with_sink(mut self, data_sink: &'a dyn DataSink) -> Self {
            self.sink = Some(data_sink);
            self
        }

        pub fn run(self) -> anyhow::Result<()> {
            let mut stream: Box<dyn Iterator<Item = anyhow::Result<UserRecord>> + Send> =
                self.source.unwrap().load()?;

            for transform in self.transforms {
                stream = transform.transform(stream);
            }

            let sink = self.sink.unwrap();
            sink.save(stream)?;
            println!("流水线处理完成！");
            Ok(())
        }
    }
}

mod infrastructure {
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

#[cfg(test)]
mod tests {
    use crate::test3::implementation::{
        ActiveUserFilter, CsvDataSource, EmailLowercaseTransform, JsonFileSink,
        NameAnonymizeTransform,
    };
    use crate::test3::pipeline;
    use std::fs;

    #[test]
    fn pipeline_builder_should_work() -> anyhow::Result<()> {
        // 准备一个临时的 CSV 文件
        let csv_data = "id,name,email,is_active,last_login\n\
                    1,Alice,Alice@EXAMPLE.COM,true,1678886400\n\
                    2,Bob,bob@example.com,false,1678886401\n\
                    3,Charlie,CHARLIE@EXAMPLE.COM,true,1678886402";
        fs::write("users.csv", csv_data)?;

        let csv_source = CsvDataSource::new("users.csv");
        let json_sink = JsonFileSink::new("active_users.json");

        let active_filter = ActiveUserFilter;
        let email_lowercase = EmailLowercaseTransform;
        let name_anonymizer = NameAnonymizeTransform;

        println!("--- 流水线 1: 过滤 + 邮箱小写 ---");
        pipeline::PipelineBuilder::new(&csv_source)
            .add_transform(&active_filter)
            .add_transform(&name_anonymizer)
            .with_sink(&json_sink)
            .run()?;

        println!("\n--- 流水线 2: 过滤 + 姓名匿名化 ---");
        pipeline::PipelineBuilder::new(&csv_source)
            .add_transform(&active_filter)
            .add_transform(&name_anonymizer)
            .with_sink(&json_sink)
            .run()?;

        Ok(())
    }
}
