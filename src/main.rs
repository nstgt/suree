use clap::Parser;

mod cli;
mod display;
mod parser;
mod suree;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let options: suree::Options = args.into();
    if let Err(e) = suree::run(options).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
