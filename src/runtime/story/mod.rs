pub use std::ops::{Generator, GeneratorState};
use super::{Input, WrappedState, Paragraph, Part, StoryPoint};

mod unstarted;
mod regular;
mod ended;

use self::{
    unstarted::UnstartedStory,
    regular::RegularStory,
    ended::EndedStory,
};

pub enum Story {
    Unstarted(UnstartedStory),
    Regular(RegularStory),
    Ended(EndedStory),
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "Story {{ .. }}")
    }
}

impl Story {
    #[doc(hidden)]
    pub fn new<Gen>(input: Input, state: WrappedState, generator: Gen) -> Self
    where
        Gen: Generator<Yield = Paragraph, Return = ()> + Sync + Send + 'static,
    {
        Story::Unstarted(UnstartedStory {
            input,
            state,
            generator: Box::new(generator),
        })
    }

    pub fn visits(&self, name: &'static str) -> usize {
        match self {
            Story::Unstarted(story) => story
                .state
                .lock()
                .unwrap()
                .counts
                .get(&StoryPoint::Named(name))
                .cloned()
                .unwrap_or_default(),
            Story::Regular(story) => story
                .state
                .lock()
                .unwrap()
                .counts
                .get(&StoryPoint::Named(name))
                .cloned()
                .unwrap_or_default(),
            Story::Ended(story) => story
                .state
                .counts
                .get(&StoryPoint::Named(name))
                .cloned()
                .unwrap_or_default(),
        }
    }
}
