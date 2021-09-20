use unm_server::cli::{Opt, StructOpt};

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
