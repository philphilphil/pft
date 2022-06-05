use std::net::SocketAddr;

use clap::{ArgEnum, ArgGroup, Parser, Subcommand};
use pft::{client, server};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    // /// Mode to run as
    // #[clap(arg_enum)]
    // mode: Mode,

    // /// Port to listen or sent to
    // #[clap(short, default_value = "localhost")]
    // host: String,

    // /// Port to listen or sent to
    // #[clap(short, long, long,parse(try_from_str=parse_port_input),default_value_t = 3030)]
    // port: usize,

    // /// One-time password for authentication
    // #[clap(short, long)]
    // otp: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    Server,
    Client,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Clone {
        /// The remote to clone
        remote: String,
    },
    /// pushes things
    #[clap(arg_required_else_help = true)]
    Push {
        /// The remote to target
        remote: String,
    },
}

fn main() {
    let args = Args::parse();
    let address = SocketAddr::from(([127, 0, 0, 1], 3030));

    // match args.mode {
    //     Mode::Server => server::start(&address),
    //     Mode::Client => client::start(&address),
    // }
}

fn parse_port_input(input_port: &str) -> Result<usize, String> {
    let port: usize = input_port
        .parse()
        .map_err(|_| format!("{} is not a valid number.", input_port))?;

    if !(1..=65535).contains(&port) {
        return Err(format!("{} is not a valid port.", port));
    }

    Ok(port)
}
