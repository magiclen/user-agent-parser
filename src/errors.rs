use std::io::Error as IOError;

use crate::yaml_rust::ScanError;
use crate::onig::Error as RegexError;

#[derive(Debug)]
pub enum UserAgentParserError {
    ScanError(ScanError),
    IOError(IOError),
    RegexError(RegexError),
    IncorrectSource
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