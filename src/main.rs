use unm_server::{
    cli::{Opt, StructOpt},
    error::*,
    logger::*,
    request::proxy::ProxyManager,
};

fn main() -> Result<()> {
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
    if let Some(url) = &opt.proxy_url {
        proxy_manager.setup_proxy(url)?;
    };

    info!("Info log!");
    warn!("Warn log with value {}", "tests");
    error!("ERROR!");

    Ok(())
}
