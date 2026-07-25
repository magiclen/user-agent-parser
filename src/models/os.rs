use std::borrow::Cow;

use super::version::join_version;

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
    /// Joins the version parts into a full version string, stopping at the first part which is missing.
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use user_agent_parser::OS;
    ///
    /// let os = OS {
    ///     name:        Some(Cow::from("Mac OS X")),
    ///     major:       Some(Cow::from("10")),
    ///     minor:       Some(Cow::from("15")),
    ///     patch:       Some(Cow::from("7")),
    ///     patch_minor: None,
    /// };
    ///
    /// assert_eq!(Some(Cow::from("10.15.7")), os.version());
    /// ```
    #[inline]
    pub fn version(&self) -> Option<Cow<'_, str>> {
        join_version(&[
            self.major.as_deref(),
            self.minor.as_deref(),
            self.patch.as_deref(),
            self.patch_minor.as_deref(),
        ])
    }

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
