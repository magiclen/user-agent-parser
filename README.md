User Agent Parser
====================

[![Build Status](https://travis-ci.org/magiclen/user-agent-parser.svg?branch=master)](https://travis-ci.org/magiclen/user-agent-parser)

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


```rust
extern crate user_agent_parser;

use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();
```

Use the `parse_*` methods and input a user-agent string to get information.

```rust
extern crate user_agent_parser;

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

```rust
extern crate user_agent_parser;

use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();

let product = ua_parser.parse_product("Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.2.12) Gecko/20101027 Ubuntu/10.04 (lucid) Firefox/3.6.12").into_owned();
```

## Rocket Support

This crate supports the Rocket framework. All you have to do is enabling the `rocketly` feature for this crate.

```toml
[dependencies.user-agent-parser]
version = "*"
features = ["rocketly"]
```

Let `Rocket` manage a `UserAgentParser` instance, and the `Product`, `OS`, `Device`, `CPU`, `Engine` models of this crate (plus the `UserAgent` model) can be used as *Request Guards*.

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate user_agent_parser;

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

fn main() {
    rocket::ignite()
        .manage(UserAgentParser::from_path("/path/to/regexes.yaml").unwrap())
        .mount("/", routes![index])
        .launch();
}
```

## Testing

```bash
git clone --recurse-submodules git://github.com/magiclen/user-agent-parser.git

cd user-agent-parser

cargo test
```

## Crates.io

https://crates.io/crates/user-agent-parser

## Documentation

https://docs.rs/user-agent-parser

## License

[MIT](LICENSE)