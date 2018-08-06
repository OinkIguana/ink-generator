pub use std::sync::Mutex;
pub use std::ops::{Generator, GeneratorState};
pub use std::sync::Arc;

fn collect_parts(mut collect: Vec<Part>, part: &Part) -> Vec<Part> {
    let len = collect.len();
    match (collect.iter().last(), part) {
        (None, part) => vec![part.clone()],
        (Some(Part::Text(prev)), Part::Text(next)) => {
            collect[len - 1] = Part::Text((prev.to_owned() + next).to_owned());
            collect
        }
        (Some(Part::Break), Part::Glue) => {
            collect[len - 1] = Part::Glue;
            collect
        }
        (Some(Part::Break), Part::Break) => collect,
        (Some(Part::Glue), Part::Break) => collect,
        (Some(Part::Glue), part) => {
            collect[len - 1] = part.clone();
            collect
        }
        (Some(..), part) => {
            collect.push(part.clone());
            collect
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[doc(hidden)]
pub enum Part {
    Text(String),
    Glue,
    Break,
}

impl Part {
    fn to_string(&self) -> String {
        match self {
            Part::Text(string) => string.to_owned(),
            Part::Glue => "".to_owned(),
            Part::Break => "\n".to_owned(),
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
            .fold(vec![], collect_parts)
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
                        .fold(vec![], collect_parts)
                        .iter()
                        .map(Part::to_string)
                        .collect::<String>()
                }).collect()
        })
    }
}

pub struct Story {
    input: Arc<Mutex<usize>>,
    generator: Box<dyn Generator<Yield = Paragraph, Return = ()> + Sync + Send>,
}

impl std::fmt::Debug for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        write!(f, "Story {{}}")
    }
}

impl Story {
    pub fn new<Gen>(input: Arc<Mutex<usize>>, generator: Gen) -> Self
    where
        Gen: Generator<Yield = Paragraph, Return = ()> + Sync + Send + 'static,
    {
        Story {
            input,
            generator: Box::new(generator),
        }
    }

    // TODO: might be nice to make many structs (UnsartedStory, Story, and ChoiceStory) with only
    // the required methods available. That adds different pain to using this library though.
    pub unsafe fn next(mut self) -> Option<(Paragraph, Self)> {
        // NOTE on unsafety: This is not actually unsafe because of the generator usage, but
        // because calling the wrong method (of select or next) could lead to weird behaviour in
        // the output of the story. For now, that behaviour is undefined, and may cause a panic
        // or even prevent compilation in the future.

        match self.generator.resume() {
            GeneratorState::Yielded(paragraph) => Some((paragraph, self)),
            GeneratorState::Complete(..) => None,
        }
    }

    pub unsafe fn select(self, input: usize) -> Option<(Paragraph, Self)> {
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
