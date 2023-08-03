use std::str::Chars;

use crate::tokens::{
    section::{Document, Section},
    Line,
};

use super::section_parser::SectionParser;

pub struct ManuscriptParser<'a> {
    pub source: Box<String>,
    scene: Option<Section>,
    chars: Box<Chars<'a>>,
    state: State,
    text_buffer: String,
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
            chars: Box::new(section.clone().chars()),
            text_buffer: "".to_string(),
            tags_buffer: vec![],
            scene: None,
        }
    }
}

impl<'a> Iterator for ManuscriptParser<'a> {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(character) = self.chars.next() {
            self.text_buffer.push(character);
            println!("{:?}: {:?}", self.state, self.text_buffer);
            // println!("{:?}", self.text_buffer);
            match self.state {
                State::Line => match &self.text_buffer {
                    buffer if buffer.starts_with('#') && buffer.ends_with('\n') => {
                        let title = buffer.strip_prefix("# ").unwrap().trim().to_string();
                        self.text_buffer.clear();
                        Some(Section::Title(title))
                    }
                    buffer if buffer.starts_with("```") && buffer.ends_with('\n') => {
                        self.state = State::MultiLine;
                        let scene_title = buffer
                            .strip_prefix("```")
                            .unwrap()
                            .strip_suffix('\n')
                            .unwrap()
                            .to_string();
                        self.scene = Some(Section::Scene(
                            Document::new(scene_title, None, vec![]),
                            vec![],
                        ));
                        self.text_buffer.clear();
                        self.next()
                    }
                    buffer if buffer.starts_with("@tags") && buffer.ends_with('\n') => {
                        let tags = buffer.strip_prefix("@tags").unwrap().trim().to_string();
                        let mut tags = tags
                            .split('/')
                            .map(|tag| tag.to_string())
                            .collect::<Vec<String>>();
                        self.tags_buffer.append(&mut tags);
                        self.text_buffer.clear();
                        self.next()
                    }
                    buffer if buffer.ends_with('\n') => {
                        self.text_buffer.clear();
                        self.next()
                    }
                    _ => self.next(),
                },
                State::MultiLine => match &self.text_buffer {
                    buffer if buffer.ends_with("```\n") => {
                        self.state = State::Line;
                        let scene_body = buffer.strip_suffix("```\n").unwrap().to_string();
                        let section_parser = SectionParser::new(scene_body.as_str());
                        let body: Vec<Line> = section_parser.collect::<Vec<Line>>();
                        self.text_buffer.clear();
                        if let Some(Section::Scene(mut document, _)) = self.scene.clone() {
                            document.tags = self.tags_buffer.clone();
                            self.tags_buffer.clear();
                            Some(Section::Scene(document, body))
                        } else {
                            None
                        }
                    }
                    _ => self.next(),
                },
            }
        } else {
            None
        }
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
