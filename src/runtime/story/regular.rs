use std::sync::Arc;
pub use std::ops::{Generator, GeneratorState};
use super::{Story, StoryID, EndedStory, Paragraph, Input, WrappedState, Part};

pub struct RegularStory {
    pub(super) id: StoryID,
    pub(super) input: Input,
    pub(super) state: WrappedState,
    pub(super) buffered_paragraph: Option<Paragraph>,
    pub(super) generator: Box<dyn Generator<Yield = Paragraph, Return = ()> + Sync + Send>,
}

impl RegularStory {
    // TODO: might be nice to make many structs (UnsartedStory, Story, and ChoiceStory) with only
    // the required methods available. That adds different pain to using this library though.
    pub fn next(mut self) -> (Paragraph, Story) {
        // NOTE on unsafety: This is not actually unsafe because of the generator usage, but
        // because calling the wrong method (of select or next) could lead to weird behaviour in
        // the output of the story. For now, that behaviour is undefined, and may cause a panic
        // or even prevent compilation in the future.

        // there's a lot of crazy moving and stuff going on here, but it works so whatever
        let mut output = self.buffered_paragraph.take();
        loop {
            if output.as_ref().and_then(Paragraph::choices).is_none() {
                match unsafe { self.generator.resume() } {
                    GeneratorState::Yielded(paragraph) => self.buffered_paragraph = Some(paragraph),
                    GeneratorState::Complete(..) => return (
                        output.expect("Should have gotten output by now"),
                        Story::Ended(EndedStory {
                            id: self.id,
                            state: Arc::try_unwrap(self.state).unwrap().into_inner().unwrap(),
                        }),
                    ),
                }
            }
            if output.is_none() || // no output, we need to join
                (output.as_ref().unwrap().choices.is_none() && ( // otherwise, join if there's glue and no choices
                    output.as_ref().unwrap().parts.iter().last() == Some(&Part::Glue) ||
                    self.buffered_paragraph.as_ref().unwrap().parts.iter().next() == Some(&Part::Glue)
                )) {
                output = Some(output.unwrap_or_default().join(self.buffered_paragraph.take().unwrap()));
                continue;
            }
            return (output.expect("Should have gotten output by now"), Story::Regular(self));
        }
    }

    pub fn select(self, input: usize) -> (Paragraph, Story) {
        *self.input.lock().unwrap() = input;
        self.next()
    }
}
