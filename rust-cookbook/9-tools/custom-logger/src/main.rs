#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::{Builder, Target};

fn main() {
    Builder::new()
        .target(Target::Stdout)
        .init();

    error!("This error has been printed to Stdout");
}