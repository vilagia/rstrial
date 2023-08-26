pub mod entities;
pub mod parser;
pub mod tokens;

pub use parser::manuscript_parser::ManuscriptParser;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
