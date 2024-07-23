use onig::Regex;
use yaml_rust2::Yaml;

use crate::UserAgentParserError;

#[derive(Debug)]
pub struct OSRegex {
    pub(crate) regex:             Regex,
    pub(crate) os_replacement:    Option<String>,
    pub(crate) os_v1_replacement: Option<String>,
    pub(crate) os_v2_replacement: Option<String>,
    pub(crate) os_v3_replacement: Option<String>,
    pub(crate) os_v4_replacement: Option<String>,
}

impl OSRegex {
    pub fn from_yaml(yaml: &Yaml) -> Result<Vec<OSRegex>, UserAgentParserError> {
        let yamls = yaml.as_vec().ok_or(UserAgentParserError::IncorrectSource)?;

        let yamls_len = yamls.len();

        if yamls_len == 0 {
            Err(UserAgentParserError::IncorrectSource)
        } else {
            let mut os_regexes = Vec::with_capacity(yamls_len);

            let yaml_regex = Yaml::String("regex".to_string());
            let yaml_os_replacement = Yaml::String("os_replacement".to_string());
            let yaml_os_v1_replacement = Yaml::String("os_v1_replacement".to_string());
            let yaml_os_v2_replacement = Yaml::String("os_v2_replacement".to_string());
            let yaml_os_v3_replacement = Yaml::String("os_v3_replacement".to_string());
            let yaml_os_v4_replacement = Yaml::String("os_v4_replacement".to_string());

            for yaml in yamls {
                let yaml = yaml.as_hash().ok_or(UserAgentParserError::IncorrectSource)?;

                let regex = Regex::new(
                    yaml.get(&yaml_regex)
                        .ok_or(UserAgentParserError::IncorrectSource)?
                        .as_str()
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                )?;

                let os_replacement = match yaml.get(&yaml_os_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let os_v1_replacement = match yaml.get(&yaml_os_v1_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let os_v2_replacement = match yaml.get(&yaml_os_v2_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let os_v3_replacement = match yaml.get(&yaml_os_v3_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let os_v4_replacement = match yaml.get(&yaml_os_v4_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let os_regex = OSRegex {
                    regex,
                    os_replacement,
                    os_v1_replacement,
                    os_v2_replacement,
                    os_v3_replacement,
                    os_v4_replacement,
                };

                os_regexes.push(os_regex);
            }

            Ok(os_regexes)
        }
    }
}
