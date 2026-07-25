use std::sync::LazyLock;

use regex::Regex;
use yaml_rust::Yaml;

use crate::{
    UserAgentParserError,
    regexes::{Replacement, build_regex, from_yaml_section, optional_replacement},
};

#[derive(Debug, Clone)]
pub(crate) struct CPURegex {
    pub(crate) regex:                    Regex,
    pub(crate) architecture_replacement: Option<Replacement>,
}

/// The built-in patterns never change, so they are compiled once instead of once per `UserAgentParser`.
static REGEXES: LazyLock<Vec<CPURegex>> = LazyLock::new(|| {
    #[inline]
    fn cpu(pattern: &str, architecture: &str) -> CPURegex {
        CPURegex {
            regex:                    build_regex(pattern, false).unwrap(),
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
        // `alpha/06` is a browser of its own, so the token only counts when one of the systems which ran on Alpha says it.
        // Windows NT spells the architecture as a trailing `[AXP]` token instead of naming it.
        cpu(
            r"(?i)(?:\b(?:osf1|(?:free|open|net|dragonfly)bsd)(?:\s+v[\d.]+)?\s+alpha\b|winnt.+\[axp)",
            "alpha",
        ),
        cpu(r"(?i)ia64[;)]", "ia64"),
        // `MC680x0` is how AmigaOS spells the same family, with a literal `x` standing for the model digit.
        cpu(r"(?i)(?:68k\)|mc680[\dx]0)", "68k"),
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
});

impl CPURegex {
    #[inline]
    pub(crate) fn built_in_regexes() -> &'static [CPURegex] {
        &REGEXES
    }

    /// Reads the `cpu_parsers` section, which is an extension of this crate rather than a part of the uap-core format.
    pub(crate) fn from_yaml(yaml: &Yaml) -> Result<Vec<CPURegex>, UserAgentParserError> {
        let yaml_architecture_replacement = Yaml::String("architecture_replacement".to_string());

        from_yaml_section(yaml, |regex, yaml| {
            Ok(CPURegex {
                regex,
                architecture_replacement: optional_replacement(
                    yaml,
                    &yaml_architecture_replacement,
                )?,
            })
        })
    }
}
