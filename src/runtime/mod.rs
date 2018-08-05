pub use std::cell::Cell;
pub use std::ops::{Generator, GeneratorState};
pub use std::rc::Rc;

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
    input: Rc<Cell<usize>>,
    generator: Box<dyn Generator<Yield = Paragraph, Return = ()>>,
}

impl Story {
    pub fn new<Gen>(input: Rc<Cell<usize>>, generator: Gen) -> Self
    where
        Gen: Generator<Yield = Paragraph, Return = ()> + 'static,
    {
        Story {
            input,
            generator: Box::new(generator),
        }
    }

    pub fn next(mut self, input: usize) -> Option<(Paragraph, Self)> {
        self.input.set(input);
        match unsafe { self.generator.resume() } {
            GeneratorState::Yielded(paragraph) => Some((paragraph, self)),
            GeneratorState::Complete(..) => None,
        }
    }
}

macro_rules! yield_all {
    ($generator:expr) => {
        loop {
            match unsafe { $generator.resume() } {
                GeneratorState::Yielded(y) => yield y;
                GeneratorState::Complete(r) => break r;
            }
        }
    }
}
