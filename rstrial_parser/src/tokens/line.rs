use super::LineItem;

#[derive(Debug, PartialEq)]
pub enum Line {
    Paragraph(Vec<LineItem>),
    Conversation(Vec<LineItem>),
    Quotation(Vec<LineItem>),
    Comment(String),
}
