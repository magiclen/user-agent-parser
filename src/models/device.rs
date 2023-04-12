use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct Device<'a> {
    pub name:  Option<Cow<'a, str>>,
    pub brand: Option<Cow<'a, str>>,
    pub model: Option<Cow<'a, str>>,
}

impl<'a> Device<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> Device<'static> {
        let device = self.name.map(|c| Cow::from(c.into_owned()));
        let brand = self.brand.map(|c| Cow::from(c.into_owned()));
        let model = self.model.map(|c| Cow::from(c.into_owned()));

        Device {
            name: device,
            brand,
            model,
        }
    }
}
