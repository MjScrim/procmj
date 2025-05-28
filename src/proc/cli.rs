use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {

    #[arg(short, long)]
    pub filter: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,

}
