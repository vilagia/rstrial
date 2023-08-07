pub mod commands;

use commands::convert::ConvertArgs;

use log::{info, warn};

use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{convert::ConvertCommand, Command};

type PathManuscriptTuple = (String, String);

/// サブコマンドの定義

#[derive(Debug, Subcommand)]
enum Commands {
    Convert(ConvertArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Vfm,
    Aozora,
}

impl ValueEnum for OutputFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[OutputFormat::Vfm, OutputFormat::Aozora]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            OutputFormat::Vfm => Some(clap::builder::PossibleValue::new("vfm")),
            OutputFormat::Aozora => Some(clap::builder::PossibleValue::new("aozora")),
        }
    }
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    info!("{:?}", args);
    match args.command {
        Commands::Convert(args) => {
            warn!("start converting...");
            ConvertCommand.execute(&args).unwrap();
            warn!("finished converting!");
        }
    }
}
