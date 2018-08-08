use crate::Error;

#[derive(Clone, PartialEq, Eq, Debug)]
crate enum Part {
    Text(String),
    Divert(Option<String>),
    Glue,
    Break,
}

impl Part {
    fn is_empty(&self) -> bool {
        match self {
            Part::Text(text) => text.is_empty(),
            Part::Divert(Some(..)) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug)]
crate struct Message {
    crate parts: Vec<Part>,
}

impl Message {
    fn new(parts: Vec<Part>) -> Message {
        Message { parts }
    }

    crate fn empty() -> Self {
        Message { parts: vec![] }
    }

    crate fn is_empty(&self) -> bool {
        self.parts
            .iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .is_empty()
    }

    crate fn parse(string: &str, line_index: usize) -> Result<Self, Error> {
        parser::parse(string).map_err(|error| match error {
            parser::Error::InvalidIdentifierName => Error::InvalidIdentifierName(line_index),
        })
    }

    crate fn with_break(string: &str, line_index: usize) -> Result<Self, Error> {
        let mut message = Self::parse(string, line_index)?;
        message.parts.push(Part::Break);
        Ok(message)
    }
}

mod parser {
    use super::{Message, Part};
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub(super) enum Error {
        InvalidIdentifierName,
    }

    #[derive(Debug)]
    enum State {
        Start,
        Dash,
        DashGt,
        LBrace,
        LBraceTilde,
        LBraceBang,
        LBraceAmp,
        RBrace,
        Bar,
        Lt,
        LtGt,
        LtDash,
        Text(char),
    }

    impl State {
        fn to_token(self) -> Token {
            match self {
                State::Start => panic!("The Start state has no Token!"),
                State::LtDash => Token::LArrow,
                State::DashGt => Token::RArrow,
                State::LBrace => Token::Seq,
                State::LBraceAmp => Token::Cycle,
                State::LBraceBang => Token::OnceOnly,
                State::LBraceTilde => Token::Shuffle,
                State::RBrace => Token::EndSeq,
                State::Bar => Token::AltSeq,
                State::LtGt => Token::Glue,
                State::Dash => Token::Text('-'),
                State::Lt => Token::Text('<'),
                State::Text(ch) => Token::Text(ch),
            }
        }
    }

    #[derive(Debug)]
    enum Token {
        Text(char),
        LArrow,
        RArrow,
        Seq,
        Cycle,
        OnceOnly,
        Shuffle,
        EndSeq,
        AltSeq,
        Glue,
    }

    fn tokenize(mut string: &str) -> Result<Vec<Token>, Error> {
        fn lmm(state: State, string: &str) -> Result<(&str, Token), Error> {
            let mut chars = string.chars();
            match (state, chars.next()) {
                (State::Start, None) => {
                    unreachable!("Trying to munch empty string from Start state")
                }
                (State::Start, Some('{')) => lmm(State::LBrace, chars.as_str()),
                (State::Start, Some('}')) => lmm(State::RBrace, chars.as_str()),
                (State::Start, Some('-')) => lmm(State::Dash, chars.as_str()),
                (State::Start, Some('<')) => lmm(State::Lt, chars.as_str()),
                (State::Start, Some('|')) => lmm(State::Bar, chars.as_str()),
                (State::Start, Some(ch)) => lmm(State::Text(ch), chars.as_str()),
                (State::LBrace, Some('~')) => lmm(State::LBraceTilde, chars.as_str()),
                (State::LBrace, Some('!')) => lmm(State::LBraceBang, chars.as_str()),
                (State::LBrace, Some('&')) => lmm(State::LBraceAmp, chars.as_str()),
                (State::Lt, Some('>')) => lmm(State::LtGt, chars.as_str()),
                (State::Lt, Some('-')) => lmm(State::LtDash, chars.as_str()),
                (State::Dash, Some('>')) => lmm(State::DashGt, chars.as_str()),
                (state, _) => Ok((string, state.to_token())),
            }
        }

        let mut tokens = vec![];
        loop {
            if string.is_empty() {
                return Ok(tokens);
            }
            let (s, token) = lmm(State::Start, string)?;
            tokens.push(token);
            string = s;
        }
    }

    fn get_ident(mut tokens: &[Token]) -> Result<(Option<String>, &[Token]), Error> {
        let mut string = String::new();
        loop {
            if let Some((&Token::Text(ch), rest)) = tokens.split_first() {
                tokens = rest;
                if string.is_empty() && ch.is_whitespace() {
                    continue;
                }
                if (string.is_empty() || string.ends_with(".")) && ch == '.' {
                    return Err(Error::InvalidIdentifierName);
                }
                if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                    string.push(ch);
                }
            } else if string.is_empty() {
                return Ok((None, tokens));
            } else {
                return Ok((Some(string), tokens));
            }
        }
    }

    fn collect_text(mut tokens: &[Token]) -> (String, &[Token]) {
        let mut string = String::new();
        loop {
            if let Some((Token::Text(ch), rest)) = tokens.split_first() {
                string.push(*ch);
                tokens = rest;
            } else {
                return (string.trim().to_string(), tokens);
            }
        }
    }

    fn to_parts(tokens: &[Token]) -> Result<VecDeque<Part>, Error> {
        if let Some((token, rest)) = tokens.split_first() {
            match token {
                Token::Glue => {
                    let mut parts = to_parts(rest)?;
                    parts.push_front(Part::Glue);
                    Ok(parts)
                }
                Token::RArrow => {
                    let (ident, rest) = get_ident(rest)?;
                    let mut parts = to_parts(rest)?;
                    parts.push_front(Part::Divert(ident));
                    Ok(parts)
                }
                Token::Text(..) => {
                    let (text, rest) = collect_text(tokens);
                    let mut parts = to_parts(rest)?;
                    parts.push_front(Part::Text(text));
                    Ok(parts)
                }
                _ => unimplemented!(),
            }
        } else {
            Ok(VecDeque::new())
        }
    }

    pub(super) fn parse(string: &str) -> Result<Message, Error> {
        Ok(Message::new(
            to_parts(&tokenize(string)?)?.into_iter().collect(),
        ))
    }
}
