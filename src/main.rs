use std::{
    io,
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use pft::{client, server};

#[derive(Parser)]
#[clap(author, version, about="Very simple file transfer.", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Mode,
}

#[derive(Subcommand)]
enum Mode {
    /// Runs the client
    #[clap(arg_required_else_help = true)]
    Client {
        /// Address to sent to. Format: host:port
        #[clap(required = true)]
        address: String,
        /// One-time password for authentication
        #[clap(required = true)]
        otp: String,
        /// Path to the file to upload
        #[clap(required = true, parse(from_os_str))]
        file_path: PathBuf,
    },
    /// Runs the server
    #[clap(arg_required_else_help = true)]
    Server {
        /// Address to listen to. Format: host:port
        #[clap(default_value = "localhost")]
        address: String,
    },
}

fn main() {
    let args = Args::parse();
    let address = match get_addr(&args.command) {
        Ok(address) => address,
        Err(e) => {
            println!("ERROR: Invalid address: {}.", e);
            return;
        }
    };
    match args.command {
        Mode::Server { .. } => server::start(&address),
        Mode::Client { otp, file_path, .. } => client::start(&address, otp, file_path),
    }
}

fn get_addr(mode: &Mode) -> io::Result<SocketAddr> {
    let address = match mode {
        Mode::Client { address, .. } | Mode::Server { address } => address,
    };

    let mut address = address.to_socket_addrs()?;
    Ok(address.next().unwrap())
}
