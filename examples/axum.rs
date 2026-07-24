use std::sync::Arc;

use axum::{Extension, Router, routing::get};
use user_agent_parser::{CPU, Device, Engine, OS, Product, UserAgent, UserAgentParser};

// The models borrow from the request, so axum extractors must use the owned `T<'static>` form.
async fn index(
    user_agent: UserAgent<'static>,
    product: Product<'static>,
    os: OS<'static>,
    device: Device<'static>,
    cpu: CPU<'static>,
    engine: Engine<'static>,
) -> String {
    format!("{user_agent:#?}\n{product:#?}\n{os:#?}\n{device:#?}\n{cpu:#?}\n{engine:#?}")
}

#[tokio::main]
async fn main() {
    let user_agent_parser = Arc::new(UserAgentParser::from_path("uap-core/regexes.yaml").unwrap());

    let app = Router::new().route("/", get(index)).layer(Extension(user_agent_parser));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
