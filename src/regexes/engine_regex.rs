use std::sync::LazyLock;

use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, build_regex, from_yaml_section, optional_replacement},
};

#[derive(Debug, Clone)]
pub(crate) struct EngineRegex {
    pub(crate) regex:              Regex,
    pub(crate) engine_replacement: Option<Replacement>,
}

/// The built-in patterns never change, so they are compiled once instead of once per `UserAgentParser`.
static REGEXES: LazyLock<Vec<EngineRegex>> = LazyLock::new(|| {
    #[inline]
    fn engine(pattern: &str, name: &str) -> EngineRegex {
        EngineRegex {
            regex:              build_regex(pattern, false).unwrap(),
            engine_replacement: Some(Replacement::new(name)),
        }
    }

    vec![
        engine(r"(?i)(windows.+\sedge)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "EdgeHTML"),
        // Blink is WebKit 537.36 with Chrome 28 or later, and its version is taken from the Chrome token.
        // `\d{3,}` must come first, otherwise `[3-9]\d` would cut `130` down to `13`.
        engine(
            r"(?i)webkit/537\.36.+(chrome)/(\d{3,}|2[89]|[3-9]\d)(?:\.(\w+))?(?:\.(\w+))?",
            "Blink",
        ),
        engine(r"(?i)(presto)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Presto"),
        engine(r"(?i)(webkit)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "WebKit"),
        engine(r"(?i)(trident)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Trident"),
        engine(r"(?i)(netfront)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "NetFront"),
        engine(r"(?i)(netsurf)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "NetSurf"),
        engine(r"(?i)(amaya)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Amaya"),
        engine(r"(?i)(servo)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Servo"),
        engine(r"(?i)(lynx)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Lynx"),
        engine(r"(?i)(w3m)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "w3m"),
        engine(r"(?i)(goanna)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Goanna"),
        engine(r"(?i)(arkweb)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "ArkWeb"),
        engine(r"(?i)ekioh(flow)/(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Flow"),
        engine(r"(?i)(khtml)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?", "KHTML"),
        engine(r"(?i)(tasman)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Tasman"),
        engine(r"(?i)(links)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Links"),
        // Dillo has to stay behind Lynx, because a Lynx user agent can carry a Dillo token of its own.
        engine(r"(?i)(dillo)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?", "Dillo"),
        engine(r"(?i)(icab)[/\s]([23])(?:\.(\d+))?(?:\.(\d+))?", "iCab"),
        engine(r"(?i)\blibweb", "LibWeb"),
        engine(r"(?i)(rv:)(\w+)(?:\.(\w+))?(?:\.(\w+))?.+gecko", "Gecko"),
    ]
});

impl EngineRegex {
    #[inline]
    pub(crate) fn built_in_regexes() -> &'static [EngineRegex] {
        &REGEXES
    }

    /// Reads the `engine_parsers` section, which is an extension of this crate rather than a part of the uap-core format.
    pub(crate) fn from_yaml(yaml: &Yaml) -> Result<Vec<EngineRegex>, UserAgentParserError> {
        let yaml_engine_replacement = Yaml::String("engine_replacement".to_string());

        from_yaml_section(yaml, |regex, yaml| {
            Ok(EngineRegex {
                regex,
                engine_replacement: optional_replacement(yaml, &yaml_engine_replacement)?,
            })
        })
    }
}
