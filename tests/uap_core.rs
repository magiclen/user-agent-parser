use std::{borrow::Cow, fs};

use user_agent_parser::UserAgentParser;
use yaml_rust2::{Yaml, YamlLoader};

#[test]
fn test_product() {
    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    let yaml = fs::read_to_string("uap-core/tests/test_ua.yaml").unwrap();

    let yamls = YamlLoader::load_from_str(&yaml).unwrap();
    let yaml = &yamls[0];

    let test_cases = yaml
        .as_hash()
        .unwrap()
        .get(&Yaml::String("test_cases".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(Cow::from);
        let major = test_case.get(&yaml_major).unwrap().as_str().map(Cow::from);
        let minor = test_case.get(&yaml_minor).unwrap().as_str().map(Cow::from);
        let patch = test_case.get(&yaml_patch).unwrap().as_str().map(Cow::from);

        let product = ua_parser.parse_product(user_agent);

        assert_eq!(name, product.name);
        assert_eq!(major, product.major);
        assert_eq!(minor, product.minor);
        assert_eq!(patch, product.patch);
    }
}

#[test]
fn test_os() {
    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    let yaml = fs::read_to_string("uap-core/tests/test_os.yaml").unwrap();

    let yamls = YamlLoader::load_from_str(&yaml).unwrap();
    let yaml = &yamls[0];

    let test_cases = yaml
        .as_hash()
        .unwrap()
        .get(&Yaml::String("test_cases".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());
    let yaml_patch_minor = Yaml::String("patch_minor".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(Cow::from);
        let major = test_case.get(&yaml_major).unwrap().as_str().map(Cow::from);
        let minor = test_case.get(&yaml_minor).unwrap().as_str().map(Cow::from);
        let patch = test_case.get(&yaml_patch).unwrap().as_str().map(Cow::from);
        let patch_minor = test_case.get(&yaml_patch_minor).unwrap().as_str().map(Cow::from);

        let os = ua_parser.parse_os(user_agent);

        assert_eq!(name, os.name);
        assert_eq!(major, os.major);
        assert_eq!(minor, os.minor);
        assert_eq!(patch, os.patch);
        assert_eq!(patch_minor, os.patch_minor);
    }
}

#[test]
fn test_device() {
    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    let yaml = fs::read_to_string("uap-core/tests/test_device.yaml").unwrap();

    let yamls = YamlLoader::load_from_str(&yaml).unwrap();
    let yaml = &yamls[0];

    let test_cases = yaml
        .as_hash()
        .unwrap()
        .get(&Yaml::String("test_cases".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_brand = Yaml::String("brand".to_string());
    let yaml_model = Yaml::String("model".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(Cow::from);
        let brand = test_case.get(&yaml_brand).unwrap().as_str().map(Cow::from);
        let model = test_case.get(&yaml_model).unwrap().as_str().map(Cow::from);

        let device = ua_parser.parse_device(user_agent);

        assert_eq!(name, device.name);
        assert_eq!(brand, device.brand);
        assert_eq!(model, device.model);
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
    ];

    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    for (answer, user_agent) in test_cases.iter() {
        let cpu = ua_parser.parse_cpu(user_agent);

        assert_eq!(Some(Cow::from(*answer)), cpu.architecture);
    }
}

#[test]
fn test_engine() {
    let test_cases = [
        (
            "Blink",
            None,
            None,
            None,
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
    ];

    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    for (answer, major, minor, patch, user_agent) in test_cases.iter() {
        let engine = ua_parser.parse_engine(user_agent);

        assert_eq!(Some(Cow::from(*answer)), engine.name);
        assert_eq!(major.map(Cow::from), engine.major);
        assert_eq!(minor.map(Cow::from), engine.minor);
        assert_eq!(patch.map(Cow::from), engine.patch);
    }
}
