use logos::Lexer;

use crate::tokens::{
    line_item::{Attribute, Terminator},
    LineItem,
};

use super::terminator_parser::TerminatorParser;

pub struct LineItemParser;

impl LineItemParser {
    pub fn to_string(lex: &mut Lexer<LineItem>) -> String {
        lex.slice().to_owned()
    }

    pub fn to_comment_string(lex: &mut Lexer<LineItem>) -> String {
        lex.slice()
            .to_owned()
            .strip_prefix("{#")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .to_owned()
    }

    pub fn to_terminator(lex: &mut Lexer<LineItem>) -> Terminator {
        let slice = lex.slice().to_string();
        let parser = TerminatorParser::new(slice.as_str());
        parser.parse()
    }

    pub fn to_rich_text(lex: &mut Lexer<LineItem>) -> Option<(String, Attribute)> {
        let slice = lex.slice().to_string();
        let (text, attr) = slice
            .strip_prefix('{')?
            .strip_suffix('}')?
            .split_once('|')
            .map(|(a, b)| (a.to_string(), b.to_string()))?;
        Some((text, Attribute::Ruby(attr)))
    }
}
