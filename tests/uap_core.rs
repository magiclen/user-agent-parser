extern crate user_agent_parser;
extern crate yaml_rust;

use std::fs;
use std::borrow::Cow;

use user_agent_parser::UserAgentParser;

use yaml_rust::{YamlLoader, Yaml};

#[test]
fn test_product() {
    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    let yaml = fs::read_to_string("uap-core/tests/test_ua.yaml").unwrap();

    let yamls = YamlLoader::load_from_str(&yaml).unwrap();
    let yaml = &yamls[0];

    let test_cases = yaml.as_hash().unwrap().get(&Yaml::String("test_cases".to_string())).unwrap().as_vec().unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(|s| Cow::from(s));
        let major = test_case.get(&yaml_major).unwrap().as_str().map(|s| Cow::from(s));
        let minor = test_case.get(&yaml_minor).unwrap().as_str().map(|s| Cow::from(s));
        let patch = test_case.get(&yaml_patch).unwrap().as_str().map(|s| Cow::from(s));

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

    let test_cases = yaml.as_hash().unwrap().get(&Yaml::String("test_cases".to_string())).unwrap().as_vec().unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_major = Yaml::String("major".to_string());
    let yaml_minor = Yaml::String("minor".to_string());
    let yaml_patch = Yaml::String("patch".to_string());
    let yaml_patch_minor = Yaml::String("patch_minor".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(|s| Cow::from(s));
        let major = test_case.get(&yaml_major).unwrap().as_str().map(|s| Cow::from(s));
        let minor = test_case.get(&yaml_minor).unwrap().as_str().map(|s| Cow::from(s));
        let patch = test_case.get(&yaml_patch).unwrap().as_str().map(|s| Cow::from(s));
        let patch_minor = test_case.get(&yaml_patch_minor).unwrap().as_str().map(|s| Cow::from(s));

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

    let test_cases = yaml.as_hash().unwrap().get(&Yaml::String("test_cases".to_string())).unwrap().as_vec().unwrap();

    let yaml_user_agent_string = Yaml::String("user_agent_string".to_string());
    let yaml_family = Yaml::String("family".to_string());
    let yaml_brand = Yaml::String("brand".to_string());
    let yaml_model = Yaml::String("model".to_string());

    for test_case in test_cases {
        let test_case = test_case.as_hash().unwrap();

        let user_agent = test_case.get(&yaml_user_agent_string).unwrap().as_str().unwrap();

        let name = test_case.get(&yaml_family).unwrap().as_str().map(|s| Cow::from(s));
        let brand = test_case.get(&yaml_brand).unwrap().as_str().map(|s| Cow::from(s));
        let model = test_case.get(&yaml_model).unwrap().as_str().map(|s| Cow::from(s));

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
        ("amd64", "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.2; Win64; x64; Trident/6.0; .NET4.0E; .NET4.0C)"),
        ("amd64", "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.1; WOW64; Trident/6.0)"),
        ("amd64", "XBMC/12.0 Git:20130127-fb595f2 (Windows NT 6.1;WOW64;Win64;x64; http://www.xbmc.org)"),
        ("arm", "Mozilla/5.0 (X11; U; Linux armv61; en-US; rv:1.9.1b2pre) Gecko/20081015 Fennec/1.0a1"),
        ("arm", "Mozilla/5.0 (X11; CrOS armv7l 9765.85.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/61.0.3163.123 Safari/537.36"),
        ("arm", "Opera/9.7 (Windows Mobile; PPC; Opera Mobi/35166; U; en) Presto/2.2.1"),
        ("ppc", "Mozilla/4.0 (compatible; MSIE 4.5; Mac_PowerPC)"),
        ("ppc", "Mozilla/4.0 (compatible; MSIE 5.17; Mac_PowerPC Mac OS; en)"),
        ("ppc", "iCab/2.9.5 (Macintosh; U; PPC; Mac OS X)"),
        ("sparc", "Mozilla/5.0 (X11; U; SunOS sun4u; en-US; rv:1.9b5) Gecko/2008032620 Firefox/3.0b5"),
    ];

    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    for (answer, user_agent) in test_cases.iter() {
        let cpu = ua_parser.parse_cpu(user_agent);

        assert_eq!(Some(Cow::from(*answer)), cpu.architecture);
    }
}