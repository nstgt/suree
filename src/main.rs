use clap::Parser;

mod cli;
mod display;
mod parser;
mod suree;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    let options: suree::Options = args.into();
    suree::run(options).await;
}
