use anyhow::Result;
use std::fs;

pub trait Loader {
    fn load(&self) -> Result<String>;
}

pub struct FsLoader<'a> {
    path: &'a str,
}
impl<'a> Loader for FsLoader<'a> {
    fn load(&self) -> Result<String> {
        match fs::read_to_string(self.path) {
            Ok(content) => Ok(content),
            Err(e) => Err(e.into()),
        }
    }
}

// 罪状一：一个“万能”的结构体，它知道的太多了！(违反 SRP)
pub struct Report {
    pub title: String,
    pub content: String,
}

impl Report {
    /// 从文件加载报告。
    /// 罪状二：高层逻辑直接依赖了底层的文件系统实现。(违反 DIP)
    pub fn load(loader: &dyn Loader) -> Result<Self> {
        // 假设文件的第一行是标题
        let content = loader.load()?;
        let (title, content) = content.split_once('\n').unwrap_or(("", &content));
        Ok(Self {
            title: title.to_string(),
            content: content.to_string(),
        })
    }
}
pub struct Export {
    report: Report,
}

pub trait Exporter {
    fn export(&self, report: &Report) -> String;
}

#[derive(Default)]
pub struct PlainTextExporter();

impl Exporter for PlainTextExporter {
    fn export(&self, report: &Report) -> String {
        format!("---\nTitle: {}\n---\n{}", report.title, report.content)
    }
}

#[derive(Default)]
pub struct JsonExporter();

impl Exporter for JsonExporter {
    fn export(&self, report: &Report) -> String {
        format!(
            "{{\n  \"title\": \"{}\",\n  \"content\": \"{}\"\n}}",
            report.title, report.content
        )
    }
}

impl Export {
    pub fn new(report: Report) -> Self {
        Self { report }
    }
    /// 将报告导出为指定格式的字符串。
    /// 罪状三：这个方法承担了所有格式的转换逻辑。(违反 SRP 和 OCP)
    pub fn export(&self, exporter: &dyn Exporter) -> String {
        exporter.export(&self.report)
    }
}

// --- 使用示例 ---
fn main() -> Result<()> {
    // 假设我们有一个 a.txt 文件，内容为:
    // 年度总结报告
    // 这是报告的正文部分...

    // let report = Report::load("a.txt")?;
    // 为方便直接运行，我们手动创建一个
    let report = Report {
        title: "年度总结报告".to_string(),
        content: "这是报告的正文部分...".to_string(),
    };

    let exporter = Export::new(report);

    let plain_exporter = PlainTextExporter::default();
    println!(
        "--- Plain Text Report ---\n{}\n",
        exporter.export(&plain_exporter)
    );

    let json_exporter = JsonExporter::default();
    println!("--- JSON Report ---\n{}\n", exporter.export(&json_exporter));

    Ok(())
}
