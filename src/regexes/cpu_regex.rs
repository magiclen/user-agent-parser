use regex::Regex;

use crate::regexes::Replacement;

#[derive(Debug)]
pub struct CPURegex {
    pub(crate) regex:                    Regex,
    pub(crate) architecture_replacement: Option<Replacement>,
}

impl CPURegex {
    pub fn built_in_regexes() -> Vec<CPURegex> {
        #[inline]
        fn cpu(pattern: &str, architecture: &'static str) -> CPURegex {
            CPURegex {
                regex:                    Regex::new(pattern).unwrap(),
                architecture_replacement: Some(Replacement::new(architecture)),
            }
        }

        // The first matching pattern wins, so every 64-bit pattern must be placed before its 32-bit counterpart.
        vec![
            cpu(r"(?i)(?:amd|x(?:(?:86|64)[_-])?|wow|win)64[;)]", "amd64"),
            cpu(r"(?i)ia32[;)]", "ia32"),
            cpu(r"(?i)(?:i[346]|x)86[;)]", "ia32"),
            cpu(r"(?i)\b(?:aarch64|arm(?:v?[89]e?l?|_?64))\b", "arm64"),
            cpu(r"(?i)\barm(?:v[67])?ht?n?[fl]p?\b", "armhf"),
            // PocketPC is mistakenly reported as PPC, but it is actually ARM.
            cpu(r"(?i)windows\s(?:ce|mobile);\sppc;", "arm"),
            // Little-endian variants such as `ppc64le` are reported as plain `ppc64`.
            cpu(r"(?i)(?:ppc|powerpc)64(?:le|el)?(?:\smac|;|\))", "ppc64"),
            cpu(r"(?i)(?:ppc|powerpc)(?:\smac|;|\))", "ppc"),
            cpu(r"(?i)sun4\w[;)]", "sparc"),
            cpu(r"(?i)ia64[;)]", "ia64"),
            cpu(r"(?i)68k\)", "68k"),
            // The leading `\b` keeps words such as `alarm` and `firearm` from being read as ARM.
            cpu(r"(?i)\b(?:aarch32|armeabi(?:[-_]v\d+)?)\b", "arm"),
            cpu(r"(?i)\barm(?:64|v\d+)?[;)l]", "arm"),
            cpu(r"(?i)avr32\b", "avr32"),
            cpu(r"(?i)atmel\s+avr", "avr"),
            cpu(r"(?i)\briscv64\b", "riscv64"),
            cpu(r"(?i)\briscv(?:32)?\b", "riscv"),
            cpu(r"(?i)\b(?:loongarch64|loong64)\b", "loong64"),
            cpu(r"(?i)\bs390x\b", "s390x"),
            cpu(r"(?i)\be2k\b", "e2k"),
            cpu(r"(?i)irix64\b", "irix64"),
            cpu(r"(?i)irix\b", "irix"),
            cpu(r"(?i)mips64(?:el)?\b", "mips64"),
            cpu(r"(?i)mips\b", "mips"),
            cpu(r"(?i)sparc64\b", "sparc64"),
            cpu(r"(?i)sparc\b", "sparc"),
            cpu(r"(?i)pa-risc", "pa-risc"),
        ]
    }
}
