#[derive(Clone, Eq, PartialEq, Debug)]
#[doc(hidden)]
pub enum Part {
    Text(String),
    Glue,
}

impl Part {
    pub(super) fn to_string(&self) -> String {
        match self {
            Part::Text(string) => string.to_owned(),
            Part::Glue => " ".to_owned(),
        }
    }
}
