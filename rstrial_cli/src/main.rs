pub mod commands;

use log::{info, warn};

use clap::Parser;

use crate::commands::{check::CheckCommand, convert::ConvertCommand, Args, Command, Commands};

type PathManuscriptTuple = (String, String);

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
        Commands::Check(args) => {
            warn!("start checking...");
            CheckCommand.execute(&args).unwrap();
            warn!("finished checking!");
        }
    }
}
