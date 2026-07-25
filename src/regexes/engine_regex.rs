use regex::Regex;

#[derive(Debug)]
pub struct EngineRegex {
    pub(crate) regex:                 Regex,
    pub(crate) name_replacement:      Option<String>,
    pub(crate) engine_v1_replacement: Option<String>,
    pub(crate) engine_v2_replacement: Option<String>,
    pub(crate) engine_v3_replacement: Option<String>,
}

impl EngineRegex {
    pub fn built_in_regexes() -> Vec<EngineRegex> {
        #[inline]
        fn engine(pattern: &str, name: &str) -> EngineRegex {
            EngineRegex {
                regex:                 Regex::new(pattern).unwrap(),
                name_replacement:      Some(name.to_string()),
                engine_v1_replacement: None,
                engine_v2_replacement: None,
                engine_v3_replacement: None,
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
            engine(r"(?i)(icab)[/\s]([23])(?:\.(\d+))?(?:\.(\d+))?", "iCab"),
            engine(r"(?i)\blibweb", "LibWeb"),
            engine(r"(?i)(rv:)(\w+)(?:\.(\w+))?(?:\.(\w+))?.+gecko", "Gecko"),
        ]
    }
}
