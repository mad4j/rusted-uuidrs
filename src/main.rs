mod uuid;

use std::str::FromStr;
use structopt::StructOpt;
use uuid::UUIDType;

#[derive(Debug)]
enum OutputFormat {
    STR,
    SIV,
}

impl FromStr for OutputFormat {
    type Err = i32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "str" => Ok(OutputFormat::STR),
            "siv" => Ok(OutputFormat::SIV),
            _ => Err(-1),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "uuidrs",
    about = "Universally Unique Identifier Command-Line Tool in Rust",
    author = "daniele.olmisani@gmail.com",
)]
struct Opt {
    /// number of ids to be generated
    #[structopt(short, long, default_value = "1")]
    count: u16,

    /// format of generated ids
    #[structopt(short, long, default_value = "str")]
    format: OutputFormat,
}

fn main() {
    // parse command-line parameters
    let opt = Opt::from_args();

    for _ in 0..opt.count {

        // generate a new id
        let uiid = UUIDType::random();

        println!(
            "{}",
            match opt.format {
                OutputFormat::STR => uiid.to_string(),
                OutputFormat::SIV => uiid.to_siv_string(),
            }
        );
    }
}
