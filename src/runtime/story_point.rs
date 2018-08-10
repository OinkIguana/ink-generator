#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[doc(hidden)]
pub enum StoryPoint {
    Named(&'static str),
    Unnamed(&'static str),
}
