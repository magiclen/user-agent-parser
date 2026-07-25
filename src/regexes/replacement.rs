use std::borrow::Cow;

use regex::Captures;

/// A replacement string whose `$1` .. `$9` placeholders are parsed once, when the parser is built.
#[derive(Debug)]
pub(crate) enum Replacement {
    /// No placeholder at all, so the text can be used as is.
    Literal(Cow<'static, str>),
    /// A mix of literal text and capture group indexes.
    Template(Vec<Segment>),
}

#[derive(Debug)]
pub(crate) enum Segment {
    Literal(String),
    Capture(usize),
}

impl Replacement {
    pub(crate) fn new<S: Into<Cow<'static, str>>>(text: S) -> Replacement {
        let text = text.into();

        match Self::parse(text.as_ref()) {
            Some(segments) => Replacement::Template(segments),
            None => Replacement::Literal(text),
        }
    }

    /// Splits the text into segments, or returns `None` when there is no placeholder at all.
    fn parse(text: &str) -> Option<Vec<Segment>> {
        let bytes = text.as_bytes();

        let mut segments = Vec::new();
        let mut literal_start = 0;
        let mut i = 0;

        // A placeholder needs two bytes, so the last byte can never start one.
        while i + 1 < bytes.len() {
            if bytes[i] == b'$' && bytes[i + 1].is_ascii_digit() {
                if literal_start < i {
                    segments.push(Segment::Literal(text[literal_start..i].to_string()));
                }

                segments.push(Segment::Capture((bytes[i + 1] - b'0') as usize));

                i += 2;
                literal_start = i;
            } else {
                i += 1;
            }
        }

        if segments.is_empty() {
            return None;
        }

        if literal_start < bytes.len() {
            segments.push(Segment::Literal(text[literal_start..].to_string()));
        }

        Some(segments)
    }

    /// Resolves this replacement against the captures of a matched user-agent string.
    pub(crate) fn resolve<'a>(&'a self, captures: &Captures<'a>) -> Option<Cow<'a, str>> {
        match self {
            Replacement::Literal(text) => Some(Cow::from(text.as_ref())),
            Replacement::Template(segments) => {
                // A lone `$1` is by far the most common template, and it needs no allocation at all.
                if let [Segment::Capture(index)] = segments.as_slice() {
                    return capture_str(*index, captures).map(Cow::from);
                }

                let mut result = String::new();

                for segment in segments {
                    match segment {
                        Segment::Literal(text) => result.push_str(text),
                        // A group which does not exist or did not participate contributes nothing.
                        Segment::Capture(index) => {
                            result.push_str(captures.get(*index).map_or("", |m| m.as_str()))
                        },
                    }
                }

                let trimmed_end = result.trim_end().len();
                result.truncate(trimmed_end);

                let trimmed_start = result.len() - result.trim_start().len();
                result.drain(..trimmed_start);

                if result.is_empty() { None } else { Some(Cow::from(result)) }
            },
        }
    }
}

/// Resolves a replacement, falling back to the capture group at `index` when there is none.
#[inline]
pub(crate) fn resolve<'a>(
    index: usize,
    replacement: Option<&'a Replacement>,
    captures: &Captures<'a>,
) -> Option<Cow<'a, str>> {
    match replacement {
        Some(replacement) => replacement.resolve(captures),
        None => capture_str(index, captures).map(Cow::from),
    }
}

#[inline]
fn capture_str<'a>(index: usize, captures: &Captures<'a>) -> Option<&'a str> {
    let s = captures.get(index)?.as_str().trim();

    if s.is_empty() { None } else { Some(s) }
}
