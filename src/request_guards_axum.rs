use std::{borrow::Cow, convert::Infallible, sync::Arc};

use axum::{
    extract::FromRequestParts,
    http::{header::USER_AGENT, request::Parts},
};

use crate::{UserAgentParser, models::*};

// axum extractors return owned values, so every model is produced as `T<'static>`.
// The parser must be shared through an `Extension<Arc<UserAgentParser>>` layer.

#[inline]
fn user_agent_str(parts: &Parts) -> Option<&str> {
    parts.headers.get(USER_AGENT).and_then(|value| value.to_str().ok())
}

#[inline]
fn parser(parts: &Parts) -> &Arc<UserAgentParser> {
    parts.extensions.get::<Arc<UserAgentParser>>().expect(
        "a `UserAgentParser` is not shared with axum; add an `Extension<Arc<UserAgentParser>>` \
         layer to the router",
    )
}

impl<S: Send + Sync> FromRequestParts<S> for UserAgent<'static> {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = user_agent_str(parts).map(|ua| Cow::from(ua.to_string()));

        Ok(UserAgent {
            user_agent,
        })
    }
}

macro_rules! impl_from_request_parts {
    ($model:ident, $parse:ident) => {
        impl<S: Send + Sync> FromRequestParts<S> for $model<'static> {
            type Rejection = Infallible;

            async fn from_request_parts(
                parts: &mut Parts,
                _state: &S,
            ) -> Result<Self, Self::Rejection> {
                // The parser is looked up first, so a missing layer is reported even by a request which carries no `User-Agent`.
                let user_agent_parser = parser(parts);

                // A request without a `User-Agent` header is normal, unlike a missing parser, so it just yields the default.
                let result = match user_agent_str(parts) {
                    Some(user_agent) => user_agent_parser.$parse(user_agent).into_owned(),
                    None => $model::default(),
                };

                Ok(result)
            }
        }
    };
}

impl_from_request_parts!(Product, parse_product);
impl_from_request_parts!(OS, parse_os);
impl_from_request_parts!(Device, parse_device);
impl_from_request_parts!(CPU, parse_cpu);
impl_from_request_parts!(Engine, parse_engine);
