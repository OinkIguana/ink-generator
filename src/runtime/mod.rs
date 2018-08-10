pub use std::ops::{Generator, GeneratorState};
use std::sync::{Arc, Mutex};

mod part;
mod paragraph;
mod state;
mod story;
mod story_point;

pub use self::part::Part;
pub use self::paragraph::Paragraph;
pub use self::state::State;
pub use self::story::{Story, UnstartedStory, RegularStory, EndedStory, StoryID};
pub use self::story_point::StoryPoint;

pub type Input = Arc<Mutex<usize>>;
pub type WrappedState = Arc<Mutex<State>>;

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
