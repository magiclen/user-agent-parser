use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, from_yaml_section, optional_replacement},
};

#[derive(Debug)]
pub(crate) struct OSRegex {
    pub(crate) regex:             Regex,
    pub(crate) os_replacement:    Option<Replacement>,
    pub(crate) os_v1_replacement: Option<Replacement>,
    pub(crate) os_v2_replacement: Option<Replacement>,
    pub(crate) os_v3_replacement: Option<Replacement>,
    pub(crate) os_v4_replacement: Option<Replacement>,
}

impl OSRegex {
    pub(crate) fn from_yaml(yaml: &Yaml) -> Result<Vec<OSRegex>, UserAgentParserError> {
        let yaml_os_replacement = Yaml::String("os_replacement".to_string());
        let yaml_os_v1_replacement = Yaml::String("os_v1_replacement".to_string());
        let yaml_os_v2_replacement = Yaml::String("os_v2_replacement".to_string());
        let yaml_os_v3_replacement = Yaml::String("os_v3_replacement".to_string());
        let yaml_os_v4_replacement = Yaml::String("os_v4_replacement".to_string());

        from_yaml_section(yaml, |regex, yaml| {
            Ok(OSRegex {
                regex,
                os_replacement: optional_replacement(yaml, &yaml_os_replacement)?,
                os_v1_replacement: optional_replacement(yaml, &yaml_os_v1_replacement)?,
                os_v2_replacement: optional_replacement(yaml, &yaml_os_v2_replacement)?,
                os_v3_replacement: optional_replacement(yaml, &yaml_os_v3_replacement)?,
                os_v4_replacement: optional_replacement(yaml, &yaml_os_v4_replacement)?,
            })
        })
    }
}
