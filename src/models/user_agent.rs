use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub struct UserAgent<'a> {
    pub user_agent: Option<Cow<'a, str>>,
}

impl<'a> UserAgent<'a> {
    /// Extracts the owned data.
    #[inline]
    pub fn into_owned(self) -> UserAgent<'static> {
        let user_agent = self.user_agent.map(|c| Cow::from(c.into_owned()));

        UserAgent {
            user_agent,
        }
    }
}