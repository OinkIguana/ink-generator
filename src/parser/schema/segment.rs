use crate::Error;

use super::super::Input;
use super::choice::Choice;
use super::message::Message;

#[derive(Clone, Debug)]
crate enum Segment {
    Text(Message),
    Choices(Vec<(Choice, Vec<Segment>)>),
}

impl Segment {
    crate fn is_empty(&self) -> bool {
        match self {
            Segment::Text(message) => message.is_empty(),
            Segment::Choices(..) => false,
        }
    }

    crate fn parse(depth: usize, lines: &mut Input) -> Result<Option<Segment>, Error> {
        if let Some((i, line)) = lines.peek().cloned() {
            if line.starts_with("=") {
                Ok(None)
            } else if line.starts_with("*") || line.starts_with("+") {
                let mut choices = vec![];
                loop {
                    if let Some((i, line)) = lines.peek().cloned() {
                        if let Some(choice) = Choice::parse(depth, i, line)? {
                            lines.skip();
                            let mut segments = vec![];
                            loop {
                                if let Some(segment) = Segment::parse(depth + 1, lines)? {
                                    segments.push(segment);
                                } else {
                                    break;
                                }
                            }
                            choices.push((choice, segments));
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if choices.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(Segment::Choices(choices)))
                }
            } else if line.starts_with("-") && !line.starts_with("->") {
                let mut count = 0usize;
                let mut line = line;
                while line.starts_with("-") && !line.starts_with("->") {
                    count += 1;
                    line = line[1..].trim_left();
                }
                if count == depth {
                    lines.skip();
                    Ok(Some(Segment::Text(Message::with_break(line, i)?)))
                } else if count < depth {
                    Ok(None)
                } else {
                    Err(Error::IncorrectCollectDepth(depth, count, i))
                }
            } else {
                lines.skip();
                Ok(Some(Segment::Text(Message::with_break(&line, i)?)))
            }
        } else {
            Ok(None)
        }
    }
}
