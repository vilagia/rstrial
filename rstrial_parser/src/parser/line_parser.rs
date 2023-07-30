use std::str::Chars;

use crate::tokens::LineItem;

use super::richtext_parser::RichTextParser;

#[derive(Debug)]
pub struct LineParser<'a> {
    pub source: Box<String>,
    chars: Box<Chars<'a>>,
    state: State,
    text_acc: Vec<String>,
    stacked_tokens: Vec<LineItem>,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Initial,
    Normal,
    Brace,
}

impl<'a> LineParser<'a> {
    pub fn new(line: &'a str) -> Self {
        Self {
            source: Box::new(line.to_string()),
            chars: Box::new(line.chars()),
            state: State::Initial,
            stacked_tokens: vec![],
            text_acc: vec![],
        }
    }
}

impl Iterator for LineParser<'_> {
    type Item = LineItem;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.stacked_tokens.pop() {
            return Some(token);
        }
        while let Some(result) = self.process_by_state() {
            return match result {
                ParseResult::Token(t) => {
                    Some(t)
                }
                ParseResult::ChangeState(new_state, Some(t)) => {
                    self.state = new_state;
                    Some(t)
                }
                ParseResult::ChangeState(new_state, None) => {
                    self.state = new_state;
                    self.next()
                }
                ParseResult::Continue(_) => {
                    continue
                }
            };
        }
        None
    }
}

impl<'a> LineParser<'a> {
    fn process_by_state(&mut self) -> Option<ParseResult> {
        if let Some(char) = self.chars.next() {
            let res = match self.state {
                State::Initial => match char {
                    ' ' | '　' => ParseResult::Continue(None),
                    _ => match char {
                        '{' => ParseResult::ChangeState(State::Brace, None),
                        _ => {
                            self.text_acc.push(char.to_string());
                            ParseResult::ChangeState(State::Normal, None)
                        }
                    },
                },
                State::Normal => match char {
                    '。' | '！' | '？' | '」' => Self::stack_and_parse(
                        LineItem::EndOfSentence(char.to_string()),
                        LineItem::Text(self.text_acc.concat()),
                        &mut self.stacked_tokens,
                        &mut self.text_acc,
                    ),
                    '、' | ',' => Self::stack_and_parse(
                        LineItem::Comma(char.to_string()),
                        LineItem::Text(self.text_acc.concat()),
                        &mut self.stacked_tokens,
                        &mut self.text_acc,
                    ),
                    '{' => {
                        let token = if !self.text_acc.is_empty() {
                            let res = Some(LineItem::Text(self.text_acc.concat()));
                            self.text_acc.clear();
                            res
                        } else {
                            None
                        };
                        ParseResult::ChangeState(State::Brace, token)
                    }
                    _ => {
                        self.text_acc.push(char.to_string());
                        ParseResult::Continue(None)
                    }
                },
                State::Brace => match char {
                    '}' => {
                        let rich_text_token =
                            RichTextParser::new(self.text_acc.concat().as_str()).parse();
                        self.text_acc.clear();
                        ParseResult::ChangeState(State::Normal, Some(rich_text_token))
                    }
                    _ => {
                        self.text_acc.push(char.to_string());
                        ParseResult::Continue(None)
                    }
                },
            };
            Some(res)
        } else {
            None
        }
    }

    fn stack_and_parse(
        to_stack: LineItem,
        to_return: LineItem,
        stack: &mut Vec<LineItem>,
        acc: &mut Vec<String>,
    ) -> ParseResult {
        stack.push(to_stack);
        let res = ParseResult::Token(to_return);
        acc.clear();
        res
    }
}

enum ParseResult {
    Token(LineItem),
    ChangeState(State, Option<LineItem>),
    Continue(Option<char>),
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
        #[test]
        fn it_returns_text_token_ruby_on_head() {
            let expected = vec![
                LineItem::RichText(
                    "吾輩".to_string(),
                    tokens::line_item::Attribute::Ruby("わがはい".to_string()),
                ),
                LineItem::Text("は猫である".to_string()),
                LineItem::EndOfSentence("。".to_string()),
                LineItem::Text("名前はまだ無い".to_string()),
                LineItem::EndOfSentence("。".to_string()),
                LineItem::Text("どこで生れたかとんと".to_string()),
                LineItem::RichText(
                    "見当".to_string(),
                    tokens::line_item::Attribute::Ruby("けんとう".to_string()),
                ),
                LineItem::Text("がつかぬ".to_string()),
                LineItem::EndOfSentence("。".to_string()),
            ];
            let parser = LineParser::new(
                "　　　　{吾輩|わがはい}は猫である。名前はまだ無い。どこで生れたかとんと{見当|けんとう}がつかぬ。",
            );
            let actual = parser.collect::<Vec<LineItem>>();
            assert_eq!(actual, expected);
        }
    }
}
