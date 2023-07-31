use rstrial_parser::tokens::{line_item::Terminator, LineItem};

use crate::converter::LineItemConverter;

pub struct AozoraLineItemConverter;
impl LineItemConverter for AozoraLineItemConverter {
    fn convert(item: LineItem) -> String {
        let breakline = "\n".to_string();
        match item {
            LineItem::Text(text) => text,
            LineItem::Comma(comma) => comma,
            LineItem::Comment(_) => "".to_string(),
            LineItem::RichText((text, attribute)) => match attribute {
                rstrial_parser::tokens::line_item::Attribute::Ruby(ruby) => {
                    format!("|{text}《{ruby}》")
                }
            },
            LineItem::EndOfSentence(Terminator::Normal(terminator)) => terminator,
            LineItem::EndOfSentence(Terminator::Exclamation(terminator)) => {
                format!("{}　", terminator)
            }
            LineItem::EndOfParagraph => breakline,
            LineItem::EndOfSection(_) => breakline,
            LineItem::EOF => breakline,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_text() {
        let item = LineItem::Text("text".to_string());
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "text");
    }

    #[test]
    fn test_convert_comma() {
        let item = LineItem::Comma(",".to_string());
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, ",");
    }

    #[test]
    fn test_convert_comment() {
        let item = LineItem::Comment("comment".to_string());
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "");
    }

    #[test]
    fn test_convert_rich_text() {
        let item = LineItem::RichText((
            "text".to_string(),
            rstrial_parser::tokens::line_item::Attribute::Ruby("ruby".to_string()),
        ));
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "|text《ruby》");
    }

    #[test]
    fn test_convert_end_of_sentence() {
        let item = LineItem::EndOfSentence(Terminator::Normal(".".to_string()));
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, ".");
    }

    #[test]
    fn test_convert_end_of_paragraph() {
        let item = LineItem::EndOfParagraph;
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_convert_end_of_section() {
        let item = LineItem::EndOfSection("".to_string());
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_convert_eof() {
        let item = LineItem::EOF;
        let result = AozoraLineItemConverter::convert(item);
        assert_eq!(result, "\n");
    }
}
