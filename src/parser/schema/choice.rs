use uuid::Uuid;
use super::{Message, StoryPoint};
use crate::Error;

#[derive(Clone, Debug)]
pub(crate) struct Choice {
    pub(crate) sticky: bool,
    pub(crate) name: StoryPoint,
    pub(crate) prefix: Message,
    pub(crate) choice: Message,
    pub(crate) suffix: Message,
}

impl Choice {
    pub(crate) fn parse(
        depth: usize,
        line_index: usize,
        mut line: &str,
    ) -> Result<Option<Choice>, Error> {
        if !line.starts_with("+") && !line.starts_with("*") {
            return Ok(None);
        }
        let sticky = line.starts_with("+");
        let pattern = if sticky { "+" } else { "*" };
        let mut count = 0usize;
        while line.starts_with(pattern) {
            count += 1;
            line = line[1..].trim_left();
        }
        if count == depth {
            let name = {
                let mut name = None;
                if line.starts_with("(") {
                    if let Some(end) = line.find(")") {
                        name = Some(StoryPoint::Named(line[1..end].to_string()));
                        line = &line[end + 1..];
                    }
                }
                name.unwrap_or(StoryPoint::Unnamed(format!("{}", Uuid::new_v4())))
            };
            let parts = line.split(|c| c == '[' || c == ']').take(3).try_fold(
                vec![],
                |mut messages, string| {
                    messages.push(Message::parse(string, line_index)?);
                    Ok(messages)
                },
            )?;
            Ok(Some(Choice {
                sticky,
                name,
                prefix: parts.get(0).cloned().unwrap_or(Message::empty()),
                choice: parts.get(1).cloned().unwrap_or(Message::empty()),
                suffix: parts.get(2).cloned().unwrap_or(Message::empty()),
            }))
        } else if count < depth {
            Ok(None)
        } else {
            Err(Error::IncorrectChoiceDepth(depth, count, line_index))
        }
    }
}
