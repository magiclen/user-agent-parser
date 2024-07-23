use onig::Regex;
use yaml_rust2::Yaml;

use crate::UserAgentParserError;

#[derive(Debug)]
pub struct ProductRegex {
    pub(crate) regex:              Regex,
    pub(crate) family_replacement: Option<String>,
    pub(crate) v1_replacement:     Option<String>,
    pub(crate) v2_replacement:     Option<String>,
    pub(crate) v3_replacement:     Option<String>,
}

impl ProductRegex {
    pub fn from_yaml(yaml: &Yaml) -> Result<Vec<ProductRegex>, UserAgentParserError> {
        let yamls = yaml.as_vec().ok_or(UserAgentParserError::IncorrectSource)?;

        let yamls_len = yamls.len();

        if yamls_len == 0 {
            Err(UserAgentParserError::IncorrectSource)
        } else {
            let mut user_agent_regexes = Vec::with_capacity(yamls_len);

            let yaml_regex = Yaml::String("regex".to_string());
            let yaml_family_replacement = Yaml::String("family_replacement".to_string());
            let yaml_v1_replacement = Yaml::String("v1_replacement".to_string());
            let yaml_v2_replacement = Yaml::String("v2_replacement".to_string());
            let yaml_v3_replacement = Yaml::String("v3_replacement".to_string());

            for yaml in yamls {
                let yaml = yaml.as_hash().ok_or(UserAgentParserError::IncorrectSource)?;

                let regex = Regex::new(
                    yaml.get(&yaml_regex)
                        .ok_or(UserAgentParserError::IncorrectSource)?
                        .as_str()
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                )?;

                let family_replacement = match yaml.get(&yaml_family_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let v1_replacement = match yaml.get(&yaml_v1_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let v2_replacement = match yaml.get(&yaml_v2_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let v3_replacement = match yaml.get(&yaml_v3_replacement) {
                    Some(yaml) => yaml
                        .as_str()
                        .map(|s| Some(s.to_string()))
                        .ok_or(UserAgentParserError::IncorrectSource)?,
                    None => None,
                };

                let user_agent_regex = ProductRegex {
                    regex,
                    family_replacement,
                    v1_replacement,
                    v2_replacement,
                    v3_replacement,
                };

                user_agent_regexes.push(user_agent_regex);
            }

            Ok(user_agent_regexes)
        }
    }
}
