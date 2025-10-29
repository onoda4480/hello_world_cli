#![allow(unused)]
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file)
        .with_context(|| format!("エラー内容 : {}が見つかりません", args.file.display()))?;
    println!("{}", content);
    Ok(())
}
