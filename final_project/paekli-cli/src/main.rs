use clap::Parser;

#[derive(Subcommand)]
enum Command {
    send,
    receive,
}

/// send and receive paekli's via command line interface
#[derive(Parser)]
#[clap(version)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let _args = CLI::parse();
    println!("Hello, world!");
}
