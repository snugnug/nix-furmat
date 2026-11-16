use pound::Parse;

#[derive(Parse, Debug)]
#[pound(name = "nix-furmat")]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    println!("Hello, world!");
}
