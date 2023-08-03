use crate::converter::ManuscriptConverter;

use super::section_converter::AozoraSectionConverter;

pub struct AozoraManuscriptConverter;

impl ManuscriptConverter for AozoraManuscriptConverter {
    type ItemConverter = AozoraSectionConverter;
}
