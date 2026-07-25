use std::{borrow::Cow, fs, sync::LazyLock};

use user_agent_parser::UserAgentParser;
use yaml_rust::{Yaml, YamlLoader, yaml::Hash};

// Building a parser compiles more than a thousand regexes, so every test shares the same one.
static UA_PARSER: LazyLock<UserAgentParser> =
    LazyLock::new(|| UserAgentParser::from_path("uap-core/regexes.yaml").unwrap());

// The same files uap-core runs its own test suite against, see `uap-core/tests/test.js`.
const PRODUCT_TEST_FILES: [&str; 5] = [
    "uap-core/tests/test_ua.yaml",
    "uap-core/test_resources/firefox_user_agent_strings.yaml",
    "uap-core/test_resources/pgts_browser_list.yaml",
    "uap-core/test_resources/opera_mini_user_agent_strings.yaml",
    "uap-core/test_resources/podcasting_user_agent_strings.yaml",
];

const OS_TEST_FILES: [&str; 2] =
    ["uap-core/tests/test_os.yaml", "uap-core/test_resources/additional_os_tests.yaml"];

const DEVICE_TEST_FILES: [&str; 1] = ["uap-core/tests/test_device.yaml"];

/// Reads the `test_cases` list of one uap-core test file and hands every case to `f`.
fn for_each_test_case(path: &str, mut f: impl FnMut(&Hash)) {
    let yaml = fs::read_to_string(path).unwrap();

    let yamls = YamlLoader::load_from_str(&yaml).unwrap();

    let test_cases = yamls[0]
        .as_hash()
        .unwrap()
        .get(&Yaml::String("test_cases".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();

    for test_case in test_cases {
        f(test_case.as_hash().unwrap());
    }
}

/// Reads one expected field of a test case, which may be missing entirely or be present without a value.
fn field<'a>(test_case: &'a Hash, key: &Yaml) -> Option<Cow<'a, str>> {
    // An empty string is how uap-core writes down a capture group which took part in the match without capturing anything, which this crate reports as no value at all.
    match test_case.get(key)?.as_str()? {
        "" => None,
        text => Some(Cow::from(text)),
    }
}

#[test]
fn test_product() {
    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());
    let yaml_patch_minor = Yaml::String("patch_minor".to_string());

    for path in PRODUCT_TEST_FILES {
        for_each_test_case(path, |test_case| {
            let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

            let name = field(test_case, &yaml_family);
            let major = field(test_case, &yaml_major);
            let minor = field(test_case, &yaml_minor);
            let patch = field(test_case, &yaml_patch);

            let product = UA_PARSER.parse_product(user_agent);

            assert_eq!(name, product.name, "{path}: {user_agent}");
            assert_eq!(major, product.major, "{path}: {user_agent}");
            assert_eq!(minor, product.minor, "{path}: {user_agent}");
            assert_eq!(patch, product.patch, "{path}: {user_agent}");

            // Unlike the OS test files, these only record `patch_minor` on some of their cases, so the others cannot be checked.
            if test_case.contains_key(&yaml_patch_minor) {
                let patch_minor = field(test_case, &yaml_patch_minor);

                assert_eq!(patch_minor, product.patch_minor, "{path}: {user_agent}");
            }
        });
    }
}

#[test]
fn test_os() {
    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());
    let yaml_patch_minor = Yaml::String("patch_minor".to_string());

    for path in OS_TEST_FILES {
        for_each_test_case(path, |test_case| {
            let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

            let name = field(test_case, &yaml_family);
            let major = field(test_case, &yaml_major);
            let minor = field(test_case, &yaml_minor);
            let patch = field(test_case, &yaml_patch);
            let patch_minor = field(test_case, &yaml_patch_minor);

            let os = UA_PARSER.parse_os(user_agent);

            assert_eq!(name, os.name, "{path}: {user_agent}");
            assert_eq!(major, os.major, "{path}: {user_agent}");
            assert_eq!(minor, os.minor, "{path}: {user_agent}");
            assert_eq!(patch, os.patch, "{path}: {user_agent}");
            assert_eq!(patch_minor, os.patch_minor, "{path}: {user_agent}");
        });
    }
}

