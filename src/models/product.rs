use std::borrow::Cow;

use super::version::join_version;

/// The product (usually the browser) a user agent belongs to.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Product<'a> {
    /// The product family, which falls back to `Some("Other")` when no pattern matches.
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

impl<'a> Product<'a> {
    /// Joins the version parts into a full version string, stopping at the first part which is missing.
    ///
    /// ```
    /// use std::borrow::Cow;
    ///
    /// use user_agent_parser::Product;
    ///
    /// let product = Product {
    ///     name:        Some(Cow::from("Chrome")),
    ///     major:       Some(Cow::from("79")),
    ///     minor:       Some(Cow::from("0")),
    ///     patch:       Some(Cow::from("3945")),
    ///     patch_minor: Some(Cow::from("79")),
    /// };
    ///
    /// assert_eq!(Some(Cow::from("79.0.3945.79")), product.version());
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
    pub fn into_owned(self) -> Product<'static> {
        let name = self.name.map(|c| Cow::from(c.into_owned()));
        let major = self.major.map(|c| Cow::from(c.into_owned()));
        let minor = self.minor.map(|c| Cow::from(c.into_owned()));
        let patch = self.patch.map(|c| Cow::from(c.into_owned()));
        let patch_minor = self.patch_minor.map(|c| Cow::from(c.into_owned()));

        Product {
            name,
            major,
            minor,
            patch,
            patch_minor,
        }
    }
}
