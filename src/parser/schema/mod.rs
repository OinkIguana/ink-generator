use super::Input;
use crate::Error;
use std::collections::HashMap;

mod choice;
mod knot;
mod message;
mod segment;
mod stitch;
mod story_point;

pub(crate) use self::{knot::*, message::*, segment::*, stitch::*, story_point::*};

#[derive(Clone, Debug)]
pub struct Ink {
    pub(crate) entry: Vec<Segment>,
    pub(crate) knots: HashMap<String, Knot>,
}

impl Ink {
    pub(crate) fn parse(string: String) -> Result<Self, Error> {
        let mut lines = Input::new(
            string
                .lines()
                .enumerate()
                .map(|(i, line)| (i, line.trim()))
                .collect::<Vec<_>>(),
        );
        let mut segments = vec![];
        loop {
            if let Some((_, line)) = lines.peek() {
                if line.starts_with("=") {
                    break;
                } else if let Some(segment) = Segment::parse(1, &mut lines)? {
                    segments.push(segment);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let mut knots = HashMap::new();
        loop {
            if let Some((i, line)) = lines.next().cloned() {
                if line.is_empty() {
                    continue;
                }
                match &line.get(..2) {
                    Some("==") => {
                        let name = Knot::parse_name(&line).ok_or(Error::MissingKnotName(i))?;
                        let knot = Knot::parse(&mut lines)?;
                        knots.insert(name, knot);
                    }
                    _ => return Err(Error::Unknown),
                }
            } else {
                return Ok(Ink {
                    entry: segments,
                    knots,
                });
            }
        }
    }
}
