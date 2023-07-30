use std::fs;

use clap::{Parser, ValueEnum};
use rstrial_converter::converter::{
    aozora::line_converter::AozoraLineConverter, vfm::line_converter::VfmLineConverter,
    LineConverter,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target file path
    #[arg(short, long)]
    target: std::path::PathBuf,

    #[arg(short, long)]
    /// Output format
    /// vfm: Vivliostyle Flavored Markdown
    /// aozora: Aozora Bunko format
    format: OutputFormat,

    /// Output file path
    /// If not specified, output to stdout
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
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
    let args = Args::parse();
    if args.target.is_dir() {
        println!("{} is directory", args.target.display());
    } else {
        println!("{} is file", args.target.display());
        let contents =
            fs::read_to_string(args.target).expect("Should have been able to read the file");
        let parser = rstrial_parser::parser::section_parser::SectionParser::new(&contents);
        match args.format {
            OutputFormat::Vfm => {
                let text: String = parser.map(VfmLineConverter::convert).collect();
                println!("{}", text)
            }
            OutputFormat::Aozora => {
                let text: String = parser.map(AozoraLineConverter::convert).collect();
                println!("{}", text)
            }
        }
    }
}
