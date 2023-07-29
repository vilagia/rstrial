use std::str::Lines;

use crate::tokens::{Line, LineItem};

use super::line_parser::LineParser;

pub struct SectionParser<'a> {
    pub source: Box<String>,
    lines: Box<Lines<'a>>,
}

impl<'a> SectionParser<'a> {
    pub fn new(section: &'a str) -> Self {
        Self {
            source: Box::new(section.to_string()),
            lines: Box::new(section.lines()),
        }
    }
}

impl<'a> Iterator for SectionParser<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line_str) = self.lines.next() {
            let line_parser: LineParser<'_> = LineParser::new(line_str);
            if line_str.starts_with("//") {
                Some(Line::Comment(
                    line_str.strip_prefix("//").unwrap().to_string(),
                ))
            } else if line_str.starts_with('「') {
                let items: Vec<LineItem> = line_parser.collect();
                Some(Line::Conversation(items))
            } else {
                let items: Vec<LineItem> = line_parser.collect();
                Some(Line::Paragraph(items))
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
        let section = "我が輩は猫である。\n名前はまだ無い。どこで生まれたのかとんと見当が付かぬ。\n// 猫でなく犬にすることも検討\n「にゃーにゃー」";
        let section_parser = SectionParser::new(section);
        let actual = section_parser.collect::<Vec<Line>>();
        let expected = vec![
            Line::Paragraph(vec![
                LineItem::Text("我が輩は猫である".to_string()),
                LineItem::EndOfSentence("。".to_string()),
            ]),
            Line::Paragraph(vec![
                LineItem::Text("名前はまだ無い".to_string()),
                LineItem::EndOfSentence("。".to_string()),
                LineItem::Text("どこで生まれたのかとんと見当が付かぬ".to_string()),
                LineItem::EndOfSentence("。".to_string()),
            ]),
            Line::Comment(" 猫でなく犬にすることも検討".to_string()),
            Line::Conversation(vec![
                LineItem::Text("「にゃーにゃー".to_string()),
                LineItem::EndOfSentence("」".to_string()),
            ]),
        ];
        assert_eq!(actual, expected);
    }
}
