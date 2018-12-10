use super::super::Input;
use super::segment::Segment;
use crate::Error;

#[derive(Clone, Debug)]
pub(crate) struct Stitch {
    pub(crate) segments: Vec<Segment>,
}

impl Stitch {
    pub(crate) fn is_empty(&self) -> bool {
        self.segments
            .iter()
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .is_empty()
    }

    pub(crate) fn parse_name(string: &str) -> Option<String> {
        if string.starts_with("=") || string.starts_with(" ") {
            Self::parse_name(&string[1..])
        } else if string.ends_with("=") || string.ends_with(" ") {
            Self::parse_name(&string[..string.len() - 1])
        } else if string.is_empty() {
            None
        } else {
            Some(string.to_string())
        }
    }

    pub(crate) fn parse(lines: &mut Input) -> Result<Stitch, Error> {
        let mut segments = vec![];
        loop {
            if let Some((_, line)) = lines.peek() {
                if line.starts_with("=") {
                    return Ok(Stitch { segments });
                } else if let Some(segment) = Segment::parse(1, lines)? {
                    segments.push(segment);
                } else {
                    return Ok(Stitch { segments });
                }
            } else {
                return Ok(Stitch { segments });
            }
        }
    }
}
