use crate::error::Error;

#[derive(Debug)]
enum State {
    Text {
        start: bool,
    },
    TagName {
        start: usize,
        closing: bool,
    },
    Tag {
        start: usize,
        end: usize,
        closing: bool,
    },
}

impl State {
    fn new() -> Self {
        Self::Text { start: true }
    }

    fn parse<'a>(self, position: usize, c: char) -> Result<Self, Error<'a>> {
        match self {
            State::Text { start } => match c {
                '<' => Ok(Self::TagName {
                    start: position,
                    closing: false,
                }),
                c if start && c.is_whitespace() => Ok(self),
                c if start => Err(Error::UnexpectedCharacter(c, position)),
                _ => Ok(self),
            },
            Self::TagName { start, closing } => match c {
                '/' if !closing && position == start + 1 => Ok(Self::TagName {
                    start: position,
                    closing: true,
                }),
                '/' => Err(Error::UnexpectedCharacter(c, position)),
                '>' => Ok(Self::Tag {
                    start,
                    end: position,
                    closing,
                }),
                _ => Ok(self),
            },
            Self::Tag { .. } => match c {
                '<' => Ok(Self::TagName {
                    start: position,
                    closing: false,
                }),
                '>' => Err(Error::UnexpectedCharacter(c, position)),
                _ => Ok(Self::Text { start: false }),
            },
        }
    }
}

pub fn validate(input: &str) -> Result<(), Error> {
    let mut state = State::new();
    let mut tags = vec![];
    for (index, c) in input.char_indices() {
        state = state.parse(index, c)?;
        if let State::Tag {
            start,
            end,
            closing,
        } = state
        {
            let tag = &input[start + 1..end];
            if closing {
                let _ = tags
                    .pop()
                    .filter(|t| t == &tag)
                    .ok_or(Error::InvalidTag(tag))?;
            } else {
                tags.push(tag);
            }
        }
    }
    if tags.len() > 0 {
        Err(Error::ExtraTags(tags))
    } else {
        Ok(())
    }
}
