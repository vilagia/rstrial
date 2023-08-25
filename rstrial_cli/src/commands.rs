use clap::{Parser, Subcommand};

use self::{check::CheckArgs, convert::ConvertArgs};

pub mod check;
pub mod convert;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

/// サブコマンドの定義
#[derive(Debug, Subcommand)]
pub enum Commands {
    Convert(ConvertArgs),
    Check(CheckArgs),
}

pub trait Command {
    type Args;
    fn execute(&self, args: &Self::Args) -> Result<(), Box<dyn std::error::Error>>;
}
