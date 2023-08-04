use std::fs;

use clap::{Parser, Subcommand, ValueEnum};
use rstrial_converter::converter::{
    aozora::manuscript_converter::AozoraManuscriptConverter,
    vfm::manuscript_converter::VfmManuscriptConverter, ManuscriptConverter,
};

/// サブコマンドの定義

#[derive(Debug, Subcommand)]
enum Commands {
    Convert(ConvertArgs),
}

#[derive(Debug, clap::Args)]
struct ConvertArgs {
    /// Target file path
    target: std::path::PathBuf,

    /// Output format
    /// vfm: Vivliostyle Flavored Markdown
    /// aozora: Aozora Bunko format
    #[arg(short, long)]
    format: OutputFormat,

    /// Output file path
    /// If not specified, output to stdout
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    /// Output file extention
    /// If Output file path is not directory, this option is ignored
    /// default: txt
    #[arg(short, long)]
    ext: Option<String>,
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

    match args.command {
        Commands::Convert(args) => {
            let manuscripts: Vec<String> = extract_manuscripts(&args);
            let manuscripts = convert_manuscripts(&args, manuscripts);
            output(&args, manuscripts);
        }
    }
}

fn extract_manuscripts(args: &ConvertArgs) -> Vec<String> {
    match args.target.is_dir() {
        true => {
            let mut manuscripts = vec![];
            for entry in
                fs::read_dir(args.target.clone()).expect("Should have been able to read the dir")
            {
                let entry = entry.expect("Should have been able to read the entry");
                let path = entry.path();
                let content =
                    fs::read_to_string(path).expect("Should have been able to read the file");
                manuscripts.push(content);
            }
            manuscripts
        }
        false => {
            let content = fs::read_to_string(args.target.clone())
                .expect("Should have been able to read the file");
            vec![content]
        }
    }
}

fn convert_manuscripts(args: &ConvertArgs, manuscripts: Vec<String>) -> Vec<String> {
    manuscripts
        .iter()
        .map(|manuscript| {
            let parser = rstrial_parser::ManuscriptParser::new(manuscript);
            let tokens = parser.collect();

            match args.format {
                OutputFormat::Vfm => VfmManuscriptConverter::convert(tokens),
                OutputFormat::Aozora => AozoraManuscriptConverter::convert(tokens),
            }
        })
        .collect::<Vec<String>>()
}

fn output(args: &ConvertArgs, manuscripts: Vec<String>) {
    match &args.output {
        Some(path) => match path.is_dir() {
            true => {
                let ext = args.ext.clone().unwrap_or("txt".to_string());
                for (i, manuscript) in manuscripts.iter().enumerate() {
                    let file_name = format!("{}.{}", i, ext);
                    let path = path.join(file_name);
                    fs::write(path, manuscript).expect("Unable to write file");
                }
            }
            false => {
                fs::write(path, manuscripts.join("\n")).expect("Unable to write file");
            }
        },
        None => {
            for manuscript in manuscripts {
                println!("{}\n\n----\n\n", manuscript);
            }
        }
    }
}
