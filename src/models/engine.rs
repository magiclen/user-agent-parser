use std::borrow::Cow;

use super::version::join_version;

/// The layout engine of the product a user agent belongs to.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Engine<'a> {
    /// The engine name, or `None` when no pattern matches.
    pub name:  Option<Cow<'a, str>>,
    /// The major version, if the matching pattern captures one.
    pub major: Option<Cow<'a, str>>,
    /// The minor version, if the matching pattern captures one.
    pub minor: Option<Cow<'a, str>>,
    /// The patch version, if the matching pattern captures one.
    pub patch: Option<Cow<'a, str>>,
}

impl<'a> Engine<'a> {
    /// Joins the version parts into a full version string, stopping at the first part which is missing.
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use user_agent_parser::Engine;
    ///
    /// let engine = Engine {
    ///     name:  Some(Cow::from("Blink")),
    ///     major: Some(Cow::from("57")),
    ///     minor: Some(Cow::from("0")),
    ///     patch: Some(Cow::from("2987")),
    /// };
    ///
    /// assert_eq!(Some(Cow::from("57.0.2987")), engine.version());
    /// ```
    #[inline]
    pub fn version(&self) -> Option<Cow<'_, str>> {
        join_version(&[self.major.as_deref(), self.minor.as_deref(), self.patch.as_deref()])
    }

    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> Engine<'static> {
        let name = self.name.map(|c| Cow::from(c.into_owned()));
        let major = self.major.map(|c| Cow::from(c.into_owned()));
        let minor = self.minor.map(|c| Cow::from(c.into_owned()));
        let patch = self.patch.map(|c| Cow::from(c.into_owned()));

        Engine {
            name,
            major,
            minor,
            patch,
        }
    }
}
