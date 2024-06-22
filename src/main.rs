use clap::{Parser, Subcommand};
use std::io::Error;
use std::path::PathBuf;

mod build;
mod compile;
mod generate;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Generate {
        #[arg(short, long, default_value = "mandolin.json")]
        input: PathBuf,

        #[arg(short, long, default_value = "output/lilypond/mandocard.ly")]
        output: PathBuf,
    },
    Compile {
        #[arg(short, long, default_value = "output/lilypond/mandocard.ly")]
        input: PathBuf,

        #[arg(short, long, default_value = "output/anki")]
        output: PathBuf,
    },
    Build {
        #[arg(short, long, default_value = "output/anki")]
        input: PathBuf,

        #[arg(short, long, default_value = "output/anki/mandocard.apkg")]
        output: PathBuf,
    },
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Generate { input, output } => generate::score(&input, &output),
        SubCommand::Compile { input, output } => compile::images(&input, &output),
        SubCommand::Build { input, output } => build::deck(&input, &output),
    }
}
