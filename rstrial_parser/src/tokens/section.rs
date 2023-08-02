use super::Line;

type Body = Vec<Line>;

#[derive(Debug, PartialEq, Clone)]
pub struct Manuscript {
    pub title: String,
    pub sections: Vec<Section>,
}

impl Manuscript {
    pub fn new(title: String) -> Self {
        Self {
            title,
            sections: vec![],
        }
    }

    pub fn push_section(&mut self, section: Section) {
        self.sections.push(section);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Section {
    Title(String),
    Scene(Document, Body),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub title: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
}
impl Document {
    pub fn new(title: String, body: Option<String>, tags: Vec<String>) -> Self {
        Self { title, body, tags }
    }
}
