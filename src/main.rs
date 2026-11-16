use pound::Parse;

mod format;

#[derive(Parse, Debug)]
#[pound(name = "nix-furmat")]
struct Cli {
    file: String,
}

fn main() {
    match Cli::try_parse() {
        Ok(cli) => format::format(cli.file),
        Err(e) => {
            println!("{}", e)
        }
    };
}
