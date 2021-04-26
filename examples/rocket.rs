#[macro_use]
extern crate rocket;

extern crate user_agent_parser;

use user_agent_parser::{Device, Engine, Product, UserAgent, UserAgentParser, CPU, OS};

#[get("/")]
fn index(
    user_agent: UserAgent,
    product: Product,
    os: OS,
    device: Device,
    cpu: CPU,
    engine: Engine,
) -> String {
    format!(
        "{user_agent:#?}\n{product:#?}\n{os:#?}\n{device:#?}\n{cpu:#?}\n{engine:#?}",
        user_agent = user_agent,
        product = product,
        os = os,
        device = device,
        cpu = cpu,
        engine = engine,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(UserAgentParser::from_path("uap-core/regexes.yaml").unwrap())
        .mount("/", routes![index])
}
