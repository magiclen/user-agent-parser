use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, from_yaml_section, optional_replacement},
};

#[derive(Debug)]
pub(crate) struct ProductRegex {
    pub(crate) regex:              Regex,
    pub(crate) family_replacement: Option<Replacement>,
    pub(crate) v1_replacement:     Option<Replacement>,
    pub(crate) v2_replacement:     Option<Replacement>,
    pub(crate) v3_replacement:     Option<Replacement>,
}

impl ProductRegex {
    pub(crate) fn from_yaml(yaml: &Yaml) -> Result<Vec<ProductRegex>, UserAgentParserError> {
        let yaml_family_replacement = Yaml::String("family_replacement".to_string());
        let yaml_v1_replacement = Yaml::String("v1_replacement".to_string());
        let yaml_v2_replacement = Yaml::String("v2_replacement".to_string());
        let yaml_v3_replacement = Yaml::String("v3_replacement".to_string());

        from_yaml_section(yaml, |regex, yaml| {
            Ok(ProductRegex {
                regex,
                family_replacement: optional_replacement(yaml, &yaml_family_replacement)?,
                v1_replacement: optional_replacement(yaml, &yaml_v1_replacement)?,
                v2_replacement: optional_replacement(yaml, &yaml_v2_replacement)?,
                v3_replacement: optional_replacement(yaml, &yaml_v3_replacement)?,
            })
        })
    }
}
