use onig::Regex;

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
        vec![
            {
                let regex =
                    Regex::new(r"(?i)(windows.+\sedge)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("EdgeHTML".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)webkit/537\.36.+chrome/(?!27)").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Blink".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(presto)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Presto".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(webkit)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("WebKit".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(trident)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Trident".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(netfront)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("NetFront".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(netsurf)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("NetSurf".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(amaya)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Amaya".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(lynx)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Lynx".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(w3m)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("w3m".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(goanna)/(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Goanna".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex =
                    Regex::new(r"(?i)(khtml)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("KHTML".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex =
                    Regex::new(r"(?i)(tasman)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Tasman".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex =
                    Regex::new(r"(?i)(links)[/\s]\(?(\w+)(?:\.(\w+))?(?:\.(\w+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Links".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex = Regex::new(r"(?i)(icab)[/\s]([23])(?:\.(\d+))?(?:\.(\d+))?").unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("iCab".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
            {
                let regex =
                    Regex::new(r"(?i)(rv:)(\w+)(?:\.(\w+))?(?:\.(\w+))?(?:(?=\.)\w+)*.+gecko")
                        .unwrap();

                EngineRegex {
                    regex,
                    name_replacement: Some("Gecko".to_string()),
                    engine_v1_replacement: None,
                    engine_v2_replacement: None,
                    engine_v3_replacement: None,
                }
            },
        ]
    }
}
