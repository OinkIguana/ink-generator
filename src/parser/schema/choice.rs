use uuid::Uuid;
use super::message::Message;
use crate::Error;

#[derive(Clone, Debug)]
crate struct Choice {
    crate sticky: bool,
    crate name: String,
    crate prefix: Message,
    crate choice: Message,
    crate suffix: Message,
}

impl Choice {
    crate fn parse(
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
            let parts = line.split(|c| c == '[' || c == ']').take(3).try_fold(
                vec![],
                |mut messages, string| {
                    messages.push(Message::parse(string, line_index)?);
                    Ok(messages)
                },
            )?;
            Ok(Some(Choice {
                sticky,
                name: format!("{}", Uuid::new_v4()),
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
