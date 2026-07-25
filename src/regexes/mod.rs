mod cpu_regex;
mod device_regex;
mod engine_regex;
mod os_regex;
mod product_regex;
mod replacement;

pub(crate) use cpu_regex::CPURegex;
pub(crate) use device_regex::DeviceRegex;
pub(crate) use engine_regex::EngineRegex;
pub(crate) use os_regex::OSRegex;
pub(crate) use product_regex::ProductRegex;
use regex::{Error as RegexError, Regex, RegexBuilder};
pub(crate) use replacement::{Replacement, resolve};
use yaml_rust::{Yaml, yaml::Hash};

use crate::UserAgentParserError;

/// A few uap-core patterns are huge literal alternations whose lazy DFA does not fit in the regex crate's default 2 MiB cache.
/// Once that cache starts thrashing, matching falls back to a far slower engine, which costs more than an order of magnitude.
/// The cliff currently sits between 3 MiB and 4 MiB, so this leaves plenty of room; the cache is grown on demand, not preallocated.
const DFA_SIZE_LIMIT: usize = 8 * 1024 * 1024;

/// Compiles a single pattern with the settings shared by every kind of parser.
#[inline]
pub(crate) fn build_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, RegexError> {
    RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .dfa_size_limit(DFA_SIZE_LIMIT)
        .build()
}

/// Walks one parser section of the YAML source, compiling the regex of each entry and letting `f` read the replacements it needs.
pub(crate) fn from_yaml_section<T>(
    yaml: &Yaml,
    mut f: impl FnMut(Regex, &Hash) -> Result<T, UserAgentParserError>,
) -> Result<Vec<T>, UserAgentParserError> {
    let yamls = yaml.as_vec().ok_or(UserAgentParserError::IncorrectSource)?;

    let mut entries = Vec::with_capacity(yamls.len());

    let yaml_regex = Yaml::String("regex".to_string());
    let yaml_regex_flag = Yaml::String("regex_flag".to_string());

    for yaml in yamls {
        let yaml = yaml.as_hash().ok_or(UserAgentParserError::IncorrectSource)?;

        let pattern = yaml
            .get(&yaml_regex)
            .ok_or(UserAgentParserError::IncorrectSource)?
            .as_str()
            .ok_or(UserAgentParserError::IncorrectSource)?;

        // uap-core only sets `regex_flag` on the device parsers, but the specification does not limit it to them.
        let case_insensitive = match yaml.get(&yaml_regex_flag) {
            Some(yaml) => yaml.as_str().ok_or(UserAgentParserError::IncorrectSource)? == "i",
            None => false,
        };

        entries.push(f(build_regex(pattern, case_insensitive)?, yaml)?);
    }

    Ok(entries)
}

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
