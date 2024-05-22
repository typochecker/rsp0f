use clap::Parser;
use rsp0f::Cli;

fn main() {
    let args = Cli::parse();

    println!("{:?}", args);
}
