use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    io::Error as IOError,
};

use onig::Error as RegexError;
use yaml_rust2::ScanError;

#[derive(Debug)]
/// Possible errors of `UserAgentParser`.
pub enum UserAgentParserError {
    ScanError(ScanError),
    IOError(IOError),
    RegexError(RegexError),
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

impl Error for UserAgentParserError {}

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
