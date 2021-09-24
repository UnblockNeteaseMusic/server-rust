use crate::logger::LevelFilter;
use crate::providers::Provider;
use regex::Regex;
use std::path::PathBuf;
pub use structopt::StructOpt;

fn parse_bool(src: &str) -> Result<bool, &str> {
    return if src == "0" || src == "false" {
        Ok(false)
    } else if src == "1" || src == "true" {
        Ok(true)
    } else {
        Err("provided string was not `true`, `false`, `0` or `1`")
    };
}

#[derive(StructOpt, PartialEq, Clone, Debug)]
/// The environment of the CLI of UNM (Rust)
pub struct OptEnv {
    /// 激活无损音质获取
    #[structopt(long, env = "ENABLE_FLAC", parse(try_from_str = parse_bool))]
    pub enable_flac: Option<bool>,

    /// 启用本地黑胶 VIP
    #[structopt(long, env = "ENABLE_LOCAL_VIP", parse(try_from_str = parse_bool))]
    pub enable_local_vip: Option<bool>,

    /// 激活故障的 Netease HTTPDNS 查询（不建议）
    #[structopt(long, env = "ENABLE_HTTPDNS", parse(try_from_str = parse_bool))]
    pub enable_httpdns: Option<bool>,

    /// 激活开发模式。
    #[structopt(long, env = "DEVELOPMENT", parse(try_from_str = parse_bool))]
    pub development: Option<bool>,

    /// 输出机器可读的 JSON 记录格式
    #[structopt(long, env = "JSON_LOG", parse(try_from_str = parse_bool))]
    pub json_log: Option<bool>,

    /// 停用 cache
    #[structopt(long, env = "NO_CACHE", parse(try_from_str = parse_bool))]
    pub no_cache: Option<bool>,

    /// 允许的最低源音质，小于该值将被替换
    #[structopt(long, env = "MIN_BR", default_value = "0")]
    pub min_br: i32,

    /// 日志输出等级。请见〈日志等级〉部分。
    #[structopt(long, env = "LOG_LEVEL", default_value = "debug")]
    pub log_level: LevelFilter,

    /// 日志输出的文件位置
    #[structopt(long, env = "LOG_FILE")]
    pub log_file: Option<PathBuf>,

    /// JOOX 音源的 wmid 和 session_key cookie "wmid=<your_wmid>; session_key=<your_session_key>"
    #[structopt(long, env = "JOOX_COOKIE")]
    pub joox_cookie: Option<String>,

    /// 咪咕音源的 aversionid cookie "<your_aversionid>"
    #[structopt(long, env = "MIGU_COOKIE")]
    pub migu_cookie: Option<String>,

    /// QQ 音源的 uin 和 qm_keyst cookie "uin=<your_uin>; qm_keyst=<your_qm_keyst>"
    #[structopt(long, env = "QQ_COOKIE")]
    pub qq_cookie: Option<String>,

    /// Youtube 音源的 Data API v3 Key "<your_data_api_key>"
    #[structopt(long, env = "YOUTUBE_KEY")]
    pub youtube_key: Option<String>,

    /// 自定义证书文件
    #[structopt(long, env = "SIGN_CERT", default_value = "./ca.crt")]
    pub sign_cert: PathBuf,

    /// 自定义密钥文件
    #[structopt(long, env = "SIGN_KEY", default_value = "./server.key")]
    pub sign_key: PathBuf,
}

/// The options of the CLI of UNM (Rust)
#[derive(StructOpt, Debug)]
#[structopt(
    name = "unm-server-rust",
    about = "The server of UnblockNeteaseMusic written in Rust"
)]
pub struct Opt {
    /// Specify the server port of UNM.
    #[structopt(
        short,
        long,
        default_value = "8080:8081",
        use_delimiter = true,
        value_delimiter = ":"
    )]
    pub port: Vec<u16>,

    /// Specify the server host of UNM.
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: String,

    /// Request through the upstream proxy. (Not implemented)
    #[structopt(short = "u", long)]
    pub proxy_url: Option<String>,

    /// Force the Netease server ip. (Not implemented)
    #[structopt(short = "f", long = "force-host")]
    pub force_netease_host: Option<String>,

    /// Set the priority of sources.
    #[structopt(
        short = "o",
        long,
        default_value = "kugou kuwo migu bilibili",
        use_delimiter = true,
        value_delimiter = " "
    )]
    pub source: Vec<Provider>,

    /// Replace the virtual endpoint with public host. (Not implemented)
    #[structopt(short, long)]
    pub endpoint: Option<String>,

    /// Enable the proxy limitation. (Not implemented)
    #[structopt(short, long, parse(try_from_str=parse_bool))]
    pub strict: Option<bool>,

    #[structopt(short, long)]
    /// set up proxy authentication
    pub token: Option<String>,

    #[structopt(flatten)]
    pub env: OptEnv,
}

impl Opt {
    pub fn is_valid(&self) -> Option<String> {
        let mut rst = self.proxy_url.as_ref().and_then(|url| {
            let proxy_url_re: Regex =
                Regex::new(r"^http(s?)://.+:\d+$").expect("wrong regex of proxy url");
            match proxy_url_re.is_match(&url) {
                true => None,
                false => Some(String::from("lease check the proxy url.")),
            }
        });
        if rst.is_some() {
            return rst;
        }

        rst = self.endpoint.as_ref().and_then(|url| {
            let re = Regex::new(r"^http(s?)://.+$").expect("wrong regex of endpoint");
            match re.is_match(&url) {
                true => None,
                false => Some(String::from("Please check the endpoint host.")),
            }
        });
        if rst.is_some() {
            return rst;
        }

        rst = self
            .host
            .parse::<std::net::IpAddr>()
            .err()
            .map(|_| String::from("Please check the server host."));
        if rst.is_some() {
            return rst;
        }

        rst = self.token.as_ref().and_then(|t| {
            let re = Regex::new(r"^\S+:\S+$").expect("wrong regex of token");
            match re.is_match(&t) {
                true => None,
                false => Some(String::from("Please check the authentication token.")),
            }
        });
        if rst.is_some() {
            return rst;
        }

        let len = self.source.len();
        for i1 in 0..len {
            for i2 in i1 + 1..len {
                if self.source[i1] == self.source[i2] {
                    return Some(String::from(format!(
                        "Please check the duplication item({:#?}) in match order.",
                        self.source[i1]
                    )));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_default_opt() -> Opt {
        let args: Vec<std::ffi::OsString> = Vec::new();
        return Opt::from_iter(args);
    }

    #[test]
    fn default_is_valid() {
        let op = new_default_opt();
        // println!("{:#?}", op);
        assert_eq!(op.is_valid(), None);
    }
    #[test]
    fn token_check() {
        let mut op = new_default_opt();
        op.token = Some(String::from("abcd:123"));
        assert!(op.is_valid().is_none());
        op.token = Some(String::from("abcd123"));
        assert!(op.is_valid().is_some());
        op.token = Some(String::from("ab cd:123"));
        assert!(op.is_valid().is_some());
    }
    #[test]
    fn dump_source_is_invalid() {
        let mut op = new_default_opt();
        op.source.resize(2, Provider::Bilibili);
        op.source[0] = Provider::Bilibili;
        op.source[1] = Provider::Bilibili;
        assert_eq!(
            op.is_valid(),
            Some(String::from(
                "Please check the duplication item(bilibili) in match order."
            ))
        );
    }
}
