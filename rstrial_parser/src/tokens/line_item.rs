// Tokens for novel-style text.
#[derive(Debug, PartialEq, Clone)]
pub enum LineItem {
    // Plaintext to be rendered as-is.
    Text(String),
    // A Sentence delimiter such as `,` or `、`.
    Comma(String),
    // A comment that should be ignored.
    Comment(String),
    // Text to be rendered with additional styles.
    RichText(String, Attribute),
    // End of sentence. Includes a string shows the end of sentence(e.g. `.`, `。` or `！`).
    EndOfSentence(Terminator),
    // End of paragraph.
    EndOfParagraph,
    // End of section such as a scene or a chapter. Includes a string shows the end of section(e.g. `†`).
    EndOfSection(String),
    // End of file.
    EOF,
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
