use log::error;
use std::net::SocketAddr;
use unm_core::server::{HttpServerConfig, HttpsServerConfig, Server};

use crate::cli::{Opt, StructOpt};
use crate::logger::init_logger;

mod cli;
mod logger;

fn init_opt() -> Opt {
    let opt: Opt = Opt::from_args();

    if let Err(msg) = opt.arg_check() {
        error!("\x1b[1;31mARGUMENT ERROR:\x1b[0m {}", msg);
        std::process::exit(1);
    }

    opt
}

// fn init_proxy_manager(opt: &Opt) -> Result<ProxyManager> {
//     let mut proxy_manager = ProxyManager { proxy: None };
//     if let Some(url) = &opt.proxy_url {
//         proxy_manager.setup_proxy(url)?;
//     };
//
//     Ok(proxy_manager)
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = init_opt();
    init_logger(opt.env.log_level, &opt.env.json_log, &opt.env.log_file)
        .expect("should be able to initiate loggers");
    // let proxy_manager =
    //     init_proxy_manager(&opt).expect("should be able to initiate the proxy manager");

    let server = Server {
        http: HttpServerConfig {
            address: SocketAddr::from(([127, 0, 0, 1], 3000)),
        },
        https: Some(HttpsServerConfig {
            address: SocketAddr::from(([127, 0, 0, 1], 3001)),
        }),
    };
    let result = server.serve().await?;

    Ok(result)
}
