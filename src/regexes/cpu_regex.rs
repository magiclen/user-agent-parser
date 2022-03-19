use onig::Regex;

#[derive(Debug)]
pub struct CPURegex {
    pub(crate) regex: Regex,
    pub(crate) architecture_replacement: Option<String>,
}

impl CPURegex {
    pub fn built_in_regexes() -> Vec<CPURegex> {
        vec![
            {
                let regex = Regex::new(r"(?i)(?:(amd|x(?:(?:86|64)[_-])?|wow|win)64)[;)]").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("amd64".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)(ia32(?=;))").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("ia32".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)((?:i[346]|x)86)[;)]").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("ia32".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)windows\s(ce|mobile);\sppc;").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("arm".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)((?:ppc|powerpc)(?:64)?)(?:\smac|;|\))").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("ppc".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(sun4\w)[;)]").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("sparc".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)(?:ia64;)").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("ia64".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)(?:68k\))").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("68k".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)arm(?:64|(?=v\d+[;l]))").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("arm".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)(?=atmel\s)avr").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("avr".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)irix(?:64)?").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("irix".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)mips(?:64)?").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("mips".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)sparc(?:64)?").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("sparc".to_string()),
                }
            },
            {
                let regex = Regex::new(r"(?i)pa-risc").unwrap();

                CPURegex {
                    regex,
                    architecture_replacement: Some("pa-risc".to_string()),
                }
            },
        ]
    }
}
