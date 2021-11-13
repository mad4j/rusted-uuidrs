
mod uuid;

use uuid::UUIDType;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "uuidrs", about = "geration of UUID values")]
struct Opt {
    /// number of ids to be generated
    #[structopt(short, long, default_value = "1")]
    count: u16,
}

fn main() {

    // parse command-line parameters
    let opt = Opt::from_args();

    for _ in 0..opt.count {
        println!("{}", UUIDType::random());
    }
}
