use std::collections::HashMap;
use super::StoryPoint;

#[derive(Clone, Default, Debug)]
pub struct State {
    pub(super) counts: HashMap<StoryPoint, usize>,
}

impl State {
    #[doc(hidden)]
    pub fn visit(&mut self, point: StoryPoint) {
        self.counts.entry(point).and_modify(|count| *count += 1).or_insert(1);
    }

    #[doc(hidden)]
    pub fn visits(&self, point: StoryPoint) -> usize {
        *self.counts.get(&point).unwrap_or(&0)
    }

    #[doc(hidden)]
    pub fn visited(&self, point: StoryPoint) -> bool {
        self.counts.get(&point).unwrap_or(&0) != &0
    }
}
