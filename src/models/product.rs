use std::borrow::Cow;

/// The product (usually the browser) a user agent belongs to.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Product<'a> {
    /// The product family, which falls back to `Some("Other")` when no pattern matches.
    pub name:  Option<Cow<'a, str>>,
    /// The major version, if the matching pattern captures one.
    pub major: Option<Cow<'a, str>>,
    /// The minor version, if the matching pattern captures one.
    pub minor: Option<Cow<'a, str>>,
    /// The patch version, if the matching pattern captures one.
    pub patch: Option<Cow<'a, str>>,
}

impl<'a> Product<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> Product<'static> {
        let name = self.name.map(|c| Cow::from(c.into_owned()));
        let major = self.major.map(|c| Cow::from(c.into_owned()));
        let minor = self.minor.map(|c| Cow::from(c.into_owned()));
        let patch = self.patch.map(|c| Cow::from(c.into_owned()));

        Product {
            name,
            major,
            minor,
            patch,
        }
    }
}
