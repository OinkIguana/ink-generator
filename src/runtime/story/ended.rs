use super::super::{State, StoryPoint, StoryID};

#[derive(Clone)]
pub struct EndedStory {
    pub(super) id: StoryID,
    pub(super) state: State,
}

impl EndedStory {
    /// The number of times a named point in the story was visited
    pub fn visits(&self, name: &'static str) -> usize {
        self.state.visits(StoryPoint::Named(name))
    }

    /// Whether a named point in the story was visited
    pub fn visited(&self, name: &'static str) -> bool {
        self.state.visited(StoryPoint::Named(name))
    }
}
