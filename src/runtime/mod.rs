pub use std::sync::Mutex;
pub use std::ops::{Generator, GeneratorState};
pub use std::sync::Arc;

#[derive(Clone, Eq, PartialEq, Debug)]
#[doc(hidden)]
pub enum Part {
    Text(String),
    Glue,
}

impl Part {
    fn to_string(&self) -> String {
        match self {
            Part::Text(string) => string.to_owned(),
            Part::Glue => " ".to_owned(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Paragraph {
    parts: Vec<Part>,
    choices: Option<Vec<Vec<Part>>>,
}

impl Paragraph {
    #[doc(hidden)]
    pub fn new(parts: Vec<Part>, choices: Option<Vec<Vec<Part>>>) -> Self {
        Self { parts, choices }
    }

    #[doc(hidden)]
    pub fn join(mut self, mut other: Paragraph) -> Self {
        self.parts.append(&mut other.parts);
        self
    }

    /// The string representation of this paragraph's text
    pub fn text(&self) -> String {
        self.parts
            .iter()
            .map(Part::to_string)
            .collect()
    }

    /// The string representations of this paragraph's choices
    pub fn choices(&self) -> Option<Vec<String>> {
        self.choices.as_ref().map(|choices| {
            choices
                .iter()
                .map(|parts| {
                    parts
                        .iter()
                        .map(Part::to_string)
                        .collect::<String>()
                }).collect()
        })
    }
}

pub struct Story {
    input: Arc<Mutex<usize>>,
    buffered_paragraph: Paragraph,
    generator: Box<dyn Generator<Yield = Paragraph, Return = ()> + Sync + Send>,
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "Story {{}}")
    }
}

impl Story {
    #[doc(hidden)]
    pub fn new<Gen>(input: Arc<Mutex<usize>>, mut generator: Gen) -> Self
    where
        Gen: Generator<Yield = Paragraph, Return = ()> + Sync + Send + 'static,
    {
        let buffered_paragraph = match unsafe { generator.resume() } {
            GeneratorState::Yielded(paragraph) => paragraph,
            GeneratorState::Complete(..) => panic!("Trying to start an empty story is illegal."),
        };
        Story {
            input,
            buffered_paragraph,
            generator: Box::new(generator),
        }
    }

    // TODO: might be nice to make many structs (UnsartedStory, Story, and ChoiceStory) with only
    // the required methods available. That adds different pain to using this library though.
    pub unsafe fn next(mut self) -> (Paragraph, Option<Self>) {
        // NOTE on unsafety: This is not actually unsafe because of the generator usage, but
        // because calling the wrong method (of select or next) could lead to weird behaviour in
        // the output of the story. For now, that behaviour is undefined, and may cause a panic
        // or even prevent compilation in the future.
        let mut output = self.buffered_paragraph;
        loop {
            match self.generator.resume() {
                GeneratorState::Yielded(paragraph) => self.buffered_paragraph = paragraph,
                GeneratorState::Complete(..) => return (output, None),
            }
            if output.parts.iter().last() == Some(&Part::Glue) || self.buffered_paragraph.parts.iter().next() == Some(&Part::Glue) {
                output = output.join(self.buffered_paragraph);
                continue;
            }
            return (output, Some(self));
        }
    }

    pub unsafe fn select(self, input: usize) -> (Paragraph, Option<Self>) {
        *self.input.lock().unwrap() = input;
        self.next()
    }
}

#[macro_export]
macro_rules! yield_all {
    ($generator:expr) => {
        loop {
            use std::ops::{Generator, GeneratorState};
            match unsafe { $generator.resume() } {
                GeneratorState::Yielded(y) => yield y,
                GeneratorState::Complete(r) => break r,
            }
        }
    };
}
