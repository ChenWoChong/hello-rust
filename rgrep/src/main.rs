use anyhow::Result;
use clap::Parser;
use colored::*;
use glob::glob;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io;
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

    if args.paths.len() == 1 && args.paths[0] == "-" {
        let reader = BufReader::new(io::stdin());
        process_stream(reader, &args.pattern)?;
    } else {
        process_path(&args.paths, args.pattern)?;
    }
    Ok(())
}

fn process_path(path_list: &Vec<String>, pattern: String) -> Result<()> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for path_pattern in path_list {
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
    let all_res = paths
        .par_iter()
        .map(|path| process_file(path, &pattern))
        .collect::<Vec<_>>();

    for res_set in all_res {
        match res_set {
            Ok(res) => {
                for line in res {
                    print!("{}", line);
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn process_stream(reader: impl BufRead, pattern: &str) -> Result<()> {
    let re = Regex::new(pattern)?;
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            let res = highlight_matches(&line, &re)?;
            print!("{}:{}:\t{}", "StdIN", line_num + 1, res);
        }
    }
    Ok(())
}

fn process_file(path: &Path, pattern: &str) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(pattern)?;
    let mut res = Vec::new();
    res.push(format!("[{}]:\n\n", path.display()));

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some(_) = re.find(&line) {
            let highlighted = highlight_matches(line.as_str(), &re)?;
            res.push(format!(
                "{}:{}:\t{}\n",
                path.display(),
                line_num,
                highlighted
            ));
        }
    }
    Ok(res)
}

fn highlight_matches(line: &str, re: &Regex) -> Result<String> {
    let mut last_end = 0;
    let mut highlight_line = String::new();

    for mat in re.find_iter(line) {
        highlight_line.push_str(&line[last_end..mat.start()]);
        highlight_line.push_str(&mat.as_str().red().bold().to_string());
        last_end = mat.end();
    }
    highlight_line.push_str(&line[last_end..]);

    Ok(highlight_line)
}
