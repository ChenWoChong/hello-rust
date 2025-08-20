use anyhow::Result;
use clap::{Parser, arg};
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

mod error;
mod parse;

#[derive(Debug, Clone, Parser)]
#[command(version="0.0.1", about="A Rust Grep", long_about=None)]
struct CliArgs {
    /// 要搜索的模式 (可以是正则表达式)
    pattern: String,

    /// 要搜索的文件路径。使用 "-" 代表从标准输入 (stdin) 读取。
    path: String,
}

fn main() -> Result<()> {
    // parse cmd
    let args = CliArgs::parse();
    println!("pattern: {}, path: {}", args.pattern, args.path);

    // read content
    let reader: Box<dyn BufRead> = if args.path == "-" {
        Box::new(BufReader::new(stdin()))
    } else {
        let file = File::open(&args.path)?;
        Box::new(BufReader::new(file))
    };

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        highlight_matches(line_num, line.as_str(), args.pattern.as_str())?;
    }
    Ok(())
}

fn highlight_matches(line_num: usize, line: &str, pattern: &str) -> Result<()> {
    let re = Regex::new(pattern)?;

    if re.find(line).is_none() {
        return Ok(());
    }

    let mut last_end = 0;

    print!("{}", format!("line {} :\t", line_num).yellow());
    for mat in re.find_iter(line) {
        print!("{}", &line[last_end..mat.start()]);
        print!("{}", mat.as_str().red().bold());
        last_end = mat.end();
    }
    println!("{}", &line[last_end..]);

    Ok(())
}
