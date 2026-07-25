use std::borrow::Cow;

/// The CPU architecture a user agent runs on.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub struct CPU<'a> {
    /// The architecture, such as `amd64` or `arm64`, or `None` when no pattern matches.
    pub architecture: Option<Cow<'a, str>>,
}

impl<'a> CPU<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> CPU<'static> {
        let architecture = self.architecture.map(|c| Cow::from(c.into_owned()));

        CPU {
            architecture,
        }
    }
}
