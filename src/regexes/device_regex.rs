use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, from_yaml_section, optional_replacement},
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
        let yaml_device_replacement = Yaml::String("device_replacement".to_string());
        let yaml_brand_replacement = Yaml::String("brand_replacement".to_string());
        let yaml_model_replacement = Yaml::String("model_replacement".to_string());

        from_yaml_section(yaml, |regex, yaml| {
            Ok(DeviceRegex {
                regex,
                device_replacement: optional_replacement(yaml, &yaml_device_replacement)?,
                brand_replacement: optional_replacement(yaml, &yaml_brand_replacement)?,
                model_replacement: optional_replacement(yaml, &yaml_model_replacement)?,
            })
        })
    }
}
