use rstrial_parser::tokens::{Line, LineItem};

pub mod aozora;
pub mod vfm;

pub trait LineConverter {
    fn convert(line: Line) -> String {
        let breakline = "\n".to_string();
        match line {
            Line::Paragraph(items) => format!(
                "　{}",
                items
                    .into_iter()
                    .map(Self::convert_line_item)
                    .collect::<Vec<String>>()
                    .concat(),
            ),
            Line::Conversation(items) => format!(
                " {}",
                items
                    .into_iter()
                    .map(Self::convert_line_item)
                    .collect::<Vec<String>>()
                    .concat(),
            ),
            Line::Quotation(items) => format!(
                "> {}",
                items
                    .into_iter()
                    .map(Self::convert_line_item)
                    .collect::<Vec<String>>()
                    .concat()
            ),
            Line::Comment(_) => breakline,
        }
    }

    fn convert_line_item(item: LineItem) -> String;
}