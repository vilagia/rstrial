use std::{path::Path, thread};

use llm_chain::{executor, parameters, prompt};
use rstrial_converter::converter::{vfm::line_converter::VfmLineConverter, LineConverter};
use rstrial_parser::{tokens::section::{Section, Document}, ManuscriptParser};
use tokio::runtime::Runtime;

use super::Command;

#[derive(Debug, clap::Args)]
pub struct CheckArgs {
    /// Target file path
    target: std::path::PathBuf,

    /// If true, use LLM for manuscript checking
    /// Currentry not implemented
    #[arg(long)]
    use_llm: bool,
}

pub struct CheckCommand;
impl Command for CheckCommand {
    type Args = CheckArgs;
    fn execute(&self, args: &Self::Args) -> Result<(), Box<dyn std::error::Error>> {
        let tokens = Self::tokenize(args.target.as_path())?;
        let sections: Vec<(Document, String)> = tokens
            .into_iter()
            .filter_map(|t| match t {
                Section::Title(_title) => None,
                Section::Scene(doc, body) => {
                    let body: String = body.into_iter().map(VfmLineConverter::convert).collect();
                    Some((doc, body))
                }
            })
            .collect();

        for (doc, body) in sections {
            Self::check_scene(doc, body);
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

    fn check_scene(document: Document, body: String) {
        let rt: Runtime = Runtime::new().unwrap();
        let mut spinning_circle = progress::SpinningCircle::new();
        spinning_circle.set_job_title("Waiting for LLM to finish");
        let tags = document.tags.join(", ");
        let handle = rt.spawn(async move {
            let exec: llm_chain_openai::chatgpt::Executor = executor!()?;
            println!("{body}");
            let temprate = r#"
            あなたは自動化された小説制作支援システムです。以下の文章は小説の一シーンです。以下の書式に沿って著者への助言を行ってください。
            
            - 各タグの妥当性と、評価の理由
            - 追加タグ案
            - タグへの適合性を向上させる施策
            "#;
            let res = prompt!(
                temprate,
                "tags:{{tags}}\n\n{{body}}\n\n",
            )
            .run(&parameters!(
                "tags" => tags,
                "body" => body,
            ), &exec).await.unwrap();
            println!("{}", res);
            Ok::<(),llm_chain::traits::ExecutorCreationError>(()) // <- note the explicit type annotation here
        });
        while !handle.is_finished() {
            thread::sleep(std::time::Duration::from_millis(100));
            spinning_circle.tick();
        }
        spinning_circle.jobs_done();
    }
}
