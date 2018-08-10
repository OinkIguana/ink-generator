pub use std::ops::{Generator, GeneratorState};
use super::{Input, WrappedState, Paragraph, Part, StoryPoint};

pub struct Story {
    input: Input,
    state: WrappedState,
    buffered_paragraph: Option<Paragraph>,
    generator: Box<dyn Generator<Yield = Paragraph, Return = ()> + Sync + Send>,
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "Story {{}}")
    }
}

impl Story {
    #[doc(hidden)]
    pub fn new<Gen>(input: Input, state: WrappedState, mut generator: Gen) -> Self
    where
        Gen: Generator<Yield = Paragraph, Return = ()> + Sync + Send + 'static,
    {
        let buffered_paragraph = match unsafe { generator.resume() } {
            GeneratorState::Yielded(paragraph) => Some(paragraph),
            GeneratorState::Complete(..) => panic!("Trying to start an empty story is illegal."),
        };
        Story {
            input,
            state,
            buffered_paragraph,
            generator: Box::new(generator),
        }
    }

    pub fn visits(&self, name: &'static str) -> usize {
        self.state
            .lock()
            .unwrap()
            .counts
            .get(&StoryPoint::Named(name))
            .cloned()
            .unwrap_or_default()
    }

    // TODO: might be nice to make many structs (UnsartedStory, Story, and ChoiceStory) with only
    // the required methods available. That adds different pain to using this library though.
    pub unsafe fn next(mut self) -> (Paragraph, Option<Self>) {
        // NOTE on unsafety: This is not actually unsafe because of the generator usage, but
        // because calling the wrong method (of select or next) could lead to weird behaviour in
        // the output of the story. For now, that behaviour is undefined, and may cause a panic
        // or even prevent compilation in the future.

        // there's a lot of crazy moving and stuff going on here, but it works so whatever
        let mut output = self.buffered_paragraph.take();
        loop {
            if output.as_ref().and_then(Paragraph::choices).is_none() {
                match self.generator.resume() {
                    GeneratorState::Yielded(paragraph) => self.buffered_paragraph = Some(paragraph),
                    GeneratorState::Complete(..) => return (output.expect("Should have gotten output by now"), None),
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
            return (output.expect("Should have gotten output by now"), Some(self));
        }
    }

    pub unsafe fn select(self, input: usize) -> (Paragraph, Option<Self>) {
        *self.input.lock().unwrap() = input;
        self.next()
    }
}