#[test]
fn test_device() {
    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_brand = Yaml::String("brand".to_string());
    let yaml_model = Yaml::String("model".to_string());

    for path in DEVICE_TEST_FILES {
        for_each_test_case(path, |test_case| {
            let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

            let name = field(test_case, &yaml_family);
            let brand = field(test_case, &yaml_brand);
            let model = field(test_case, &yaml_model);

            let device = UA_PARSER.parse_device(user_agent);

            assert_eq!(name, device.name, "{path}: {user_agent}");
            assert_eq!(brand, device.brand, "{path}: {user_agent}");
            assert_eq!(model, device.model, "{path}: {user_agent}");
        });
    }
}

#[test]
fn test_cpu() {
    let test_cases = [
        ("ia32", "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:19.0) Gecko/20100101 Firefox/19.0"),
        ("ia32", "Mozilla/5.0 (X11; U; FreeBSD i386; en-US; rv:1.7) Gecko/20040628 Epiphany/1.2.6"),
        ("ia32", "QuickTime/7.5.6 (qtver=7.5.6;cpu=IA32;os=Mac 10.5.8)"),
        ("amd64", "Opera/9.80 (X11; Linux x86_64; U; Linux Mint; en) Presto/2.2.15 Version/10.10"),
        (
            "amd64",
            "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.2; Win64; x64; Trident/6.0; \
             .NET4.0E; .NET4.0C)",
        ),
        ("amd64", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; WOW64; Trident/6.0)"),
        (
            "amd64",
            "XBMC/12.0 Git:20130127-fb595f2 (Windows NT 6.1;WOW64;Win64;x64; http://www.xbmc.org)",
        ),
        (
            "arm",
            "Mozilla/5.0 (X11; U; Linux armv61; en-US; rv:1.9.1b2pre) Gecko/20081015 Fennec/1.0a1",
        ),
        (
            "arm",
            "Mozilla/5.0 (X11; CrOS armv7l 9765.85.0) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/61.0.3163.123 Safari/537.36",
        ),
        ("arm", "Opera/9.7 (Windows Mobile; PPC; Opera Mobi/35166; U; en) Presto/2.2.1"),
        ("ppc", "Mozilla/4.0 (compatible; MSIE 4.5; Mac_PowerPC)"),
        ("ppc", "Mozilla/4.0 (compatible; MSIE 5.17; Mac_PowerPC Mac OS; en)"),
        ("ppc", "iCab/2.9.5 (Macintosh; U; PPC; Mac OS X)"),
        (
            "sparc",
            "Mozilla/5.0 (X11; U; SunOS sun4u; en-US; rv:1.9b5) Gecko/2008032620 Firefox/3.0b5",
        ),
        (
            "arm64",
            "Mozilla/5.0 (X11; Linux aarch64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/100.0.0.0 Safari/537.36",
        ),
        ("armhf", "Mozilla/5.0 (X11; Linux armv7hl; rv:100.0) Gecko/20100101 Firefox/100.0"),
        ("arm64", "Mozilla/5.0 (X11; Linux armv9l; rv:130.0) Gecko/20100101 Firefox/130.0"),
        ("arm", "Mozilla/5.0 (Windows NT 6.2; ARM; Trident/7.0; Touch; rv:11.0) like Gecko"),
        (
            "riscv64",
            "Mozilla/5.0 (X11; Linux riscv64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/130.0.0.0 Safari/537.36",
        ),
        (
            "loong64",
            "Mozilla/5.0 (X11; Linux loongarch64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/130.0.0.0 Safari/537.36",
        ),
        ("mips64", "Mozilla/5.0 (X11; U; Linux mips64el; rv:60.0) Gecko/20100101 Firefox/60.0"),
        ("ppc64", "Mozilla/5.0 (X11; U; Linux ppc64le; rv:60.0) Gecko/20100101 Firefox/60.0"),
    ];

    for (answer, user_agent) in test_cases {
        let cpu = UA_PARSER.parse_cpu(user_agent);

        assert_eq!(Some(Cow::from(answer)), cpu.architecture);
    }
}

