use super::Part;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Paragraph {
    pub(super) parts: Vec<Part>,
    pub(super) choices: Option<Vec<Vec<Part>>>,
}

impl Paragraph {
    #[doc(hidden)]
    pub fn new(parts: Vec<Part>, choices: Option<Vec<Vec<Part>>>) -> Self {
        Self { parts, choices }
    }

    #[doc(hidden)]
    pub fn join(mut self, mut other: Paragraph) -> Self {
        self.parts.append(&mut other.parts);
        self.choices = other.choices;
        self
    }

    /// The string representation of this paragraph's text
    pub fn text(&self) -> String {
        self.parts
            .iter()
            .map(Part::to_string)
            .collect()
    }

    /// The string representations of this paragraph's choices
    pub fn choices(&self) -> Option<Vec<String>> {
        self.choices.as_ref().map(|choices| {
            choices
                .iter()
                .map(|parts| {
                    parts
                        .iter()
                        .map(Part::to_string)
                        .collect::<String>()
                }).collect()
        })
    }
}
