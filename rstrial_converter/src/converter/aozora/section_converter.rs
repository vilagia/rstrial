use crate::converter::SectionConverter;

use super::line_converter::AozoraLineConverter;

pub struct AozoraSectionConverter;

impl SectionConverter for AozoraSectionConverter {
    type ItemConverter = AozoraLineConverter;
}
