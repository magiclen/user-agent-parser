/*!
# User Agent Parser

A parser to get the product, OS, device, cpu, and engine information from a user agent, inspired by https://github.com/faisalman/ua-parser-js and https://github.com/ua-parser/uap-core

## Usage

You can make a **regexes.yaml** file or copy one from https://github.com/ua-parser/uap-core

This is a simple example of **regexes.yaml**.

```yaml
user_agent_parsers:
  - regex: '(ESPN)[%20| ]+Radio/(\d+)\.(\d+)\.(\d+) CFNetwork'
  - regex: '(Namoroka|Shiretoko|Minefield)/(\d+)\.(\d+)\.(\d+(?:pre|))'
    family_replacement: 'Firefox ($1)'
  - regex: '(Android) Eclair'
    v1_replacement: '2'
    v2_replacement: '1'

os_parsers:
  - regex: 'Win(?:dows)? ?(95|98|3.1|NT|ME|2000|XP|Vista|7|CE)'
    os_replacement: 'Windows'
    os_v1_replacement: '$1'

device_parsers:
  - regex: '\bSmartWatch *\( *([^;]+) *; *([^;]+) *;'
    device_replacement: '$1 $2'
    brand_replacement: '$1'
    model_replacement: '$2'
```

Then, use the `from_path` (or `from_str` if your YAML data is in-memory) associated function to create a `UserAgentParser` instance.


```rust,no_run
use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();
```

Use the `parse_*` methods and input a user-agent string to get information.

```rust,no_run
use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();

let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0 [FBAN/FBIOS;FBAV/8.0.0.28.18;FBBV/1665515;FBDV/iPhone4,1;FBMD/iPhone;FBSN/iPhone OS;FBSV/7.0.4;FBSS/2; FBCR/Telekom.de;FBID/phone;FBLC/de_DE;FBOP/5]";

let product = ua_parser.parse_product(user_agent);

println!("{:#?}", product);

//    Product {
//        name: Some(
//            "Facebook",
//        ),
//        major: Some(
//            "8",
//        ),
//        minor: Some(
//            "0",
//        ),
//        patch: Some(
//            "0",
//        ),
//    }

let os = ua_parser.parse_os(user_agent);

println!("{:#?}", os);

//    OS {
//        name: Some(
//            "iOS",
//        ),
//        major: None,
//        minor: None,
//        patch: None,
//        patch_minor: None,
//    }

let device = ua_parser.parse_device(user_agent);

println!("{:#?}", device);

//    Device {
//        name: Some(
//            "iPhone",
//        ),
//        brand: Some(
//            "Apple",
//        ),
//        model: Some(
//            "iPhone4,1",
//        ),
//    }

let cpu = ua_parser.parse_cpu(user_agent);

println!("{:#?}", cpu);

//    CPU {
//        architecture: Some(
//            "amd64",
//        ),
//    }

let engine = ua_parser.parse_engine(user_agent);

println!("{:#?}", engine);

//    Engine {
//        name: Some(
//            "Gecko",
//        ),
//        major: Some(
//            "10",
//        ),
//        minor: Some(
//            "0",
//        ),
//        patch: None,
//    }
```

The lifetime of result instances of the `parse_*` methods depends on the user-agent string and the `UserAgentParser` instance. To make it independent, call the `into_owned` method.

```rust,no_run
use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();

let product = ua_parser.parse_product("Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.2.12) Gecko/20101027 Ubuntu/10.04 (lucid) Firefox/3.6.12").into_owned();
```

## Rocket Support

This crate supports the Rocket framework. All you have to do is enabling the `rocket` feature for this crate.

```toml
[dependencies.user-agent-parser]
version = "*"
features = ["rocket"]
```

Let `Rocket` manage a `UserAgentParser` instance, and the `Product`, `OS`, `Device`, `CPU`, `Engine` models of this crate (plus the `UserAgent` model) can be used as *Request Guards*.

A request guard panics if no `UserAgentParser` is managed, because that is a mistake in the setup rather than something a single request can recover from. A request without a `User-Agent` header is fine and yields the default model.

```rust,ignore
#[macro_use]
extern crate rocket;

use user_agent_parser::{UserAgentParser, UserAgent, Product, OS, Device, CPU, Engine};

#[get("/")]
fn index(user_agent: UserAgent, product: Product, os: OS, device: Device, cpu: CPU, engine: Engine) -> String {
    format!("{user_agent:#?}\n{product:#?}\n{os:#?}\n{device:#?}\n{cpu:#?}\n{engine:#?}",
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
        .manage(UserAgentParser::from_path("/path/to/regexes.yaml").unwrap())
        .mount("/", routes![index])
}
```

## Axum Support

This crate also supports the Axum framework. All you have to do is enabling the `axum` feature for this crate.

```toml
[dependencies.user-agent-parser]
version = "*"
features = ["axum"]
```

Share a `UserAgentParser` instance through an `Extension<Arc<UserAgentParser>>` layer, and the owned models `Product<'static>`, `OS<'static>`, `Device<'static>`, `CPU<'static>`, `Engine<'static>` (plus the `UserAgent<'static>` model) can be used as *extractors*.

An extractor panics if that layer is missing, because that is a mistake in the setup rather than something a single request can recover from. A request without a `User-Agent` header is fine and yields the default model.

```rust,ignore
use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use user_agent_parser::{UserAgentParser, UserAgent, Product, OS, Device, CPU, Engine};

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
    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(Arc::new(UserAgentParser::from_path("/path/to/regexes.yaml").unwrap())));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
```

## Testing

```bash
# git clone --recurse-submodules git://github.com/magiclen/user-agent-parser.git

git clone git://github.com/magiclen/user-agent-parser.git

cd user-agent-parser

git submodule init
git submodule update --recursive

cargo test
```
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
mod models;
mod regexes;

#[cfg(feature = "rocket")]
mod request_guards;

#[cfg(feature = "axum")]
mod request_guards_axum;

use std::{borrow::Cow, fs, path::Path, str::FromStr};

pub use errors::UserAgentParserError;
pub use models::*;
use regexes::*;
use yaml_rust::{Yaml, YamlLoader};

/// A set of regular expressions which can extract the product, OS, device, CPU and engine information out of a user-agent string.
#[derive(Debug)]
pub struct UserAgentParser {
    product_regexes: Vec<ProductRegex>,
    os_regexes:      Vec<OSRegex>,
    device_regexes:  Vec<DeviceRegex>,
    cpu_regexes:     &'static [CPURegex],
    engine_regexes:  &'static [EngineRegex],
}

impl UserAgentParser {
    /// Read the list of regular expressions (YAML data) from a file to create a `UserAgentParser` instance.
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<UserAgentParser, UserAgentParserError> {
        let yaml = fs::read_to_string(path)?;

        Self::from_str(yaml)
    }

    /// Read the list of regular expressions (YAML data) from a string to create a `UserAgentParser` instance.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(yaml: S) -> Result<UserAgentParser, UserAgentParserError> {
        let yamls = YamlLoader::load_from_str(yaml.as_ref())?;

        if yamls.is_empty() {
            Err(UserAgentParserError::IncorrectSource)
        } else {
            let yaml = &yamls[0];

            match yaml.as_hash() {
                Some(yaml) => {
                    let user_agent_parsers =
                        yaml.get(&Yaml::String("user_agent_parsers".to_string()));
                    let os_parsers = yaml.get(&Yaml::String("os_parsers".to_string()));
                    let device_parsers = yaml.get(&Yaml::String("device_parsers".to_string()));

                    let product_regexes = match user_agent_parsers {
                        Some(user_agent_parsers) => ProductRegex::from_yaml(user_agent_parsers)?,
                        None => Vec::new(),
                    };

                    let os_regexes = match os_parsers {
                        Some(os_parsers) => OSRegex::from_yaml(os_parsers)?,
                        None => Vec::new(),
                    };

                    let device_regexes = match device_parsers {
                        Some(device_parsers) => DeviceRegex::from_yaml(device_parsers)?,
                        None => Vec::new(),
                    };

                    Ok(UserAgentParser {
                        product_regexes,
                        os_regexes,
                        device_regexes,
                        cpu_regexes: CPURegex::built_in_regexes(),
                        engine_regexes: EngineRegex::built_in_regexes(),
                    })
                },
                None => Err(UserAgentParserError::IncorrectSource),
            }
        }
    }
}

/// The results borrow from both the parser and the user-agent string, so call `into_owned` on them to break that tie.
impl UserAgentParser {
    /// Extracts the product information, falling back to a `name` of `Some("Other")` when no pattern matches.
    pub fn parse_product<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Product<'a> {
        let user_agent = user_agent.as_ref();

        let mut product = Product::default();

        // Do not replace this walk with a `RegexSet`, it was tried and it is much slower.
        // A `RegexSet` has to report every pattern that matches, so it cannot stop at the first one and cannot use the fast leftmost engine.
        // Trying the patterns one by one wins because each one carries its own literal prefilter, which rejects most of them almost for free.
        // Measured against uap-core (433 product, 204 OS and 633 device patterns) with `cargo bench`:
        // parse_product 94us -> 4852us, parse_os 20us -> 970us, parse_device 226us -> 4769us, and from_path 234ms -> 528ms.
        for product_regex in self.product_regexes.iter() {
            // `is_match` skips capture tracking, so the regexes that do not match cost much less than a `captures` call.
            if !product_regex.regex.is_match(user_agent) {
                continue;
            }

            let captures = product_regex.regex.captures(user_agent).unwrap();

            product.name = resolve(1, product_regex.family_replacement.as_ref(), &captures);
            product.major = resolve(2, product_regex.v1_replacement.as_ref(), &captures);
            product.minor = resolve(3, product_regex.v2_replacement.as_ref(), &captures);
            product.patch = resolve(4, product_regex.v3_replacement.as_ref(), &captures);

            break;
        }

        if product.name.is_none() {
            product.name = Some(Cow::from("Other"));
        }

        product
    }

    /// Extracts the OS information, falling back to a `name` of `Some("Other")` when no pattern matches.
    pub fn parse_os<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> OS<'a> {
        let user_agent = user_agent.as_ref();

        let mut os = OS::default();

        for os_regex in self.os_regexes.iter() {
            if !os_regex.regex.is_match(user_agent) {
                continue;
            }

            let captures = os_regex.regex.captures(user_agent).unwrap();

            os.name = resolve(1, os_regex.os_replacement.as_ref(), &captures);
            os.major = resolve(2, os_regex.os_v1_replacement.as_ref(), &captures);
            os.minor = resolve(3, os_regex.os_v2_replacement.as_ref(), &captures);
            os.patch = resolve(4, os_regex.os_v3_replacement.as_ref(), &captures);
            os.patch_minor = resolve(5, os_regex.os_v4_replacement.as_ref(), &captures);

            break;
        }

        if os.name.is_none() {
            os.name = Some(Cow::from("Other"));
        }

        os
    }

    /// Extracts the device information, falling back to a `name` of `Some("Other")` when no pattern matches.
    pub fn parse_device<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Device<'a> {
        let user_agent = user_agent.as_ref();

        let mut device = Device::default();

        for device_regex in self.device_regexes.iter() {
            if !device_regex.regex.is_match(user_agent) {
                continue;
            }

            let captures = device_regex.regex.captures(user_agent).unwrap();

            device.name = resolve(1, device_regex.device_replacement.as_ref(), &captures);
            // uap-core gives the brand no positional default; it is only set when a `brand_replacement` exists.
            device.brand =
                device_regex.brand_replacement.as_ref().and_then(|r| r.resolve(&captures));
            device.model = resolve(1, device_regex.model_replacement.as_ref(), &captures);

            break;
        }

        if device.name.is_none() {
            device.name = Some(Cow::from("Other"));
        }

        device
    }

    /// Extracts the CPU architecture, which stays `None` when no pattern matches.
    pub fn parse_cpu<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> CPU<'a> {
        let user_agent = user_agent.as_ref();

        let mut cpu = CPU::default();

        for cpu_regex in self.cpu_regexes.iter() {
            if !cpu_regex.regex.is_match(user_agent) {
                continue;
            }

            let captures = cpu_regex.regex.captures(user_agent).unwrap();

            cpu.architecture = resolve(1, cpu_regex.architecture_replacement.as_ref(), &captures);

            break;
        }

        cpu
    }

    /// Extracts the layout engine information, which stays `None` when no pattern matches.
    pub fn parse_engine<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Engine<'a> {
        let user_agent = user_agent.as_ref();

        let mut engine = Engine::default();

        for engine_regex in self.engine_regexes.iter() {
            if !engine_regex.regex.is_match(user_agent) {
                continue;
            }

            let captures = engine_regex.regex.captures(user_agent).unwrap();

            engine.name = resolve(1, engine_regex.name_replacement.as_ref(), &captures);
            engine.major = resolve(2, engine_regex.engine_v1_replacement.as_ref(), &captures);
            engine.minor = resolve(3, engine_regex.engine_v2_replacement.as_ref(), &captures);
            engine.patch = resolve(4, engine_regex.engine_v3_replacement.as_ref(), &captures);

            break;
        }

        engine
    }
}

impl FromStr for UserAgentParser {
    type Err = UserAgentParserError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UserAgentParser::from_str(s)
    }
}
