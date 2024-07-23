use onig::{Regex, RegexOptions, Syntax};
use yaml_rust2::Yaml;

use crate::UserAgentParserError;

#[derive(Debug)]
pub struct DeviceRegex {
    pub(crate) regex:              Regex,
    pub(crate) device_replacement: Option<String>,
    pub(crate) brand_replacement:  Option<String>,
    pub(crate) model_replacement:  Option<String>,
}

impl DeviceRegex {
    pub fn from_yaml(yaml: &Yaml) -> Result<Vec<DeviceRegex>, UserAgentParserError> {
        let yamls = yaml.as_vec().ok_or(UserAgentParserError::IncorrectSource)?;

        let yamls_len = yamls.len();

        if yamls_len == 0 {
            Err(UserAgentParserError::IncorrectSource)
        } else {
            let mut device_regexes = Vec::with_capacity(yamls_len);

            let yaml_regex = Yaml::String("regex".to_string());
            let yaml_device_replacement = Yaml::String("device_replacement".to_string());
            let yaml_brand_replacement = Yaml::String("brand_replacement".to_string());
            let yaml_model_replacement = Yaml::String("model_replacement".to_string());
            let yaml_regex_flag = Yaml::String("regex_flag".to_string());

            for yaml in yamls {
                let yaml = yaml.as_hash().ok_or(UserAgentParserError::IncorrectSource)?;

                let device_replacement = match yaml.get(&yaml_device_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let brand_replacement = match yaml.get(&yaml_brand_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let model_replacement = match yaml.get(&yaml_model_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let regex_options = if let Some(yaml) = yaml.get(&yaml_regex_flag) {
                    let regex_flag = yaml.as_str().ok_or(UserAgentParserError::IncorrectSource)?;

                    if regex_flag == "i" {
                        RegexOptions::REGEX_OPTION_IGNORECASE
                    } else {
                        RegexOptions::REGEX_OPTION_NONE
                    }
                } else {
                    RegexOptions::REGEX_OPTION_NONE
                };

                let regex = Regex::with_options(
                    yaml.get(&yaml_regex)
                        .ok_or(UserAgentParserError::IncorrectSource)?
                        .as_str()
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    regex_options,
                    Syntax::default(),
                )?;

                let device_regex = DeviceRegex {
                    regex,
                    device_replacement,
                    brand_replacement,
                    model_replacement,
                };

                device_regexes.push(device_regex);
            }

            Ok(device_regexes)
        }
    }
}
