pub use structopt::StructOpt;


/// The options of the CLI of UNM (Rust)
#[derive(StructOpt, Debug)]
#[structopt(name = "unm-server-rust")]
pub struct Opt {
    /// Specify the server port of UNM.
    #[structopt(short, long, default_value = "8080;8081", use_delimiter=true, value_delimiter=";")]
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
    #[structopt(short = "o", long)]
    pub source: Vec<String>,

    /// Replace the virtual endpoint with public host. (Not implemented)
    #[structopt(short, long)]
    pub endpoint: Option<String>,

    /// Enable the proxy limitation. (Not implemented)
    #[structopt(short, long)]
    pub strict: bool,
}
