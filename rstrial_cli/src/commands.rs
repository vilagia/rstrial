pub mod convert;

pub trait Command {
    type Args;
    fn execute(&self, args: &Self::Args) -> Result<(), Box<dyn std::error::Error>>;
}