#[test]
fn test_engine() {
    let test_cases = [
        (
            "Blink",
            Some("57"),
            Some("0"),
            Some("2987"),
            "Mozilla/5.0 (Linux; Android 7.0; SM-G920I Build/NRD90M) AppleWebKit/537.36 (KHTML, \
             like Gecko) OculusBrowser/3.4.9 SamsungBrowser/4.0 Chrome/57.0.2987.146 Mobile VR \
             Safari/537.36",
        ),
        (
            "EdgeHTML",
            Some("12"),
            Some("0"),
            None,
            "Mozilla/5.0 (Windows NT 6.4; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/36.0.1985.143 Safari/537.36 Edge/12.0",
        ),
        (
            "Gecko",
            Some("2"),
            Some("0b9pre"),
            None,
            "Mozilla/5.0 (X11; Linux x86_64; rv:2.0b9pre) Gecko/20110111 Firefox/4.0b9pre",
        ),
        (
            "Goanna",
            Some("2"),
            Some("2"),
            None,
            "Mozilla/5.0 (Windows NT 5.1; rv:38.9) Gecko/20100101 Goanna/2.2 Firefox/38.9 \
             PaleMoon/26.5.0",
        ),
        (
            "KHTML",
            Some("4"),
            Some("5"),
            Some("4"),
            "Mozilla/5.0 (compatible; Konqueror/4.5; FreeBSD) KHTML/4.5.4 (like Gecko)",
        ),
        (
            "NetFront",
            Some("3"),
            Some("0"),
            None,
            "Mozilla/4.0 (PDA; Windows CE/1.0.1) NetFront/3.0",
        ),
        (
            "Presto",
            Some("2"),
            Some("8"),
            Some("149"),
            "Opera/9.80 (Windows NT 6.1; Opera Tablet/15165; U; en) Presto/2.8.149 Version/11.1",
        ),
        (
            "Tasman",
            Some("1"),
            Some("0"),
            None,
            "Mozilla/4.0 (compatible; MSIE 6.0; PPC Mac OS X 10.4.7; Tasman 1.0)",
        ),
        (
            "Trident",
            Some("6"),
            Some("0"),
            None,
            "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2; Win64; x64; Trident/6.0)",
        ),
        (
            "WebKit",
            Some("533"),
            Some("19"),
            Some("4"),
            "Mozilla/5.0 (Windows; U; Windows NT 6.1; sv-SE) AppleWebKit/533.19.4 (KHTML, like \
             Gecko) Version/5.0.3 Safari/533.19.4",
        ),
        (
            "WebKit",
            Some("537"),
            Some("36"),
            None,
            "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML like Gecko) \
             Chrome/27.0.1453.110 Safari/537.36",
        ),
        (
            "Servo",
            Some("1"),
            Some("0"),
            None,
            "Mozilla/5.0 (X11; Linux x86_64; rv:1.0) Servo/1.0 Firefox/108.0",
        ),
        (
            "LibWeb",
            None,
            None,
            None,
            "Mozilla/5.0 (SerenityOS; x86_64) LibWeb+LibJS/1.0 Browser/1.0",
        ),
        ("ArkWeb", Some("2"), Some("0"), Some("1"), "Mozilla/5.0 (Linux; ArkWeb/2.0.1) like Gecko"),
    ];

    for (answer, major, minor, patch, user_agent) in test_cases {
        let engine = UA_PARSER.parse_engine(user_agent);

        assert_eq!(Some(Cow::from(answer)), engine.name);
        assert_eq!(major.map(Cow::from), engine.major);
        assert_eq!(minor.map(Cow::from), engine.minor);
        assert_eq!(patch.map(Cow::from), engine.patch);
    }
}

