use clap::{Parser, Subcommand};

#[derive(Parser)]
  #[clap(
      name = "hello_world_cli",
      author = "Author's name",
      version = "v1.0.0",
      about = "Application short description."
  )]

struct Cli {
    #[clap(subcommand)]
    command: Commands,
    file: std::path::PathBuf
}

#[derive(Subcommand)]
enum Commands {
      Name { proguraming_language: String }
  }

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file).expect("could not read file");

    match &args.command {
        Commands::Name { proguraming_language } => {
            println!("Hello, {}!", proguraming_language);
        }
    }
}
