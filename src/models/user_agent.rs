use std::borrow::Cow;

/// The raw `User-Agent` header of a request.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UserAgent<'a> {
    /// The header value, or `None` when the request carries no `User-Agent` header.
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
