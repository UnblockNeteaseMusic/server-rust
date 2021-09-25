use std::error::Error;
use unm_server::cli::{Opt, StructOpt};
use unm_server::logger::*;
use unm_server::request;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    match opt.is_valid() {
        None => {}
        Some(msg) => {
            println!("{}", msg);
        }
    }

    init_logger(&opt)?;
    request::setup_global_proxy(&opt.proxy_url)?;
    info!("Info log!");
    warn!("Warn log with value {}", "test");
    error!("ERROR!");

    Ok(())
}
