extern crate yaml_rust;
extern crate onig;

mod errors;
mod regexes;
mod models;

use std::path::Path;
use std::fs;
use std::borrow::Cow;

use yaml_rust::{YamlLoader, Yaml};
use onig::Regex;

use regexes::*;
use errors::*;

pub use models::*;

#[derive(Debug)]
pub struct UserAgentParser {
    replacement_regex: Regex,
    product_regexes: Vec<ProductRegex>,
    os_regexes: Vec<OSRegex>,
    device_regexes: Vec<DeviceRegex>,
    cpu_regexes: Vec<CPURegex>,
}

impl UserAgentParser {
    /// Read the list of regular expressions (YAML data) from a file to create a `UserAgentParser` instance.
    #[inline]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<UserAgentParser, UserAgentParserError> {
        let yaml = fs::read_to_string(path)?;

        Self::from_str(&yaml)
    }

    /// Read the list of regular expressions (YAML data) from a string to create a `UserAgentParser` instance.
    pub fn from_str<S: AsRef<str>>(yaml: S) -> Result<UserAgentParser, UserAgentParserError> {
        let yamls = YamlLoader::load_from_str(yaml.as_ref())?;

        if yamls.is_empty() {
            Err(UserAgentParserError::IncorrectSource)
        } else {
            let yaml = &yamls[0];

            match yaml.as_hash() {
                Some(yaml) => {
                    let user_agent_parsers = yaml.get(&Yaml::String("user_agent_parsers".to_string()));
                    let os_parsers = yaml.get(&Yaml::String("os_parsers".to_string()));
                    let device_parsers = yaml.get(&Yaml::String("device_parsers".to_string()));

                    let user_agent_regexes = match user_agent_parsers {
                        Some(user_agent_parsers) => {
                            ProductRegex::from_yaml(user_agent_parsers)?
                        }
                        None => Vec::new()
                    };

                    let os_regexes = match os_parsers {
                        Some(os_parsers) => {
                            OSRegex::from_yaml(os_parsers)?
                        }
                        None => Vec::new()
                    };

                    let device_regexes = match device_parsers {
                        Some(device_parsers) => {
                            DeviceRegex::from_yaml(device_parsers)?
                        }
                        None => Vec::new()
                    };

                    Ok(UserAgentParser {
                        replacement_regex: Regex::new(r"\$(\d){1,9}").unwrap(),
                        product_regexes: user_agent_regexes,
                        os_regexes,
                        device_regexes,
                        cpu_regexes: CPURegex::built_in_regexes(),
                    })
                }
                None => Err(UserAgentParserError::IncorrectSource)
            }
        }
    }
}

macro_rules! get_string {
    ($index:expr, $replacement:expr, $replacement_regex:expr, $captures:expr) => {
        match $replacement.as_ref() {
            Some(replacement) => {
                let replacement_captures_vec: Vec<_> = $replacement_regex.captures_iter(replacement).collect();

                if replacement_captures_vec.is_empty() {
                    Some(Cow::from(replacement))
                } else {
                    let mut replacement = replacement.to_string();

                    let captures_len = $captures.len();

                    for replacement_captures in replacement_captures_vec.into_iter().rev() {
                        let index = replacement_captures.at(1).unwrap().parse::<usize>().unwrap();

                        let pos = replacement_captures.pos(0).unwrap();

                        if index < captures_len {
                            replacement.replace_range(pos.0..pos.1, $captures.at(index).unwrap_or_default());
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
            None => match $captures.at($index) {
                Some(s) => {
                    let s = s.trim();

                    if s.is_empty() {
                        None
                    } else {
                        Some(Cow::from(s))
                    }
                }
                None => None
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
            None => None
        }
    };
}

impl UserAgentParser {
    pub fn parse_product<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Product<'a> {
        let mut product = Product::default();

        for product_regex in self.product_regexes.iter() {
            if let Some(captures) = product_regex.regex.captures(user_agent.as_ref()) {
                product.name = get_string!(1, product_regex.family_replacement, self.replacement_regex, captures);
                product.major = get_string!(2, product_regex.v1_replacement, self.replacement_regex, captures);
                product.minor = get_string!(3, product_regex.v2_replacement, self.replacement_regex, captures);
                product.patch = get_string!(4, product_regex.v3_replacement, self.replacement_regex, captures);

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
                os.major = get_string!(2, os_regex.os_v1_replacement, self.replacement_regex, captures);
                os.minor = get_string!(3, os_regex.os_v2_replacement, self.replacement_regex, captures);
                os.patch = get_string!(4, os_regex.os_v3_replacement, self.replacement_regex, captures);
                os.patch_minor = get_string!(5, os_regex.os_v4_replacement, self.replacement_regex, captures);

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
                device.name = get_string!(1, device_regex.device_replacement, self.replacement_regex, captures);
                device.brand = get_string!(2, device_regex.brand_replacement, self.replacement_regex, captures);
                device.model = get_string!(1, device_regex.model_replacement, self.replacement_regex, captures);

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
                cpu.architecture = get_string!(1, cpu_regex.architecture_replacement, self.replacement_regex, captures);

                break;
            }
        }

        cpu
    }
}