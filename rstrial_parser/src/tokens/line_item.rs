use logos::Logos;

use crate::parser::line_item_parser::LineItemParser;

// Tokens for novel-style text.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum LineItem {
    // Plaintext to be rendered as-is.
    #[regex(r"[^!?！？。」]+", priority = 0, callback = LineItemParser::to_string)]
    Text(String),
    // A Sentence delimiter such as `,` or `、`.
    #[regex(r",|、|，", LineItemParser::to_string)]
    Comma(String),
    // A comment that should be ignored.
    #[regex(r"\{#\w+\}", LineItemParser::to_string)]
    Comment(String),
    // Text to be rendered with additional styles.
    #[regex(r"\{.+|.+\}", LineItemParser::to_rich_text)]
    RichText((String, Attribute)),
    // End of sentence. Includes a string shows the end of sentence(e.g. `.`, `。` or `！`).
    #[regex(r"[!?！？。」]+", priority = 2, callback = LineItemParser::to_terminator)]
    EndOfSentence(Terminator),
    // End of section such as a scene or a chapter. Includes a string shows the end of section(e.g. `†`).
    EndOfSection(String),
}

// Tokens for Rich Text.
#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    // Ruby(furigana): a small text above the main text.
    Ruby(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Terminator {
    Normal(String),
    Exclamation(String),
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::tokens::line_item::{Attribute, Terminator};

    use super::LineItem;

    #[test]
    fn parse() {
        let line = LineItem::lexer("吾輩は{猫|ねこ}である{#犬のほうがいいかも}???!?!?!?!！？。名前はまだ無い。どこで生まれたのかとんと見当がつかぬ。").map(|item| item.unwrap()).collect::<Vec<_>>();
        assert_eq!(
            line,
            vec![
                LineItem::Text("吾輩は".to_string()),
                LineItem::RichText(("猫".to_string(), Attribute::Ruby("ねこ".to_string()))),
                LineItem::Text("である".to_string()),
                LineItem::Comment("犬のほうがいいかも".to_string()),
                LineItem::EndOfSentence(Terminator::Exclamation("???!?!?!?!！？。".to_string())),
                LineItem::Text("名前はまだ無い".to_string()),
                LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                LineItem::Text("どこで生まれたのかとんと見当がつかぬ".to_string()),
                LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
            ]
        );
    }
}
