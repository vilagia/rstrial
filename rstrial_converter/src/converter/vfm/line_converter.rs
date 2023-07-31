use crate::converter::LineConverter;

use super::line_item_converter::VfmLineItemConverter;

pub struct VfmLineConverter;

impl LineConverter for VfmLineConverter {
    type ItemConverter = VfmLineItemConverter;
}
#[cfg(test)]
mod tests {
    use rstrial_parser::tokens::{line_item::Terminator, Line};

    use super::*;

    #[test]
    fn test_convert_paragraph() {
        let line = Line::Paragraph(vec![
            rstrial_parser::tokens::LineItem::Text("我が輩は".to_string()),
            rstrial_parser::tokens::LineItem::Comma("、".to_string()),
            rstrial_parser::tokens::LineItem::Comment("猫である。".to_string()),
            rstrial_parser::tokens::LineItem::RichText((
                "名前".to_string(),
                rstrial_parser::tokens::line_item::Attribute::Ruby("なまえ".to_string()),
            )),
            rstrial_parser::tokens::LineItem::Text("はまだ無い".to_string()),
            rstrial_parser::tokens::LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
        ]);
        let result = VfmLineConverter::convert(line);
        assert_eq!(result, "　我が輩は、{名前|なまえ}はまだ無い。");
    }

    #[test]
    fn test_convert_conversation() {
        let line = Line::Conversation(vec![
            rstrial_parser::tokens::LineItem::Text("「我が輩は".to_string()),
            rstrial_parser::tokens::LineItem::Comma("、".to_string()),
            rstrial_parser::tokens::LineItem::Comment("猫である。".to_string()),
            rstrial_parser::tokens::LineItem::RichText((
                "名前".to_string(),
                rstrial_parser::tokens::line_item::Attribute::Ruby("なまえ".to_string()),
            )),
            rstrial_parser::tokens::LineItem::Text("はまだ無い".to_string()),
            rstrial_parser::tokens::LineItem::EndOfSentence(Terminator::Normal("」".to_string())),
        ]);
        let result = VfmLineConverter::convert(line);
        assert_eq!(result, " 「我が輩は、{名前|なまえ}はまだ無い」");
    }
}
