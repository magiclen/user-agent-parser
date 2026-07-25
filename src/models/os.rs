use std::borrow::Cow;

/// The operating system a user agent runs on.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct OS<'a> {
    /// The OS family, which falls back to `Some("Other")` when no pattern matches.
    pub name:        Option<Cow<'a, str>>,
    /// The major version, if the matching pattern captures one.
    pub major:       Option<Cow<'a, str>>,
    /// The minor version, if the matching pattern captures one.
    pub minor:       Option<Cow<'a, str>>,
    /// The patch version, if the matching pattern captures one.
    pub patch:       Option<Cow<'a, str>>,
    /// The patch minor version, if the matching pattern captures one.
    pub patch_minor: Option<Cow<'a, str>>,
}

impl<'a> OS<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> OS<'static> {
        let name = self.name.map(|c| Cow::from(c.into_owned()));
        let major = self.major.map(|c| Cow::from(c.into_owned()));
        let minor = self.minor.map(|c| Cow::from(c.into_owned()));
        let patch = self.patch.map(|c| Cow::from(c.into_owned()));
        let patch_minor = self.patch_minor.map(|c| Cow::from(c.into_owned()));

        OS {
            name,
            major,
            minor,
            patch,
            patch_minor,
        }
    }
}
