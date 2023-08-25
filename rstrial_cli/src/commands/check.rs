use std::{path::Path, vec};

use rstrial_converter::converter::{vfm::line_converter::VfmLineConverter, LineConverter};
use rstrial_parser::{
    tokens::{
        section::{Manuscript, Section},
        LineItem,
    },
    ManuscriptParser,
};

use super::Command;

#[derive(Debug, clap::Args)]
pub struct CheckArgs {
    /// Target file path
    target: std::path::PathBuf,

    /// If true, use LLM for manuscript checking
    #[arg(short, long)]
    use_llm: bool,
}

pub struct CheckCommand;
impl Command for CheckCommand {
    type Args = CheckArgs;
    fn execute(&self, args: &Self::Args) -> Result<(), Box<dyn std::error::Error>> {
        let tokens = Self::tokenize(args.target.as_path())?;
        let sections: Vec<(Vec<String>, String)> = tokens
            .into_iter()
            .filter_map(|t| match t {
                Section::Title(_title) => None,
                Section::Scene(doc, body) => {
                    let tags = doc.tags;
                    let body: String = body.into_iter().map(VfmLineConverter::convert).collect();
                    Some((tags, body))
                }
            })
            .collect();
        for (tags, body) in sections {
            println!("tags: {:?}", tags);
            println!("body: {}", body);
        }
        Ok(())
    }
}

impl CheckCommand {
    fn tokenize(path: &Path) -> Result<Vec<Section>, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let tokens = ManuscriptParser::new(&content).collect();
        Ok(tokens)
    }
}
