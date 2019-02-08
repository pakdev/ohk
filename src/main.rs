#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
On-Hot-Key.

Usage:
  ohk <config>
  ohk [(<keys> <exe>)...]
  ohk (-h | --help)
  ohk --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_config: String,
    arg_keys: Vec<String>,
    arg_exe: Vec<String>,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
