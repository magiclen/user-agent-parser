use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct OS<'a> {
    pub name:        Option<Cow<'a, str>>,
    pub major:       Option<Cow<'a, str>>,
    pub minor:       Option<Cow<'a, str>>,
    pub patch:       Option<Cow<'a, str>>,
    pub patch_minor: Option<Cow<'a, str>>,
}

impl<'a> OS<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> OS<'static> {
        let os = self.name.map(|c| Cow::from(c.into_owned()));
        let major = self.major.map(|c| Cow::from(c.into_owned()));
        let minor = self.minor.map(|c| Cow::from(c.into_owned()));
        let patch = self.patch.map(|c| Cow::from(c.into_owned()));
        let patch_minor = self.patch_minor.map(|c| Cow::from(c.into_owned()));

        OS {
            name: os,
            major,
            minor,
            patch,
            patch_minor,
        }
    }
}
