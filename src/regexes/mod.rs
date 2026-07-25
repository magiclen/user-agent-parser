mod cpu_regex;
mod device_regex;
mod engine_regex;
mod os_regex;
mod product_regex;
mod replacement;

pub use cpu_regex::CPURegex;
pub use device_regex::DeviceRegex;
pub use engine_regex::EngineRegex;
pub use os_regex::OSRegex;
pub use product_regex::ProductRegex;
pub(crate) use replacement::{Replacement, resolve};
use yaml_rust::{Yaml, yaml::Hash};

use crate::UserAgentParserError;

/// Reads an optional replacement string out of a single parser entry of the YAML source.
pub(crate) fn optional_replacement(
    yaml: &Hash,
    key: &Yaml,
) -> Result<Option<Replacement>, UserAgentParserError> {
    match yaml.get(key) {
        Some(yaml) => {
            let text = yaml.as_str().ok_or(UserAgentParserError::IncorrectSource)?;

            Ok(Some(Replacement::new(text.to_string())))
        },
        None => Ok(None),
    }
}
