use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io::Error as IOError,
};

use regex::Error as RegexError;
use yaml_rust::ScanError;

#[derive(Debug)]
/// Possible errors of `UserAgentParser`.
pub enum UserAgentParserError {
    /// The source is not valid YAML.
    ScanError(ScanError),
    /// The file of regular expressions could not be read.
    IOError(IOError),
    /// One of the patterns could not be compiled, or it is too large.
    RegexError(RegexError),
    /// The source is valid YAML, but it is not shaped like a list of regular expressions.
    IncorrectSource,
}

impl Display for UserAgentParserError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            UserAgentParserError::ScanError(err) => Display::fmt(&err, f),
            UserAgentParserError::IOError(err) => Display::fmt(&err, f),
            UserAgentParserError::RegexError(err) => Display::fmt(&err, f),
            UserAgentParserError::IncorrectSource => {
                f.write_str("The source of regular expressions is incorrect.")
            },
        }
    }
}

impl Error for UserAgentParserError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            UserAgentParserError::ScanError(err) => Some(err),
            UserAgentParserError::IOError(err) => Some(err),
            UserAgentParserError::RegexError(err) => Some(err),
            UserAgentParserError::IncorrectSource => None,
        }
    }
}

impl From<ScanError> for UserAgentParserError {
    #[inline]
    fn from(error: ScanError) -> UserAgentParserError {
        UserAgentParserError::ScanError(error)
    }
}

impl From<IOError> for UserAgentParserError {
    #[inline]
    fn from(error: IOError) -> UserAgentParserError {
        UserAgentParserError::IOError(error)
    }
}

impl From<RegexError> for UserAgentParserError {
    #[inline]
    fn from(error: RegexError) -> UserAgentParserError {
        UserAgentParserError::RegexError(error)
    }
}
