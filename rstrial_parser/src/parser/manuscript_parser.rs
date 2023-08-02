use std::str::Chars;

use crate::{
    tokens::{
        section::{Document, Section},
        Line,
    },
    SectionParser,
};

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
            // println!("{:?}", self.text_buffer);
            match self.state {
                State::Line => match &self.text_buffer {
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
                    buffer if buffer.starts_with('@') && buffer.ends_with('\n') => {
                        let tag = buffer.strip_prefix('@').unwrap().to_string();
                        self.tags_buffer.push(tag.trim().to_string());
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

    use super::*;

    #[test]
    fn test_parse() {
        let manuscript = "# タイトル\n以下、本文\n@猫\n@夏目漱石\n```第一シーン\n吾輩は猫である。名前はまだ無い。\nどこで生まれたのかとんと{見当|けんとう}が付かぬ。\n```\n以上本文\n```第二シーン\nにゃあにゃあにゃあ。\n```\n";
        let manuscript = ManuscriptParser::new(manuscript);
        for section in manuscript {
            println!("{:?}", section);
        }
    }
}
