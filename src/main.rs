use unm_server::cli::{Opt, StructOpt};

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let msg = opt.is_valid();
    match msg {
        None => {}
        Some(msg) => {
            println!("{}", msg);
        }
    }
}
