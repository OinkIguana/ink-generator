#[derive(Debug)]
pub enum Error {
    MissingKnotName(usize),
    MissingStitchName(usize),
    IncorrectChoiceDepth(usize, usize, usize),
    IncorrectCollectDepth(usize, usize, usize),
    InvalidIdentifierName(usize),
    InvalidEscapeSequence(usize),
    Unknown,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::MissingKnotName(line) => write!(f, "Knot is missing a name (line {})", line),
            Error::MissingStitchName(line) => write!(f, "Stitch is missing a name (line {})", line),
            Error::IncorrectChoiceDepth(expected, actual, line) => write!(
                f,
                "Incorrect choice depth. Expected {}, found {} (line {})",
                expected, actual, line
            ),
            Error::IncorrectCollectDepth(expected, actual, line) => write!(
                f,
                "Incorrect collect depth. Expected {} or less, found {} (line {})",
                expected, actual, line
            ),
            Error::InvalidIdentifierName(line) => {
                write!(f, "Invalid identifier name. (line {})", line)
            }
            Error::InvalidEscapeSequence(line) => {
                write!(f, "Invalid or incomplete escape sequence. (line {})", line)
            }
            _ => write!(f, "{}", std::error::Error::description(self)),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &'static str {
        match self {
            Error::MissingKnotName(..) => "Knot is missing a name",
            Error::MissingStitchName(..) => "Stitch is missing a name",
            Error::IncorrectChoiceDepth(..) => "Incorrect choice depth",
            Error::IncorrectCollectDepth(..) => "Incorrect collect depth",
            Error::InvalidIdentifierName(..) => "Invalid identifier name",
            Error::InvalidEscapeSequence(..) => "Invalid or incomplete escape sequence",
            Error::Unknown => "An unknown error occurred",
        }
    }
}
