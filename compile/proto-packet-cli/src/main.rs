use clap::{Parser, Subcommand};
use clerr::Report;
use std::process::exit;
mod args;
mod io;
mod sub;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        file: String,
    },
    Validate {
        file: String,
    },
    Compile {
        lang: String,
        source: String,
        target: String,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    if let Err(report) = run(cli) {
        eprintln!("{}", report);
        exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Report> {
    match cli.command {
        Commands::Parse { file } => sub::parse(file),
        Commands::Validate { file } => sub::validate(file),
        Commands::Compile {
            lang,
            source,
            target,
        } => sub::compile(lang, source, target),
    }
}
