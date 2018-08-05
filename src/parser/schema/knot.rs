use super::super::Input;
use super::stitch::Stitch;
use crate::Error;

#[derive(Clone, Debug)]
crate struct Knot {
    crate entry: Option<Stitch>,
    crate stitches: Vec<(String, Stitch)>,
}

impl Knot {
    crate fn parse_name(string: &str) -> Option<String> {
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

    crate fn parse(lines: &mut Input) -> Result<Self, crate::Error> {
        let entry = if !lines.peek().ok_or(Error::Unknown)?.1.starts_with("=") {
            let stitch = Stitch::parse(lines)?;
            if stitch.is_empty() {
                None
            } else {
                Some(stitch)
            }
        } else {
            None
        };
        let mut stitches = vec![];
        loop {
            if let Some((i, line)) = lines.peek().cloned() {
                if line.starts_with("==") {
                    return Ok(Knot { entry, stitches });
                } else if line.starts_with("=") {
                    let name = Stitch::parse_name(&line).ok_or(Error::MissingStitchName(i))?;
                    lines.skip();
                    let stitch = Stitch::parse(lines)?;
                    stitches.push((name, stitch));
                }
            } else {
                return Ok(Knot { entry, stitches });
            }
        }
    }
}
