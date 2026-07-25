use std::borrow::Cow;

use rocket::{
    outcome::Outcome,
    request::{FromRequest, Outcome as OutcomeResult, Request},
};

use crate::{UserAgentParser, models::*};

const NOT_MANAGED: &str = "a `UserAgentParser` is not managed by Rocket; add \
                           `.manage(UserAgentParser::from_path(..))` when building the instance";

fn from_request_user_agent<'r>(request: &'r Request<'_>) -> UserAgent<'r> {
    let user_agent: Option<Cow<'r, str>> =
        request.headers().get("user-agent").next().map(Cow::from);

    UserAgent {
        user_agent,
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rocket")))]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success(from_request_user_agent(request))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rocket")))]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| from_request_user_agent(request).into_owned());

        Outcome::Success(cache)
    }
}

/// Implements both the owned and the cached-reference request guard of one model.
macro_rules! impl_from_request {
    ($model:ident, $parse:ident) => {
        #[cfg_attr(docsrs, doc(cfg(feature = "rocket")))]
        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for $model<'r> {
            type Error = ();

            async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
                Outcome::Success(parse(request))
            }
        }

        #[cfg_attr(docsrs, doc(cfg(feature = "rocket")))]
        #[rocket::async_trait]
        impl<'r> FromRequest<'r> for &$model<'r> {
            type Error = ();

            async fn from_request(request: &'r Request<'_>) -> OutcomeResult<Self, Self::Error> {
                let cache = request.local_cache_async(async { parse(request).into_owned() }).await;

                Outcome::Success(cache)
            }
        }

        fn parse<'r>(request: &'r Request<'_>) -> $model<'r> {
            let user_agent_parser = request.rocket().state::<UserAgentParser>().expect(NOT_MANAGED);

            match request.headers().get("user-agent").next() {
                Some(user_agent) => user_agent_parser.$parse(user_agent),
                None => $model::default(),
            }
        }
    };
}

// Each model gets its own module, so that the `parse` function of the macro does not clash.
mod product {
    use super::*;

    impl_from_request!(Product, parse_product);
}

mod os {
    use super::*;

    impl_from_request!(OS, parse_os);
}

mod device {
    use super::*;

    impl_from_request!(Device, parse_device);
}

mod cpu {
    use super::*;

    impl_from_request!(CPU, parse_cpu);
}

mod engine {
    use super::*;

    impl_from_request!(Engine, parse_engine);
}
