use std::borrow::Cow;

use regex::Captures;

/// The captured text is not known until a match happens, so each group gets a small fixed guess.
const CAPTURE_LENGTH_GUESS: usize = 8;

/// A replacement string whose `$1` .. `$9` placeholders are parsed once, when the parser is built.
#[derive(Debug, Clone)]
pub(crate) enum Replacement {
    /// No placeholder at all, so the text can be used as is.
    Literal(String),
    /// A lone placeholder, which is the most common replacement of uap-core by far and needs nothing but the capture group.
    Capture(usize),
    /// A mix of literal text and capture group indexes.
    Template {
        segments: Vec<Segment>,
        /// A guess at the length of the result, so that the string is allocated once instead of growing.
        capacity: usize,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum Segment {
    Literal(String),
    Capture(usize),
}

impl Replacement {
    pub(crate) fn new(text: &str) -> Replacement {
        // Catching a lone placeholder before parsing keeps most of the templates from allocating anything at all.
        if let [b'$', index] = text.as_bytes()
            && index.is_ascii_digit()
        {
            return Replacement::Capture((index - b'0') as usize);
        }

        match Self::parse(text) {
            Some(segments) => {
                let capacity = segments
                    .iter()
                    .map(|segment| match segment {
                        Segment::Literal(text) => text.len(),
                        Segment::Capture(_) => CAPTURE_LENGTH_GUESS,
                    })
                    .sum();

                Replacement::Template {
                    segments,
                    capacity,
                }
            },
            // uap-core asks for the result to be trimmed, and a literal can be trimmed once here instead of on every match.
            None => Replacement::Literal(text.trim().to_string()),
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
            Replacement::Literal(text) if text.is_empty() => None,
            Replacement::Literal(text) => Some(Cow::from(text.as_str())),
            Replacement::Capture(index) => capture_str(*index, captures).map(Cow::from),
            Replacement::Template {
                segments,
                capacity,
            } => {
                let mut result = String::with_capacity(*capacity);

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
pub(crate) fn capture_str<'a>(index: usize, captures: &Captures<'a>) -> Option<&'a str> {
    let s = captures.get(index)?.as_str().trim();

    if s.is_empty() { None } else { Some(s) }
}
