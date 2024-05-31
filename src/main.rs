use clap::Parser;
use db_gen::cli::Cli;
use log::error;

fn main() {
    env_logger::init();

    if let Err(err) = Cli::parse().main() {
        error!("Critical error: {:?}", err);
    }
}
