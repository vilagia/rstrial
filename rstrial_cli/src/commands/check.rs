use std::{path::Path, thread};

use llm_chain::{executor, parameters, prompt};
use rstrial_converter::converter::{vfm::line_converter::VfmLineConverter, LineConverter};
use rstrial_parser::{
    tokens::section::{Document, Section},
    ManuscriptParser,
};

use tokio_stream::StreamExt;

use super::Command;

static LLM_API_REQUEST_CHUNK_SIZE: usize = 10;

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
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let section_batches: Vec<_> = sections
            .chunks(LLM_API_REQUEST_CHUNK_SIZE)
            .enumerate()
            .collect();
        let batch_count = section_batches.len();
        for (index, batch) in section_batches {
            let mut spinning_circle = progress::SpinningCircle::new();
            spinning_circle.set_job_title(
                format!("Checking scenes batch {}/{}", index + 1, batch_count).as_str(),
            );
            let h = rt.spawn(Self::check_scenes(batch.to_vec()));
            while !h.is_finished() {
                thread::sleep(std::time::Duration::from_millis(100));
                spinning_circle.tick();
            }
            spinning_circle.jobs_done();
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

    async fn check_scenes(scenes: Vec<(Document, String)>) {
        let handles: Vec<_> = scenes
            .into_iter()
            .enumerate()
            .map(|(index, (doc, body))| Self::check_scene(index, doc, body))
            .collect();
        let mut results = tokio_stream::iter(handles).map(|h| h);
        let mut res = vec![];
        while let Some(r) = results.next().await {
            res.push(r.await);
        }
        res.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
        for (_, r) in res {
            println!("{}", r);
        }
    }

    async fn check_scene(index: usize, document: Document, body: String) -> (usize, String) {
        let tags = document.tags.join(", ");
        let exec: llm_chain_openai::chatgpt::Executor = executor!().unwrap();
        let temprate = r#"
            あなたは自動化された小説制作支援システムです。以下の文章は小説の一シーンです。以下の書式に沿って著者への助言を行ってください。
            
            - 各タグの妥当性と、評価の理由
            - 追加タグ案
            - タグへの適合性を向上させる施策
            "#;
        let res = prompt!(temprate, "tags:{{tags}}\n\n{{body}}\n\n",)
            .run(
                &parameters!(
                    "tags" => tags,
                    "body" => body,
                ),
                &exec,
            )
            .await
            .unwrap();
        let result = format!(
            r#"
## {} への分析結果

{}

-------------
            "#,
            document.title, res
        );
        (index, result)
    }
}
