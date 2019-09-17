use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct Engine<'a> {
    pub name: Option<Cow<'a, str>>,
    pub major: Option<Cow<'a, str>>,
    pub minor: Option<Cow<'a, str>>,
    pub patch: Option<Cow<'a, str>>,
}

impl<'a> Engine<'a> {
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
