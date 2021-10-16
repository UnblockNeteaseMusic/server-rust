use std::path::PathBuf;

use regex::Regex;
pub use structopt::StructOpt;

use crate::logger::LevelFilter;
use crate::providers::identifiers::Provider;
use crate::Error;

fn parse_bool(src: &str) -> Result<bool, &str> {
    if src == "0" || src == "false" {
        Ok(false)
    } else if src == "1" || src == "true" {
        Ok(true)
    } else {
        Err("provided string was not `true`, `false`, `0` or `1`")
    }
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

type CheckerReturnType<'a> = Result<(), String>;

struct Checker {}

impl Checker {
    pub fn proxy_url(proxy_url: &str) -> CheckerReturnType {
        let proxy_url_re: Regex =
            Regex::new(r"^http(s?)://.+:\d+$").expect("wrong regex of proxy url");
        match proxy_url_re.is_match(proxy_url) {
            true => Ok(()),
            false => Err("Please check the proxy url.".to_string()),
        }
    }

    pub fn host(host: &str) -> CheckerReturnType {
        match host.parse::<std::net::IpAddr>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Please check the server host.".to_string()),
        }
    }

    pub fn token(token: &str) -> CheckerReturnType {
        let re = Regex::new(r"^\S+:\S+$").expect("wrong regex of token");
        match re.is_match(token) {
            true => Ok(()),
            false => Err("Please check the authentication token.".to_string()),
        }
    }

    pub fn source(sources: &[Provider]) -> CheckerReturnType {
        let len = sources.len();
        for i1 in 0..len {
            for i2 in i1 + 1..len {
                if sources[i1] == sources[i2] {
                    return Err(format!(
                        "Please check the duplication item({:#?}) in match order.",
                        sources[i1]
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Opt {
    fn build_arg_error(reason: &str) -> Error {
        Error::ArgumentError(reason.to_string())
    }

    fn execute_optional_checker<F, V>(value_to_check: &Option<V>, checker: F) -> Result<(), Error>
    where
        F: Fn(&V) -> CheckerReturnType,
    {
        if let Some(v) = value_to_check {
            checker(v).map_err(|s| Self::build_arg_error(s.as_str()))
        } else {
            Ok(())
        }
    }

    fn execute_checker<F, V>(value_to_check: &V, checker: F) -> Result<(), Error>
    where
        F: Fn(&V) -> CheckerReturnType,
    {
        checker(value_to_check).map_err(|s| Self::build_arg_error(s.as_str()))
    }

    pub fn arg_check(&self) -> Result<(), Error> {
        Self::execute_checker(&self.host, |v| Checker::host(v.as_str()))?;
        Self::execute_checker(&self.source, |v| Checker::source(v.as_slice()))?;
        Self::execute_optional_checker(&self.proxy_url, |v| Checker::proxy_url(v.as_str()))?;
        Self::execute_optional_checker(&self.token, |v| Checker::token(v.as_str()))?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_default_opt() -> Opt {
        let args: Vec<std::ffi::OsString> = Vec::new();
        Opt::from_iter(args)
    }

    #[test]
    fn default_is_valid() {
        let op = new_default_opt();

        assert!(op.arg_check().is_ok());
    }
    #[test]
    fn token_check() {
        let mut op = new_default_opt();
        op.token = Some(String::from("abcd:123"));
        assert!(op.arg_check().is_ok());
        op.token = Some(String::from("abcd123"));
        assert!(op.arg_check().is_err());
        op.token = Some(String::from("ab cd:123"));
        assert!(op.arg_check().is_err());
    }
    #[test]
    fn dump_source_is_invalid() {
        let mut op = new_default_opt();
        op.source.resize(2, Provider::Bilibili);
        op.source[0] = Provider::Bilibili;
        op.source[1] = Provider::Bilibili;

        let check_result = op.arg_check();
        assert!(check_result.is_err());

        if let Err(Error::ArgumentError(msg)) = check_result {
            assert_eq!(
                msg,
                "Please check the duplication item(bilibili) in match order.".to_string()
            );
        } else {
            panic!("Not: Error::ArgumentError");
        }
    }
}
