#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string(&args.file);
    let content =  match result{
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}: {}", args.file.display(), error);
            std::process::exit(1);
        }
    };
    println!("{}", content);
}
