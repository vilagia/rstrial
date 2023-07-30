use crate::converter::SectionConverter;

use super::line_converter::VfmLineConverter;

pub struct VfmSectionConverter;

impl SectionConverter for VfmSectionConverter {
    type ItemConverter = VfmLineConverter;
}
