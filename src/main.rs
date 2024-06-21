//use genanki_rs::{Note, Error};
use clap::{Parser, Subcommand};
use std::io::Error;
use std::path::PathBuf;

mod build;
mod compile;
mod generate;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "target")]
    directory: PathBuf,

    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Generate {
        #[arg(short, long, default_value = "mandolin.json")]
        input: PathBuf,

        #[arg(short, long, default_value = "-")]
        output: PathBuf,
    },
    Compile {},
    Build {},
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.cmd {
        SubCommand::Generate { input, output } => match generate::score(&input, &output) {
            Ok(result) => Ok(result),
            Err(e) => panic!("Generation error: {e:?}"),
        },
        SubCommand::Compile {} => match compile::images() {
            Ok(result) => Ok(result),
            Err(e) => panic!("Compilation error: {e:?}"),
        },
        SubCommand::Build {} => match build::deck() {
            Ok(result) => Ok(result),
            Err(e) => panic!("Build error: {e:?}"),
        },
    }
}
