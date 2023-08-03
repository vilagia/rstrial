use rstrial_parser::tokens::section::Section;

use crate::converter::{SectionConverter, LineConverter};

use super::line_converter::VfmLineConverter;

pub struct VfmSectionConverter;

impl SectionConverter for VfmSectionConverter {
    type ItemConverter = VfmLineConverter;

    fn convert(section: Section) -> String {
        match section {
            Section::Title(title) => format!("# {}\n", title),
            Section::Scene(_, body) => {
                body.into_iter()
                    .map(VfmLineConverter::convert)
                    .collect::<Vec<String>>()
                    .concat()
            },
        }
    }
}
