use log::trace;
use std::str::Lines;

use crate::tokens::{
    section::{Document, Section},
    Line,
};

use super::section_parser::SectionParser;

#[derive(Debug)]
pub struct ManuscriptParser<'a> {
    pub source: Box<String>,
    scene: Option<Section>,
    lines: Box<Lines<'a>>,
    state: State,
    text_buffer: Vec<String>,
    tags_buffer: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Line,
    MultiLine,
}

impl<'a> ManuscriptParser<'a> {
    pub fn new(section: &'a str) -> Self {
        Self {
            source: Box::new(section.to_string()),
            state: State::Line,
            lines: Box::new(section.lines()),
            text_buffer: vec![],
            tags_buffer: vec![],
            scene: None,
        }
    }
}

impl<'a> Iterator for ManuscriptParser<'a> {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        let token = if let Some(line) = self.lines.next() {
            trace!("manuscript: {:?}, character: {:?}", self, line);
            match &self.state {
                State::Line => match line {
                    line if line.starts_with("# ") => {
                        let title = line.strip_prefix("# ").unwrap().to_string();
                        Some(Section::Title(title))
                    }
                    line if line.starts_with("```") => {
                        self.state = State::MultiLine;
                        let title = line.strip_prefix("```").unwrap().to_string();
                        self.scene = Some(Section::Scene(
                            Document::new(title, None, self.tags_buffer.clone()),
                            vec![],
                        ));
                        self.tags_buffer.clear();
                        self.next()
                    }
                    line if line.starts_with("@tags") => {
                        let tags = line
                            .strip_prefix("@tags ")?
                            .split('/')
                            .map(|tag| tag.to_string())
                            .collect::<Vec<String>>();
                        self.tags_buffer.extend(tags);
                        self.next()
                    }
                    _ => self.next(),
                },
                State::MultiLine => match line {
                    line if line.starts_with("```") => {
                        let body = self.text_buffer.join("\n");
                        self.text_buffer.clear();
                        let parser = SectionParser::new(body.as_str());
                        let body: Vec<Line> = parser.collect::<Vec<Line>>();
                        self.state = State::Line;
                        if let Some(Section::Scene(document, _)) = &self.scene {
                            Some(Section::Scene(document.clone(), body))
                        } else {
                            None
                        }
                    }
                    _ => {
                        self.text_buffer.push(line.to_string());
                        self.next()
                    }
                },
            }
        } else {
            None
        };
        trace!("parse: {:?}", token);
        token
    }
}

#[cfg(test)]
mod tests {

    use crate::tokens::{line_item::Terminator, LineItem};

    use super::*;

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                 "# タイトル\n以下、本文\n@tags 猫/夏目漱石\n```第一シーン\n吾輩は{猫|ねこ}である。名前は{まだ|.}無い。\nどこで生まれたのかとんと見当が付かぬ。\n```\n以上本文\n```第二シーン\nにゃあにゃあにゃあ。\n```\n",
                    vec![
                        Section::Title("タイトル".to_string()),
                        Section::Scene(
                            Document::new("第一シーン".to_string(), None, vec!["猫".to_string(), "夏目漱石".to_string()]),
                            vec![
                                Line::Paragraph(vec![
                                    LineItem::Text("吾輩は".to_string()),
                                    LineItem::TextWithRuby(("猫".to_string(), "ねこ".to_string())),
                                    LineItem::Text("である".to_string()),
                                    LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                                    LineItem::Text("名前は".to_string()),
                                    LineItem::TextWithSesame(("まだ".to_string(), '・')),
                                    LineItem::Text("無い".to_string()),
                                    LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                                ]),
                                Line::Paragraph(vec![
                                    LineItem::Text("どこで生まれたのかとんと見当が付かぬ".to_string()),
                                    LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                                ]),
                            ],
                        ),
                        Section::Scene(
                            Document::new("第二シーン".to_string(), None, vec![]),
                            vec![
                                Line::Paragraph(vec![
                                    LineItem::Text("にゃあにゃあにゃあ".to_string()),
                                    LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                                ]),
                            ],
                        ),
                    ],
        )];

        for (input, expected) in cases {
            let manuscript_parser = ManuscriptParser::new(input);
            let actual = manuscript_parser.collect::<Vec<Section>>();
            assert_eq!(actual, expected);
        }
    }
}
