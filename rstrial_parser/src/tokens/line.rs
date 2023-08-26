use super::LineItem;

#[derive(Debug, PartialEq, Clone)]
pub enum Line {
    Paragraph(Vec<LineItem>),
    Conversation(Vec<LineItem>),
    Quotation(Vec<LineItem>),
    Comment(String),
}
