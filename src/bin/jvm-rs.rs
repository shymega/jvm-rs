// This file is part of jvm-rs.

// jvm-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// jvm-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with jvm-rs.  If not, see <http://www.gnu.org/licenses/>

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
