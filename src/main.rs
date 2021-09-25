use std::error::Error;

use unm_server::cli::{Opt, StructOpt};
use unm_server::logger::*;
use unm_server::request::proxy::ProxyManager;

fn main() -> Result<(), Box<dyn Error>> {
    let opt: Opt = Opt::from_args();
    println!("{:#?}", opt);
    match opt.is_valid() {
        None => {}
        Some(msg) => {
            println!("{}", msg);
        }
    }

    init_logger(&opt)?;
    let mut proxy_manager = ProxyManager { proxy: None };
    proxy_manager.setup_proxy(&opt.proxy_url.unwrap())?;

    info!("Info log!");
    warn!("Warn log with value {}", "tests");
    error!("ERROR!");

    Ok(())
}
