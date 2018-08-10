pub use std::ops::{Generator, GeneratorState};
use super::{Story, RegularStory, Paragraph, Input, WrappedState};

pub struct UnstartedStory {
    pub(super) input: Input,
    pub(super) state: WrappedState,
    pub(super) generator: Box<dyn Generator<Yield = Paragraph, Return = ()> + Sync + Send>,
}

impl UnstartedStory {
    pub fn start(mut self) -> (Paragraph, Story) {
        let paragraph = match unsafe { self.generator.resume() } {
            GeneratorState::Yielded(paragraph) => paragraph,
            GeneratorState::Complete(..) => panic!("Trying to build an empty story is illegal."),
        };

        RegularStory {
            input: self.input,
            state: self.state,
            buffered_paragraph: Some(paragraph),
            generator: self.generator,
        }.next()
    }
}
