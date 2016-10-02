extern crate clap;
extern crate jvmrs;

#[macro_use]
extern crate log;
extern crate log4rs;

use jvmrs::logging::logger::init_logger;
use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_arguments() -> clap::ArgMatches<'static> {
    let matches = App::new("jvm-rs")
        .version(VERSION)
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("JVM in Rust.")
        .arg(Arg::with_name("log_config")
            .short("lc")
            .long("log_config")
            .value_name("FILE")
            .help("Sets a configuration file for the logger.")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .takes_value(false)
            .required(false)
            .help("Sets the level of logging verbosity."))
        .get_matches();

    return matches;
}

fn main() {
    let matches = get_arguments();

    let log_config = matches.value_of("log_config").unwrap_or("./log4rs.yml");

    init_logger(log_config);

    info!("jvm-rs starting NOW..");
}