#[test]
fn test_version() {
    // The fourth part of the first product version is the one https://github.com/magiclen/user-agent-parser/issues/2 could not reach.
    let test_cases = [
        (
            Some("79.0.3945.79"),
            None,
            Some("79.0.3945"),
            "Mozilla/5.0 (Web0S; Linux/SmartTV) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/79.0.3945.79 Safari/537.36",
        ),
        (
            Some("17.6"),
            Some("10.15.7"),
            Some("605.1.15"),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like \
             Gecko) Version/17.6 Safari/605.1.15",
        ),
        (None, None, None, "there-is-no-user-agent-here"),
    ];

    for (product, os, engine, user_agent) in test_cases {
        assert_eq!(product.map(Cow::from), UA_PARSER.parse_product(user_agent).version());
        assert_eq!(os.map(Cow::from), UA_PARSER.parse_os(user_agent).version());
        assert_eq!(engine.map(Cow::from), UA_PARSER.parse_engine(user_agent).version());
    }
}

#[test]
fn test_custom_cpu_and_engine_parsers() {
    // Neither section is a part of the uap-core format, so both of them are checked against a source written just for this test.
    const CUSTOM_PARSERS: &str = r"
cpu_parsers:
  - regex: '\bmips64(?:el)?\b'
    architecture_replacement: 'my-mips'
  - regex: '\b(riscv\d*)\b'

engine_parsers:
  - regex: '(myengine)/(\w+)(?:\.(\w+))?(?:\.(\w+))?'
    regex_flag: 'i'
    engine_replacement: 'MyEngine'
";

    const CPU_ONLY_PARSERS: &str = r"
cpu_parsers:
  - regex: '\bmips64(?:el)?\b'
    architecture_replacement: 'my-mips'
";

    const MIPS_USER_AGENT: &str =
        "Mozilla/5.0 (X11; U; Linux mips64el; rv:60.0) Gecko/20100101 Firefox/60.0";

    let ua_parser = UserAgentParser::from_str(CUSTOM_PARSERS).unwrap();

    // A section replaces the built-in patterns as a whole, so this no longer reports the built-in `mips64`.
    assert_eq!(Some(Cow::from("my-mips")), ua_parser.parse_cpu(MIPS_USER_AGENT).architecture);
    assert_eq!(None, ua_parser.parse_engine(MIPS_USER_AGENT).name);

    // A pattern without a replacement takes its architecture from the first capture group.
    let cpu = ua_parser.parse_cpu("Mozilla/5.0 (X11; Linux riscv64) AppleWebKit/537.36");

    assert_eq!(Some(Cow::from("riscv64")), cpu.architecture);

    // The replacement wins over the first capture group, which `regex_flag` let match in another case.
    let engine = ua_parser.parse_engine("Mozilla/5.0 (X11; Linux x86_64) MyEngine/2.0.1");

    assert_eq!(Some(Cow::from("MyEngine")), engine.name);
    assert_eq!(Some(Cow::from("2")), engine.major);
    assert_eq!(Some(Cow::from("0")), engine.minor);
    assert_eq!(Some(Cow::from("1")), engine.patch);

    // A source which writes down only one of the two sections keeps the built-in patterns of the other one.
    let cpu_only_parser = UserAgentParser::from_str(CPU_ONLY_PARSERS).unwrap();

    assert_eq!(Some(Cow::from("my-mips")), cpu_only_parser.parse_cpu(MIPS_USER_AGENT).architecture);
    assert_eq!(Some(Cow::from("Gecko")), cpu_only_parser.parse_engine(MIPS_USER_AGENT).name);
}
