#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    text: String,
    file: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file).unwrap();
    println!("{}", content);
}
