extern crate user_agent_parser;

use user_agent_parser::UserAgentParser;

fn main(){
    let ua_parser = UserAgentParser::from_path("uap-core/regexes.yaml").unwrap();

    let user_agent = "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0 [FBAN/FBIOS;FBAV/8.0.0.28.18;FBBV/1665515;FBDV/iPhone4,1;FBMD/iPhone;FBSN/iPhone OS;FBSV/7.0.4;FBSS/2; FBCR/Telekom.de;FBID/phone;FBLC/de_DE;FBOP/5]";

    let product = ua_parser.parse_product(user_agent);

    println!("{:#?}", product);

//    Product {
//        name: Some(
//            "Facebook",
//        ),
//        major: Some(
//            "8",
//        ),
//        minor: Some(
//            "0",
//        ),
//        patch: Some(
//            "0",
//        ),
//    }

    let os = ua_parser.parse_os(user_agent);

    println!("{:#?}", os);

//    OS {
//        name: Some(
//            "iOS",
//        ),
//        major: None,
//        minor: None,
//        patch: None,
//        patch_minor: None,
//    }

    let device = ua_parser.parse_device(user_agent);

    println!("{:#?}", device);

//    Device {
//        name: Some(
//            "iPhone",
//        ),
//        brand: Some(
//            "Apple",
//        ),
//        model: Some(
//            "iPhone4,1",
//        ),
//    }

    let cpu = ua_parser.parse_cpu(user_agent);

    println!("{:#?}", cpu);

//    CPU {
//        architecture: Some(
//            "amd64",
//        ),
//    }

    let engine = ua_parser.parse_engine(user_agent);

    println!("{:#?}", engine);

//    Engine {
//        name: Some(
//            "Gecko",
//        ),
//        major: Some(
//            "10",
//        ),
//        minor: Some(
//            "0",
//        ),
//        patch: None,
//    }
}