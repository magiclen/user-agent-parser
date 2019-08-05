use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct CPU<'a> {
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