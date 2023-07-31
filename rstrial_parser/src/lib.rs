pub mod entities;
pub mod parser;
pub mod tokens;

pub use parser::section_parser::SectionParser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
