use logos::Logos;

use crate::parser::line_item_parser::LineItemParser;

// Tokens for novel-style text.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum LineItem {
    // Plaintext to be rendered as-is.
    #[regex(r"[^!?！？。」{}]+", priority = 0, callback = LineItemParser::to_string)]
    Text(String),
    // A Sentence delimiter such as `,` or `、`.
    #[regex(r"[,、，]", LineItemParser::to_string)]
    Comma(String),
    // A comment that should be ignored.
    #[regex(r"\{#[^}]+\}", LineItemParser::to_comment_string)]
    Comment(String),
    // Text to be rendered with additional styles.
    #[regex(r"\{[^}]+\|[^}]+}", LineItemParser::to_ruby)]
    TextWithRuby((String, String)),
    // Text to be rendered with additional styles.
    #[regex(r"\{[^}]+\|\.}", LineItemParser::to_sesame)]
    TextWithSesame((String, char)),
    // End of sentence. Includes a string shows the end of sentence(e.g. `.`, `。` or `！`).
    #[regex(r"[!?！？。」]+", callback = LineItemParser::to_terminator)]
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

    use crate::tokens::line_item::Terminator;

    use super::LineItem;

    #[test]
    fn parse() {
        let cases = vec![
            (
                "吾輩は{猫|ねこ}である{#犬のほうが いいかも}???!?!?!?!！？名前は{まだ|.}無い。どこで生まれたのかとんと見当がつかぬ。",
                vec![
                LineItem::Text("吾輩は".to_string()),
                LineItem::TextWithRuby(("猫".to_string(), "ねこ".to_string())),
                LineItem::Text("である".to_string()),
                LineItem::Comment("犬のほうが いいかも".to_string()),
                LineItem::EndOfSentence(Terminator::Exclamation("???!?!?!?!！？".to_string())),
                LineItem::Text("名前は".to_string()),
                LineItem::TextWithSesame(("まだ".to_string(), '・')),
                LineItem::Text("無い".to_string()),
                LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
                LineItem::Text("どこで生まれたのかとんと見当がつかぬ".to_string()),
                LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
            ]),
            (
                "{吾輩は猫である|I am a cat}。",
                vec![
                LineItem::TextWithRuby(("吾輩は猫である".to_string(), "I am a cat".to_string())),
                LineItem::EndOfSentence(Terminator::Normal("。".to_string())),
            ]),
            (
                "「ああああ」",
                vec![
                LineItem::Text("「ああああ".to_string()),
                LineItem::EndOfSentence(Terminator::Normal("」".to_string())),
                ],
            )
        ];
        for (input, expected) in cases {
            let lexer = LineItem::lexer(input);
            let mut actual = vec![];
            for token in lexer {
                actual.push(token.unwrap());
            }
            assert_eq!(actual, expected);
        }
    }
}
