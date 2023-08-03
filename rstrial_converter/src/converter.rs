use rstrial_parser::tokens::{section::Section, Line, LineItem};

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

    fn convert(section: Section) -> String {
        match section {
            Section::Title(_) => "".to_string(),
            Section::Scene(_, body) => body
                .into_iter()
                .map(Self::ItemConverter::convert)
                .collect::<Vec<String>>()
                .concat(),
        }
    }
}

pub trait ManuscriptConverter {
    type ItemConverter: SectionConverter;
    fn convert(sections: Vec<Section>) -> String {
        let mut previous_section = None;
        sections
        .into_iter()
        .map(|section| {
                let section_string = match section {
                    Section::Title(_) => format!("{}\n", Self::ItemConverter::convert(section.clone())),
                    Section::Scene(_, _) => {
                        match previous_section {
                            Some(Section::Title(_)) => format!("\n{}", Self::ItemConverter::convert(section.clone())),
                            _ => format!("\n†\n\n{}", Self::ItemConverter::convert(section.clone())),
                        }
                    }
                };
                previous_section = Some(section);
                section_string
            })
            .collect()
    }
}
