use std::borrow::Cow;

/// The physical device a user agent runs on.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Device<'a> {
    /// The device family, which falls back to `Some("Other")` when no pattern matches.
    pub name:  Option<Cow<'a, str>>,
    /// The manufacturer, which is only set when the matching pattern names one.
    pub brand: Option<Cow<'a, str>>,
    /// The model, if the matching pattern captures one.
    pub model: Option<Cow<'a, str>>,
}

impl<'a> Device<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> Device<'static> {
        let name = self.name.map(|c| Cow::from(c.into_owned()));
        let brand = self.brand.map(|c| Cow::from(c.into_owned()));
        let model = self.model.map(|c| Cow::from(c.into_owned()));

        Device {
            name,
            brand,
            model,
        }
    }
}
