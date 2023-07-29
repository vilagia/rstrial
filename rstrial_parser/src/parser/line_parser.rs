use std::str::Chars;

use crate::tokens::LineItem;

#[derive(Debug)]
pub struct LineParser<'a> {
    pub source: Box<String>,
    chars: Box<Chars<'a>>,
    state: State,
    stacked_tokens: Vec<LineItem>,
}

#[derive(Debug)]
enum State {
    Normal,
    Brace,
}

impl<'a> LineParser<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            source: Box::new(line.to_string()),
            chars: Box::new(line.chars()),
            state: State::Normal,
            stacked_tokens: vec![],
        }
    }
}

impl Iterator for LineParser<'_> {
    type Item = LineItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.stacked_tokens.pop() {
            return Some(token);
        }
        let mut token: Option<LineItem> = None;
        match &self.state {
            State::Normal => {
                let mut texts = vec![];
                for char in self.chars.by_ref() {
                    match char {
                        '。' | '！' | '？' | '」' => {
                            self.stacked_tokens
                                .push(LineItem::EndOfSentence(char.to_string()));
                            token = Some(LineItem::Text(texts.concat()));
                            break;
                        }
                        '、' | ',' => {
                            self.stacked_tokens.push(LineItem::Comma(char.to_string()));
                            token = Some(LineItem::Text(texts.concat()));
                            break;
                        }
                        '{' => {
                            self.state = State::Brace;
                            token = self.next();
                            break;
                        }
                        _ => {
                            texts.push(char.to_string());
                        }
                    }
                }
            }
            State::Brace => {
                let mut texts = vec![];
                for char in self.chars.by_ref() {
                    match char {
                        '}' => {
                            self.state = State::Normal;
                            let rich_text: String = texts.concat();
                            let mut richtext_parser =
                                crate::parser::richtext_parser::RichTextParser::new(
                                    rich_text.as_str(),
                                );
                            token = Some(richtext_parser.parse());
                            break;
                        }
                        _ => {
                            texts.push(char.to_string());
                        }
                    }
                }
            }
        };
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod next {
        use crate::tokens::{self, LineItem};

        use super::*;

        #[test]
        fn it_returns_text_token() {
            let expected = vec![
                LineItem::Text("我が輩は".to_string()),
                LineItem::Comma("、".to_string()),
                LineItem::RichText(
                    "猫".to_string(),
                    tokens::line_item::Attribute::Ruby("ねこ".to_string()),
                ),
                LineItem::Text("である".to_string()),
                LineItem::EndOfSentence("。".to_string()),
                LineItem::Text("名前は".to_string()),
                LineItem::Comma("、".to_string()),
                LineItem::Text("まだ無い".to_string()),
                LineItem::EndOfSentence("。".to_string()),
            ];
            let parser = LineParser::new("我が輩は、{猫|ねこ}である。名前は、まだ無い。");
            let actual = parser.collect::<Vec<LineItem>>();
            assert_eq!(actual, expected);
        }
    }
}
