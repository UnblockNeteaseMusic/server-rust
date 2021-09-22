use unm_server::cli::{Opt, StructOpt};
use unm_server::logger::*;

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    match opt.is_valid() {
        None => {}
        Some(msg) => {
            println!("{}", msg);
        }
    }
    init_logger(&opt);
}
