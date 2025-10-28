#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.file);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };
    println!("{}", content);
    Ok(())
}
