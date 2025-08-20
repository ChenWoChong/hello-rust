use anyhow::Result;
use clap::Parser;
use colored::*;
use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

mod error;
mod parse;

#[derive(Debug, Clone, Parser)]
#[command(version="0.0.1", about="A Rust Grep", long_about=None)]
struct CliArgs {
    /// 要搜索的模式 (可以是正则表达式)
    pattern: String,

    /// 要搜索的文件路径。使用 "-" 代表从标准输入 (stdin) 读取。
    #[arg(required = true)]
    paths: Vec<String>,
}

fn main() -> Result<()> {
    // parse cmd
    let args = CliArgs::parse();
    println!("pattern: {}, path: {:?}", args.pattern, args.paths);
    let mut paths: Vec<PathBuf> = Vec::new();
    for path_pattern in &args.paths {
        match glob(path_pattern) {
            Ok(path_iter) => {
                // 将 glob 返回的迭代器中的有效路径，添加到总列表中
                for entry in path_iter {
                    if let Ok(path) = entry {
                        paths.push(path);
                    } else if let Err(e) = entry {
                        eprintln!("错误: 处理通配符条目时出错: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("错误: 无效的通配符模式 '{}': {}", path_pattern, e);
            }
        }
    }

    paths.par_iter().for_each(|path| {
        if let Err(e) = process_file(path, &args.pattern) {
            eprintln!("Error processing file {}: {}", path.display(), e);
        }
    });

    Ok(())
}

fn process_file(path: &Path, pattern: &str) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(pattern)?;

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some(_) = re.find(&line) {
            highlight_matches(path, line_num, line.as_str(), &re)?;
        }
    }
    Ok(())
}

fn highlight_matches(path: &Path, line_num: usize, line: &str, re: &Regex) -> Result<()> {
    print!("{}:{}:\t", path.display(), line_num);
    let mut last_end = 0;

    for mat in re.find_iter(line) {
        print!("{}", &line[last_end..mat.start()]);
        print!("{}", mat.as_str().red().bold());
        last_end = mat.end();
    }
    println!("{}", &line[last_end..]);

    Ok(())
}
