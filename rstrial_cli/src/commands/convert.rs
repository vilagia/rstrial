use std::{fs, path::Path, fmt::{Display, Formatter}};

use clap::ValueEnum;
use common_path::common_path;
use log::{info, warn};
use rstrial_converter::converter::{
    aozora::manuscript_converter::AozoraManuscriptConverter,
    vfm::manuscript_converter::VfmManuscriptConverter, ManuscriptConverter,
};

use crate::PathManuscriptTuple;

use super::Command;

#[derive(Debug, clap::Args)]
pub struct ConvertArgs {
    /// Target file path
    target: std::path::PathBuf,

    /// target file extentions
    /// 
    /// default: txt
    #[arg(short, long)]
    ext: Option<Vec<AvailableExt>>,

    /// Output format
    /// vfm: Vivliostyle Flavored Markdown
    /// aozora: Aozora Bunko format
    #[arg(short, long)]
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

#[derive(Debug, Clone)]
enum AvailableExt {
    Txt,
    Md,
    Adoc,
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


impl ValueEnum for AvailableExt {
    fn value_variants<'a>() -> &'a [Self] {
        &[AvailableExt::Txt, AvailableExt::Md, AvailableExt::Adoc]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            AvailableExt::Txt => Some(clap::builder::PossibleValue::new("txt")),
            AvailableExt::Md => Some(clap::builder::PossibleValue::new("md")),
            AvailableExt::Adoc => Some(clap::builder::PossibleValue::new("adoc")),
        }
    }
}

impl Display for AvailableExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AvailableExt::Txt => write!(f, "txt"),
            AvailableExt::Md => write!(f, "md"),
            AvailableExt::Adoc => write!(f, "adoc"),
        }
    }
}

pub struct ConvertCommand;

impl Command for ConvertCommand {
    type Args = ConvertArgs;

    fn execute(&self, args: &Self::Args) -> Result<(), Box<dyn std::error::Error>> {
        let manuscripts = Self::extract_manuscripts(args);
        let manuscripts = Self::convert_manuscripts(args, manuscripts);
        Self::output(args, manuscripts);
        Ok(())
    }
}

impl ConvertCommand {
    fn extract_manuscripts(args: &ConvertArgs) -> Vec<PathManuscriptTuple> {
        match args.target.is_dir() {
            true => {
                let mut manuscripts = vec![];
                for entry in walkdir::WalkDir::new(args.target.clone()).into_iter() {
                    match entry {
                        Ok(entry) => {
                            if entry.file_type().is_dir() || entry.file_type().is_symlink() {
                                continue;
                            }

                            let path = entry.path();
                            let target_ext = args
                                .ext
                                .clone()
                                .unwrap_or(vec![AvailableExt::Txt])
                                .iter()
                                .map(|ext| ext.to_string())
                                .collect::<Vec<String>>();
                            let ext = path
                                .extension()
                                .expect("Should have been able to read the file");
                            if !target_ext.contains(&ext.to_str().unwrap().to_string()) {
                                continue;
                            }

                            let content = fs::read_to_string(path)
                                .expect("Should have been able to read the file");
                            manuscripts.push((path.to_string_lossy().to_string(), content));
                        }
                        Err(err) => {
                            warn!("Error: {:?}", err);
                            continue;
                        }
                    }
                }
                manuscripts
            }
            false => {
                let path = args.target.as_path();
                let content = fs::read_to_string(args.target.clone())
                    .expect("Should have been able to read the file");
                vec![(path.to_string_lossy().to_string(), content)]
            }
        }
    }

    fn convert_manuscripts(
        args: &ConvertArgs,
        manuscripts: Vec<PathManuscriptTuple>,
    ) -> Vec<PathManuscriptTuple> {
        let mut bar = progress::Bar::new();
        bar.set_job_title("Converting");
        let bar_tick = 100 / manuscripts.len() as u64;
        manuscripts
            .iter()
            .map(|(path, text)| {
                let path = Path::new(path);
                bar.add_percent(bar_tick as i32);
                let parser = rstrial_parser::ManuscriptParser::new(text);
                let tokens = parser.collect();

                let path = path.to_string_lossy().to_string();
                match args.format {
                    OutputFormat::Vfm => (path, VfmManuscriptConverter::convert(tokens)),
                    OutputFormat::Aozora => (path, AozoraManuscriptConverter::convert(tokens)),
                }
            })
            .collect::<Vec<PathManuscriptTuple>>()
    }

    fn output(args: &ConvertArgs, manuscripts: Vec<PathManuscriptTuple>) {
        match &args.output {
            Some(path) => match path.is_dir() {
                true => {
                    for (p, text) in manuscripts.iter() {
                        let output_path = path.canonicalize().unwrap();
                        let ext_path = Path::new(p).canonicalize().unwrap();
                        let common_prefix = common_path(&output_path, &ext_path)
                            .expect("Unable to get common path");
                        let relative_path = ext_path
                            .strip_prefix(common_prefix.to_str().unwrap())
                            .expect("Unable to get relative path");
                        let target_path = output_path.join(relative_path);
                        let target_dir_path = target_path.parent().unwrap();
                        info!(
                            "Saving: {} -> {}",
                            ext_path.display(),
                            target_path.display()
                        );
                        fs::create_dir_all(target_dir_path).expect("Unable to create directory");
                        fs::write(target_path, text).expect("Unable to write file");
                    }
                }
                false => {
                    let manuscripts = manuscripts
                        .iter()
                        .map(|(_, text)| text.to_owned())
                        .collect::<Vec<String>>();
                    fs::write(path, manuscripts.join("\n")).expect("Unable to write file");
                }
            },
            None => {
                for (_, text) in manuscripts {
                    println!("{}\n\n----\n\n", text);
                }
            }
        }
    }
}
