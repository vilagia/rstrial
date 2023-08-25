use std::{path::Path, thread};

use llm_chain::{executor, parameters, prompt};
use rstrial_converter::converter::{vfm::line_converter::VfmLineConverter, LineConverter};
use rstrial_parser::{tokens::section::Section, ManuscriptParser};
use tokio::runtime::Runtime;

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

        let rt: Runtime = Runtime::new().unwrap();

        for (tags, body) in sections {
            let handle = rt.spawn(async move {
                let exec = executor!()?;
                let t: String = tags.clone().join(", ");
                println!("{body}");
                let res = prompt!(
                    "あなたは自動化された小説制作支援システムです。以下の文章は小説の一シーンです。以下の書式に沿って著者への助言を行ってください。\n\n - 各タグの妥当性と、評価の理由 \n - 追加タグ案 \n - タグへの適合性を向上させる施策",
                    format!("tags: {t}\n\n{body}\n\n").as_str(),
                )
                .run(&parameters!(), &exec).await.unwrap();
                println!("{}", res);
                Ok::<(),llm_chain::traits::ExecutorCreationError>(()) // <- note the explicit type annotation here
            });
            while !handle.is_finished() {
                thread::sleep(std::time::Duration::from_millis(100));
            }
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
