use std::net::SocketAddr;

use clap::{ArgEnum, Parser};
use pft::{client, server};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Mode to run as
    #[clap(arg_enum)]
    mode: Mode,

    /// Port to listen or sent to
    #[clap(short, long, default_value_t = 3030)]
    port: u16,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Server,
    Client,
}

fn main() {
    let args = Args::parse();
    let address = SocketAddr::from(([127, 0, 0, 1], 3030));

    match args.mode {
        Mode::Server => server::start(&address),
        Mode::Client => client::start(&address),
    }
}
