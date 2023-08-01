use rstrial_parser::tokens::{Line, LineItem};

pub mod aozora;
pub mod vfm;

pub trait LineItemConverter {
    fn convert(item: LineItem) -> String;
}

pub trait LineConverter {
    type ItemConverter: LineItemConverter;

    fn line_separator() -> String {
        "\n".to_string()
    }

    fn convert(line: Line) -> String {
        let breakline = "\n".to_string();
        match line {
            Line::Paragraph(items) => format!(
                "　{}{}",
                items
                    .into_iter()
                    .map(Self::ItemConverter::convert)
                    .collect::<Vec<String>>()
                    .concat(),
                Self::line_separator(),
            ),
            Line::Conversation(items) => format!(
                " {}{}",
                items
                    .into_iter()
                    .map(Self::ItemConverter::convert)
                    .collect::<Vec<String>>()
                    .concat(),
                Self::line_separator(),
            ),
            Line::Quotation(items) => format!(
                "> {}{}",
                items
                    .into_iter()
                    .map(Self::ItemConverter::convert)
                    .collect::<Vec<String>>()
                    .concat(),
                Self::line_separator(),
            ),
            Line::Comment(_) => breakline,
        }
    }
}

pub trait SectionConverter {
    type ItemConverter: LineConverter;
    fn convert(lines: Vec<Line>) -> String {
        lines
            .into_iter()
            .map(Self::ItemConverter::convert)
            .collect::<Vec<String>>()
            .concat()
    }
}
