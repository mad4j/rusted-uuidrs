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

#[derive(Debug)]
enum UuidVersion {
    V1,
    V2,
    V3,
    V4,
}

impl FromStr for UuidVersion {
    type Err = i32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "1" | "time" => Ok(UuidVersion::V1),
            "2" | "v2" => Ok(UuidVersion::V2),
            "3" | "v3" => Ok(UuidVersion::V3),
            "4" | "random" => Ok(UuidVersion::V4),
            _ => Err(-1),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "uuidrs",
    about = "Universally Unique Identifier Command-Line Tool in Rust",
    author = "daniele.olmisani@gmail.com"
)]
struct Opt {
    /// number of ids to be generated
    #[structopt(short, long, default_value = "1")]
    count: u16,

    /// format of generated ids
    #[structopt(short, long, default_value = "str")]
    format: OutputFormat,

    /// type of generated ids [ time | 2 | 3 | random ]
    #[structopt(short, long, default_value = "random")]
    version: UuidVersion,
}

fn main() {
    // parse command-line parameters
    let opt = Opt::from_args();

    // generate several ids
    for _ in 0..opt.count {
        // generate a new id
        let uuid = match opt.version {
            UuidVersion::V4 => UUIDType::generate_v4_random(),
            _ => UUIDType::generate_v4_random(),
        };

        println!(
            "{}",
            match opt.format {
                OutputFormat::STR => uuid.to_string(),
                OutputFormat::SIV => uuid.to_siv_string(),
            }
        );
    }
}
