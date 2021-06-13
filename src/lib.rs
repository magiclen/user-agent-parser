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


```rust,ignore
extern crate user_agent_parser;

use user_agent_parser::UserAgentParser;

let ua_parser = UserAgentParser::from_path("/path/to/regexes.yaml").unwrap();
```

Use the `parse_*` methods and input a user-agent string to get information.

```rust,ignore
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

```rust,ignore
extern crate user_agent_parser;

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

```rust,ignore
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(UserAgentParser::from_path("/path/to/regexes.yaml").unwrap())
        .mount("/", routes![index])
}
```

## Testing

```bash
git clone --recurse-submodules git://github.com/magiclen/user-agent-parser.git

cd user-agent-parser

cargo test
```
*/

extern crate onig;
extern crate yaml_rust;

#[cfg(feature = "rocket")]
extern crate rocket;

mod errors;
mod models;
mod regexes;

#[cfg(feature = "rocket")]
mod request_guards;

use std::borrow::Cow;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use onig::Regex;
use yaml_rust::{Yaml, YamlLoader};

use regexes::*;

pub use errors::UserAgentParserError;
pub use models::*;

#[derive(Debug)]
pub struct UserAgentParser {
    replacement_regex: Regex,
    product_regexes: Vec<ProductRegex>,
    os_regexes: Vec<OSRegex>,
    device_regexes: Vec<DeviceRegex>,
    cpu_regexes: Vec<CPURegex>,
    engine_regexes: Vec<EngineRegex>,
}

impl UserAgentParser {
    /// Read the list of regular expressions (YAML data) from a file to create a `UserAgentParser` instance.
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<UserAgentParser, UserAgentParserError> {
        let yaml = fs::read_to_string(path)?;

        Self::from_str(&yaml)
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

                    let user_agent_regexes = match user_agent_parsers {
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
                        replacement_regex: Regex::new(r"\$(\d){1,9}").unwrap(),
                        product_regexes: user_agent_regexes,
                        os_regexes,
                        device_regexes,
                        cpu_regexes: CPURegex::built_in_regexes(),
                        engine_regexes: EngineRegex::built_in_regexes(),
                    })
                }
                None => Err(UserAgentParserError::IncorrectSource),
            }
        }
    }
}

macro_rules! get_string {
    ($index:expr, $replacement:expr, $replacement_regex:expr, $captures:expr) => {
        match $replacement.as_ref() {
            Some(replacement) => {
                let replacement_captures_vec: Vec<_> =
                    $replacement_regex.captures_iter(replacement).collect();

                if replacement_captures_vec.is_empty() {
                    Some(Cow::from(replacement))
                } else {
                    let mut replacement = replacement.to_string();

                    let captures_len = $captures.len();

                    for replacement_captures in replacement_captures_vec.into_iter().rev() {
                        let index = replacement_captures.at(1).unwrap().parse::<usize>().unwrap();

                        let pos = replacement_captures.pos(0).unwrap();

                        if index < captures_len {
                            replacement.replace_range(
                                pos.0..pos.1,
                                $captures.at(index).unwrap_or_default(),
                            );
                        } else {
                            replacement.replace_range(pos.0..pos.1, "");
                        }
                    }

                    let start_trimmed_replacement = replacement.trim_start();

                    if start_trimmed_replacement.len() != replacement.len() {
                        replacement = start_trimmed_replacement.trim_end().to_string();
                    } else {
                        replacement.truncate(replacement.trim_end().len());
                    }

                    if replacement.is_empty() {
                        None
                    } else {
                        Some(Cow::from(replacement))
                    }
                }
            }
            None => {
                match $captures.at($index) {
                    Some(s) => {
                        let s = s.trim();

                        if s.is_empty() {
                            None
                        } else {
                            Some(Cow::from(s))
                        }
                    }
                    None => None,
                }
            }
        }
    };

    ($index:expr, $captures:expr) => {
        match $captures.at($index) {
            Some(s) => {
                let s = s.trim();

                if s.is_empty() {
                    None
                } else {
                    Some(Cow::from(s))
                }
            }
            None => None,
        }
    };
}

impl UserAgentParser {
    pub fn parse_product<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Product<'a> {
        let mut product = Product::default();

        for product_regex in self.product_regexes.iter() {
            if let Some(captures) = product_regex.regex.captures(user_agent.as_ref()) {
                product.name = get_string!(
                    1,
                    product_regex.family_replacement,
                    self.replacement_regex,
                    captures
                );
                product.major =
                    get_string!(2, product_regex.v1_replacement, self.replacement_regex, captures);
                product.minor =
                    get_string!(3, product_regex.v2_replacement, self.replacement_regex, captures);
                product.patch =
                    get_string!(4, product_regex.v3_replacement, self.replacement_regex, captures);

                break;
            }
        }

        if product.name.is_none() {
            product.name = Some(Cow::from("Other"));
        }

        product
    }

    pub fn parse_os<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> OS<'a> {
        let mut os = OS::default();

        for os_regex in self.os_regexes.iter() {
            if let Some(captures) = os_regex.regex.captures(user_agent.as_ref()) {
                os.name = get_string!(1, os_regex.os_replacement, self.replacement_regex, captures);
                os.major =
                    get_string!(2, os_regex.os_v1_replacement, self.replacement_regex, captures);
                os.minor =
                    get_string!(3, os_regex.os_v2_replacement, self.replacement_regex, captures);
                os.patch =
                    get_string!(4, os_regex.os_v3_replacement, self.replacement_regex, captures);
                os.patch_minor =
                    get_string!(5, os_regex.os_v4_replacement, self.replacement_regex, captures);

                break;
            }
        }

        if os.name.is_none() {
            os.name = Some(Cow::from("Other"));
        }

        os
    }

    pub fn parse_device<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Device<'a> {
        let mut device = Device::default();

        for device_regex in self.device_regexes.iter() {
            if let Some(captures) = device_regex.regex.captures(user_agent.as_ref()) {
                device.name = get_string!(
                    1,
                    device_regex.device_replacement,
                    self.replacement_regex,
                    captures
                );
                device.brand = get_string!(
                    2,
                    device_regex.brand_replacement,
                    self.replacement_regex,
                    captures
                );
                device.model = get_string!(
                    1,
                    device_regex.model_replacement,
                    self.replacement_regex,
                    captures
                );

                break;
            }
        }

        if device.name.is_none() {
            device.name = Some(Cow::from("Other"));
        }

        device
    }

    pub fn parse_cpu<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> CPU<'a> {
        let mut cpu = CPU::default();

        for cpu_regex in self.cpu_regexes.iter() {
            if let Some(captures) = cpu_regex.regex.captures(user_agent.as_ref()) {
                cpu.architecture = get_string!(
                    1,
                    cpu_regex.architecture_replacement,
                    self.replacement_regex,
                    captures
                );

                break;
            }
        }

        cpu
    }

    pub fn parse_engine<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Engine<'a> {
        let mut engine = Engine::default();

        for engine_regex in self.engine_regexes.iter() {
            if let Some(captures) = engine_regex.regex.captures(user_agent.as_ref()) {
                engine.name =
                    get_string!(1, engine_regex.name_replacement, self.replacement_regex, captures);
                engine.major = get_string!(
                    2,
                    engine_regex.engine_v1_replacement,
                    self.replacement_regex,
                    captures
                );
                engine.minor = get_string!(
                    3,
                    engine_regex.engine_v2_replacement,
                    self.replacement_regex,
                    captures
                );
                engine.patch = get_string!(
                    4,
                    engine_regex.engine_v3_replacement,
                    self.replacement_regex,
                    captures
                );

                break;
            }
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
