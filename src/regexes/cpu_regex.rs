use regex::Regex;

#[derive(Debug)]
pub struct CPURegex {
    pub(crate) regex:                    Regex,
    pub(crate) architecture_replacement: Option<String>,
}

impl CPURegex {
    pub fn built_in_regexes() -> Vec<CPURegex> {
        #[inline]
        fn cpu(pattern: &str, architecture: &str) -> CPURegex {
            CPURegex {
                regex:                    Regex::new(pattern).unwrap(),
                architecture_replacement: Some(architecture.to_string()),
            }
        }

        vec![
            cpu(r"(?i)(?:amd|x(?:(?:86|64)[_-])?|wow|win)64[;)]", "amd64"),
            cpu(r"(?i)ia32[;)]", "ia32"),
            cpu(r"(?i)(?:i[346]|x)86[;)]", "ia32"),
            cpu(r"(?i)\b(?:aarch64|arm(?:v?8e?l?|_?64))\b", "arm64"),
            cpu(r"(?i)\barm(?:v[67])?ht?n?[fl]p?\b", "armhf"),
            // PocketPC is mistakenly reported as PPC, but it is actually ARM.
            cpu(r"(?i)windows\s(?:ce|mobile);\sppc;", "arm"),
            cpu(r"(?i)(?:ppc|powerpc)(?:64)?(?:\smac|;|\))", "ppc"),
            cpu(r"(?i)sun4\w[;)]", "sparc"),
            cpu(r"(?i)ia64[;)]", "ia64"),
            cpu(r"(?i)68k\)", "68k"),
            cpu(r"(?i)arm(?:64|v\d+[;l])", "arm"),
            cpu(r"(?i)avr32\b", "avr32"),
            cpu(r"(?i)atmel\s+avr", "avr"),
            cpu(r"(?i)irix(?:64)?", "irix"),
            cpu(r"(?i)mips(?:64)?", "mips"),
            cpu(r"(?i)sparc(?:64)?", "sparc"),
            cpu(r"(?i)pa-risc", "pa-risc"),
        ]
    }
}
