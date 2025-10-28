#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file)
        .map_err(|err| CustomError(format!("エラー内容: `{}`: {}", args.file.display(), err)))?;

    println!("{}", content);
    Ok(())
}
