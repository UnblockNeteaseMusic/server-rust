use regex::Regex;
pub use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Clone)]
/// The source provider
pub enum Provider {
    QQ,
    Kugou,
    Kuwo,
    Migu,
    Joox,
    Youtube,
    YoutubeDL,
    Bilibili,
    Pyncmd,
}

impl std::str::FromStr for Provider {
    type Err = String;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "qq" => Ok(Provider::QQ),
            "kugou" => Ok(Provider::Kugou),
            "kuwo" => Ok(Provider::Kuwo),
            "migu" => Ok(Provider::Migu),
            "joox" => Ok(Provider::Joox),
            "youtube" => Ok(Provider::Youtube),
            "youtubedl" => Ok(Provider::YoutubeDL),
            "bilibili" => Ok(Provider::Bilibili),
            "pyncmd" => Ok(Provider::Pyncmd),
            _ => Err(String::from(format!("{}", src))),
        }
    }
}
impl std::fmt::Debug for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Provider::QQ => "qq",
                Provider::Kugou => "kugou",
                Provider::Kuwo => "kuwo",
                Provider::Migu => "migu",
                Provider::Joox => "joox",
                Provider::Youtube => "youtube",
                Provider::YoutubeDL => "youtubedl",
                Provider::Bilibili => "bilibili",
                Provider::Pyncmd => "pyncmd",
            }
        )
    }
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
    #[structopt(short, long)]
    pub strict: bool,

    #[structopt(short, long)]
    /// set up proxy authentication
    pub token: Option<String>,
}

impl Opt {
    pub fn is_valid(&self) -> Option<String> {
        let mut rst = self.proxy_url.as_ref().and_then(|url| {
            let proxy_url_re: Regex =
                Regex::new(r"^http(s?):\/\/.+:\d+$").expect("wrong regex of proxy url");
            match proxy_url_re.is_match(&url) {
                true => None,
                false => Some(String::from("lease check the proxy url.")),
            }
        });
        if rst.is_some() {
            return rst;
        }

        rst = self.endpoint.as_ref().and_then(|url| {
            let re = Regex::new(r"^http(s?):\/\/.+$").expect("wrong regex of endpoint");
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
