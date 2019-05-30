extern crate clap;
extern crate leabharlann;

use clap::{App, Arg};
use std::env;
use std::process;

use leabharlann::Config;

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("a little tool to keep track of reading lists")
        .arg(
            Arg::with_name("ACTION")
                .required(true)
                .takes_value(false)
                .index(1)
                .help(
                    "`now` or `anois` to document current reading\n\
                     `finish` or `criochnaigh` to document finished reading\n\
                     `future` or `sa-todhchai` to document future reading",
                ),
        )
        .get_matches();

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = leabharlann::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
