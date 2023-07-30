use crate::tokens::line_item::Terminator;

pub struct TerminatorParser {
    pub source: String,
    state: State,
}

enum State {
    Normal,
}

impl TerminatorParser {
    pub fn new(text: &str) -> Self {
        Self {
            source: text.to_string(),
            state: State::Normal,
        }
    }

    pub fn parse(&self) -> Terminator {
        let terminator: String = self.source.clone();
        let exclamations = vec!["!", "?", "！", "？"];
        let is_exclamation_suffixed = exclamations
            .into_iter()
            .any(|exclamation| terminator.ends_with(exclamation));

        match is_exclamation_suffixed {
            true => Terminator::Exclamation(terminator),
            false => Terminator::Normal(terminator),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_terminator_exclamation() {
        let parser = TerminatorParser::new("！！。！？！！");
        let token = parser.parse();
        assert_eq!(token, Terminator::Exclamation("！！。！？！！".to_string()));
    }

    #[test]
    fn test_parse_terminator_normal() {
        let parser = TerminatorParser::new("！！。！？！」");
        let token = parser.parse();
        assert_eq!(token, Terminator::Normal("！！。！？！」".to_string()));
    }
}
