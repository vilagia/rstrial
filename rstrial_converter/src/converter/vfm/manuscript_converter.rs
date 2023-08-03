use crate::converter::ManuscriptConverter;

use super::section_converter::VfmSectionConverter;

pub struct VfmManuscriptConverter;

impl ManuscriptConverter for VfmManuscriptConverter {
    type ItemConverter = VfmSectionConverter;
}