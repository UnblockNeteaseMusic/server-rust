use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use log::{error, info};

use unm_core::server::{root_handler, shutdown_signal};

use crate::cli::{Opt, StructOpt};
use crate::logger::init_logger;

mod cli;
mod logger;

fn init_opt() -> Opt {
    let opt: Opt = Opt::from_args();

    if let Err(msg) = opt.arg_check() {
        eprintln!("\x1b[1;31mARGUMENT ERROR:\x1b[0m {}", msg);
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
async fn main() {
    let opt = init_opt();
    init_logger(opt.env.log_level, &opt.env.json_log, &opt.env.log_file)
        .expect("should be able to initiate loggers");
    // let proxy_manager =
    //     init_proxy_manager(&opt).expect("should be able to initiate the proxy manager");

    let instantiate_server = |addr: SocketAddr| {
        let service =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(root_handler)) });
        let server = Server::bind(&addr)
            .serve(service)
            .with_graceful_shutdown(shutdown_signal());

        server
    };

    let http = |port: u16| {
        tokio::spawn(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let server = instantiate_server(addr);

            info!(
                "[HTTP] Welcome! You can access UNM service on: \x1b[1m{}\x1b[0m",
                addr.to_string()
            );

            server.await
        })
    };

    let https = |port: u16| {
        tokio::spawn(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            let server = instantiate_server(addr);

            info!(
                "[HTTPS] Welcome! You can access UNM service on: \x1b[1m{}\x1b[0m",
                addr.to_string()
            );

            server.await
        })
    };

    let (http, https) = tokio::join!(http(3000), https(3001));

    if let Err(e) = http {
        error!("[HTTP] Server Error: {}", e);
    }

    if let Err(e) = https {
        error!("[HTTPS] Server Error: {}", e);
    }
}
