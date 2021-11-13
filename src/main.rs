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
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
}

impl FromStr for UuidVersion {
    type Err = i32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "0" | "nil" => Ok(UuidVersion::V0),
            "1" | "mac" => Ok(UuidVersion::V1),
            "2" | "dce" => Ok(UuidVersion::V2),
            "3" | "md5" => Ok(UuidVersion::V3),
            "4" | "random" => Ok(UuidVersion::V4),
            "5" | "sha1" => Ok(UuidVersion::V5),
            _ => Err(-1),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "uuidrs",
    about = "Universally Unique Identifier Command-Line Tool in Rust",
    author = "github.com/mad4j"
)]
struct Opt {
    /// number of ids to be generated
    #[structopt(short, long, default_value = "1")]
    count: u16,

    /// format of generated ids {str|siv}
    #[structopt(short, long, default_value = "str")]
    format: OutputFormat,

    /// type of generated ids {nil|mac|dce|md5|random|sha1}
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
            UuidVersion::V0 => UUIDType::generate_v0_nil(),
            UuidVersion::V1 => UUIDType::generate_v1_mac(),
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
