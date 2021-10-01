use unm_server::{
    cli::{Opt, StructOpt},
    error::*,
    logger::*,
    request::proxy::ProxyManager,
};

fn init_opt() -> Opt {
    let opt: Opt = Opt::from_args();

    if let Some(msg) = opt.arg_check() {
        panic!("\x1b[1;31mERROR:\x1b[0m {}", msg);
    }

    opt
}

fn init_proxy_manager(opt: &Opt) -> Result<ProxyManager> {
    let mut proxy_manager = ProxyManager { proxy: None };
    if let Some(url) = &opt.proxy_url {
        proxy_manager.setup_proxy(url)?;
    };

    Ok(proxy_manager)
}

fn main() -> Result<()> {
    let opt = init_opt();
    init_logger(&opt)?;
    let proxy_manager = init_proxy_manager(&opt)?;

    info!("Info log!");
    warn!("Warn log with value {}", "tests");
    error!("ERROR!");

    Ok(())
}
