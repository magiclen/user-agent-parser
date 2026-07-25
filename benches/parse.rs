use std::sync::LazyLock;

use bencher::{Bencher, benchmark_group, benchmark_main, black_box};
use user_agent_parser::UserAgentParser;

const REGEXES_PATH: &str = "uap-core/regexes.yaml";

// A spread of desktop, mobile, bot and legacy user agents, so that both early-matching and late-matching regexes are covered.
const USER_AGENTS: [&str; 8] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) \
     Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) \
     Version/17.6 Safari/605.1.15",
    "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like \
     Gecko) Version/17.6 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Linux; Android 14; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) \
     Chrome/131.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
    "Mozilla/5.0 (X11; U; SunOS sun4u; en-US; rv:1.9b5) Gecko/2008032620 Firefox/3.0b5",
    "Mozilla/5.0 (X11; Linux x86_64; rv:10.0) Gecko/20100101 Firefox/10.0 \
     [FBAN/FBIOS;FBAV/8.0.0.28.18;FBBV/1665515;FBDV/iPhone4,1;FBMD/iPhone;FBSN/iPhone \
     OS;FBSV/7.0.4;FBSS/2; FBCR/Telekom.de;FBID/phone;FBLC/de_DE;FBOP/5]",
];

// `bencher` re-runs the whole benchmark function once per sample, so a parser built inside one would be thrown away every time.
// Sharing it keeps the lazy DFA caches of the regexes warm, which is what a long-running process actually sees.
static UA_PARSER: LazyLock<UserAgentParser> =
    LazyLock::new(|| UserAgentParser::from_path(REGEXES_PATH).unwrap());

fn from_path(bencher: &mut Bencher) {
    bencher.iter(|| black_box(UserAgentParser::from_path(REGEXES_PATH).unwrap()));
}

fn parse_product(bencher: &mut Bencher) {
    bencher.iter(|| {
        for user_agent in USER_AGENTS {
            black_box(UA_PARSER.parse_product(black_box(user_agent)));
        }
    });
}

fn parse_os(bencher: &mut Bencher) {
    bencher.iter(|| {
        for user_agent in USER_AGENTS {
            black_box(UA_PARSER.parse_os(black_box(user_agent)));
        }
    });
}

fn parse_device(bencher: &mut Bencher) {
    bencher.iter(|| {
        for user_agent in USER_AGENTS {
            black_box(UA_PARSER.parse_device(black_box(user_agent)));
        }
    });
}

fn parse_cpu(bencher: &mut Bencher) {
    bencher.iter(|| {
        for user_agent in USER_AGENTS {
            black_box(UA_PARSER.parse_cpu(black_box(user_agent)));
        }
    });
}

fn parse_engine(bencher: &mut Bencher) {
    bencher.iter(|| {
        for user_agent in USER_AGENTS {
            black_box(UA_PARSER.parse_engine(black_box(user_agent)));
        }
    });
}

benchmark_group!(
    benches,
    from_path,
    parse_product,
    parse_os,
    parse_device,
    parse_cpu,
    parse_engine
);
benchmark_main!(benches);
