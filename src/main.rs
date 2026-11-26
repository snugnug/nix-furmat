use pound::Parse;

use crate::format::FormatterConfig;

mod format;

#[derive(Parse, Debug)]
#[pound(name = "nix-furmat")]
struct Cli {
    file: String,
}

fn main() {
    match Cli::try_parse() {
        Ok(cli) => {
            let config = FormatterConfig::new(&cli.file);
            format::run_format(config, cli.file);
        }
        Err(e) => {
            println!("{}", e)
        }
    };
}
