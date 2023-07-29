use std::str::Chars;

use crate::tokens::{line_item::Attribute, LineItem};

pub struct RichTextParser<'a> {
    pub source: String,
    chars: Box<Chars<'a>>,
    state: State,
}

enum State {
    Text,
    Ruby,
}

impl<'a> RichTextParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            source: text.to_string(),
            chars: Box::new(text.chars()),
            state: State::Text,
        }
    }

    pub fn parse(&mut self) -> LineItem {
        let mut texts = vec![];
        let mut attribute_chars = vec![];
        for char in self.chars.by_ref() {
            match self.state {
                State::Text => match char {
                    '|' => {
                        self.state = State::Ruby;
                    }
                    _ => {
                        texts.push(char.to_string());
                    }
                },
                _ => {
                    attribute_chars.push(char.to_string());
                }
            }
        }
        let attribute = match self.state {
            State::Ruby => Attribute::Ruby(attribute_chars.concat()),
            State::Text => panic!("Invalid state."),
        };
        LineItem::RichText(texts.concat(), attribute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ruby() {
        let mut parser = RichTextParser::new("漢字|かんじ");
        let token = parser.parse();
        assert_eq!(
            token,
            LineItem::RichText("漢字".to_string(), Attribute::Ruby("かんじ".to_string()))
        );
    }
}
