#[derive(Clone, Eq, PartialEq, Debug)]
#[doc(hidden)]
pub enum Part {
    Text(&'static str),
    Tag(&'static str),
    Glue,
}

impl Part {
    pub(super) fn to_string(&self) -> String {
        match self {
            Part::Text(string) => string.to_string(),
            Part::Tag(string) => string.to_string(),
            Part::Glue => String::from(" "),
        }
    }
}
