# rusted-uuidrs

simple UUID command line generation tool

``` bash
uuidrs 0.1.0
github.com/mad4j
Universally Unique Identifier Command-Line Tool in Rust

USAGE:
    uuidrs.exe [OPTIONS]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
    -c, --count <count>        number of ids to be generated [default: 1]
    -f, --format <format>      format of generated ids {str|siv} [default: str]
    -v, --version <version>    type of generated ids {nil|mac|dce|md5|random|sha1} [default: random]
```