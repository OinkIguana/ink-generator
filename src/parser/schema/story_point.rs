#[derive(Clone, Debug)]
pub(crate) enum StoryPoint {
    Named(String),
    Unnamed(String),
}
