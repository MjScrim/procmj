use clap::Parser;
use procmj::proc::{
    cli::CliArgs, 
    process::fetch_processes, 
    output::handle_output
};

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let processes = fetch_processes(&args.filter).await;
    handle_output(&processes, &args).await;
}
