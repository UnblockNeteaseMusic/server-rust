use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use unm_server::server::{root_handler, shutdown_signal};
use unm_server::{
    cli::{Opt, StructOpt},
    logger::*,
};

fn init_opt() -> Opt {
    let opt: Opt = Opt::from_args();

    if let Some(msg) = opt.arg_check() {
        panic!("\x1b[1;31mERROR:\x1b[0m {}", msg);
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
    init_logger(opt.env.log_level, &opt.env.json_log, &opt.env.log_file).expect("should be able to initiate loggers");
    // let proxy_manager =
    //     init_proxy_manager(&opt).expect("should be able to initiate the proxy manager");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(root_handler)) });
    let server = Server::bind(&addr)
        .serve(service)
        .with_graceful_shutdown(shutdown_signal());

    info!(
        "Welcome! You can access UNM service on: \x1b[1m{}\x1b[0m",
        addr.to_string()
    );
    // Run this server for... forever!
    if let Err(e) = server.await {
        error!("Server Error: {}", e);
    }
}
