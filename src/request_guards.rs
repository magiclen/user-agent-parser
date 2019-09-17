use std::borrow::Cow;

use crate::rocket::request::{FromRequest, Outcome as OutcomeResult, Request};
use crate::rocket::{Outcome, State};

use crate::models::*;

use crate::UserAgentParser;

impl<'a, 'r> FromRequest<'a, 'r> for UserAgent<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent: Option<Cow<'a, str>> =
                request.headers().get("user-agent").next().map(Cow::from);

            UserAgent {
                user_agent,
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &UserAgent<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent: Option<Cow<'a, str>> =
                request.headers().get("user-agent").next().map(Cow::from);

            UserAgent {
                user_agent,
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Product<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_product(user_agent),
                None => Product::default(),
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &Product<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_product(user_agent),
                None => Product::default(),
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for OS<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_os(user_agent),
                None => OS::default(),
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &OS<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_os(user_agent),
                None => OS::default(),
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Device<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_device(user_agent),
                None => Device::default(),
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &Device<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_device(user_agent),
                None => Device::default(),
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CPU<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_cpu(user_agent),
                None => CPU::default(),
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &CPU<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_cpu(user_agent),
                None => CPU::default(),
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Engine<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        Outcome::Success({
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_engine(user_agent),
                None => Engine::default(),
            }
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for &Engine<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> OutcomeResult<Self, Self::Error> {
        let cache = request.local_cache(|| {
            let user_agent_parser = request.guard::<State<UserAgentParser>>().unwrap();

            let user_agent: Option<&str> = request.headers().get("user-agent").next();

            match user_agent {
                Some(user_agent) => user_agent_parser.inner().parse_engine(user_agent),
                None => Engine::default(),
            }
            .into_owned()
        });

        Outcome::Success(cache)
    }
}
