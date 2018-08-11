use super::Part;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Paragraph {
    pub(super) parts: Vec<Part>,
    pub(super) choices: Option<Vec<Vec<Part>>>,
    pub(super) tags: Vec<String>,
}

impl Paragraph {
    #[doc(hidden)]
    pub fn new(mut parts: Vec<Part>, choices: Option<Vec<Vec<Part>>>) -> Self {
        let tags = if let Some(Part::Tag(..)) = parts.iter().last() {
            vec![parts.pop().unwrap().to_string()]
        } else {
            vec![]
        };
        Self { parts, choices, tags }
    }

    #[doc(hidden)]
    pub fn join(mut self, mut other: Paragraph) -> Self {
        self.parts.append(&mut other.parts);
        self.choices = other.choices;
        self.tags.append(&mut other.tags);
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

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}
