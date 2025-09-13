use crate::test1::domain::Report;
use crate::test1::persistence::{FsLoader, ReportLoader};
use crate::test1::presentation::{JsonExporter, PlainTextExporter, ReportExporter};
use anyhow::Result;

// 罪状一：一个“万能”的结构体，它知道的太多了！(违反 SRP)
mod domain {
    pub struct Report {
        pub title: String,
        pub content: String,
    }
}

mod persistence {
    use crate::test1::domain::Report;
    use std::fs;

    pub trait ReportLoader {
        fn load(&self) -> anyhow::Result<Report>;
    }

    pub struct FsLoader<'a> {
        path: &'a str,
    }

    impl<'a> FsLoader<'a> {
        pub fn new(path: &'a str) -> Self {
            Self { path }
        }
    }

    impl<'a> ReportLoader for FsLoader<'a> {
        fn load(&self) -> anyhow::Result<Report> {
            let content = fs::read_to_string(self.path)?;
            let (title, content) = content.split_once('\n').unwrap_or(("", &content));
            Ok(Report {
                title: title.to_string(),
                content: content.to_string(),
            })
        }
    }
}

mod presentation {
    use crate::test1::domain::Report;

    pub trait ReportExporter {
        fn export(&self, report: &Report) -> String;
    }

    #[derive(Default)]
    pub struct PlainTextExporter;
    impl ReportExporter for PlainTextExporter {
        fn export(&self, report: &Report) -> String {
            format!("---\nTitle: {}\n---\n{}", report.title, report.content)
        }
    }

    pub struct JsonExporter;
    impl ReportExporter for JsonExporter {
        fn export(&self, report: &Report) -> String {
            format!(
                "{{\n  \"title\": \"{}\",\n  \"content\": \"{}\"\n}}",
                report.title, report.content
            )
        }
    }
}

fn main() -> Result<()> {
    let loader = FsLoader::new("a.txt");
    // let report = loader.load()?;
    // 为方便直接运行，我们手动创建一个
    let report = Report {
        title: "年度总结报告".to_string(),
        content: "这是报告的正文部分...".to_string(),
    };

    println!(
        "--- Plain Text Report ---\n{}\n",
        PlainTextExporter.export(&report)
    );

    println!("--- JSON Report ---\n{}\n", JsonExporter.export(&report));

    Ok(())
}
