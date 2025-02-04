#![feature(path_file_prefix)]

mod app_args;
mod one_shot;
mod ratls_server;
mod signature;

extern crate rand;
extern crate secp256k1;

use anyhow::Result;
use app_args::{App, Command};
use clap::Parser;
use one_shot::{bootstrap, one_shot};
use ratls_server::ratls_server;

#[tokio::main]
pub async fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        Command::Server(server_args) => {
            println!("Starting RA-TLS server - listening on {}", server_args.addr);
            ratls_server(args.global_opts, server_args);
        }
        Command::OneShot(one_shot_args) => {
            println!("Starting one shot mode");
            one_shot(args.global_opts, one_shot_args).await?
        }
        Command::Bootstrap => {
            println!("Bootstrapping the app");
            bootstrap(args.global_opts)?
        }
    }

    Ok(())
}
