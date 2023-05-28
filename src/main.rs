use clap::{Parser, Subcommand};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Result;
use std::process::ExitCode;

mod stklang;
mod runtime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// runs a program
    Run {
        path: String
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Run { path } => {
            command_run(&path)
        }
    };

    match result {
        Ok(_) => {
            ExitCode::SUCCESS
        }
        Err(err) => {
            println!("Error occurred: {}", &err);
            ExitCode::FAILURE
        }
    }
}

fn command_run(path: &String) -> Result<()> {
    let program_file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let reader = BufReader::new(program_file);
    let program: stklang::inst::Program = serde_json::from_reader(reader)?;

    let mut runner = runtime::runner::init()?;
    stklang::runner::run(&mut runner, &program)?;

    Result::Ok(())
}
