use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, build_regex, optional_replacement},
};

#[derive(Debug)]
pub(crate) struct DeviceRegex {
    pub(crate) regex:              Regex,
    pub(crate) device_replacement: Option<Replacement>,
    pub(crate) brand_replacement:  Option<Replacement>,
    pub(crate) model_replacement:  Option<Replacement>,
}

impl DeviceRegex {
    pub(crate) fn from_yaml(yaml: &Yaml) -> Result<Vec<DeviceRegex>, UserAgentParserError> {
        let yamls = yaml.as_vec().ok_or(UserAgentParserError::IncorrectSource)?;

        let mut device_regexes = Vec::with_capacity(yamls.len());

        let yaml_regex = Yaml::String("regex".to_string());
        let yaml_device_replacement = Yaml::String("device_replacement".to_string());
        let yaml_brand_replacement = Yaml::String("brand_replacement".to_string());
        let yaml_model_replacement = Yaml::String("model_replacement".to_string());
        let yaml_regex_flag = Yaml::String("regex_flag".to_string());

        for yaml in yamls {
            let yaml = yaml.as_hash().ok_or(UserAgentParserError::IncorrectSource)?;

            let case_insensitive = match yaml.get(&yaml_regex_flag) {
                Some(yaml) => yaml.as_str().ok_or(UserAgentParserError::IncorrectSource)? == "i",
                None => false,
            };

            let regex = build_regex(
                yaml.get(&yaml_regex)
                    .ok_or(UserAgentParserError::IncorrectSource)?
                    .as_str()
                    .ok_or(UserAgentParserError::IncorrectSource)?,
                case_insensitive,
            )?;

            let device_regex = DeviceRegex {
                regex,
                device_replacement: optional_replacement(yaml, &yaml_device_replacement)?,
                brand_replacement: optional_replacement(yaml, &yaml_brand_replacement)?,
                model_replacement: optional_replacement(yaml, &yaml_model_replacement)?,
            };

            device_regexes.push(device_regex);
        }

        Ok(device_regexes)
    }
}
