//! This file is part of jvm-rs.

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

use clap::App;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn get_arguments() -> clap::ArgMatches<'static> {
    App::new("jvm-rs")
        .version(VERSION)
        .author("Dom Rodriguez <shymega@shymega.org.uk>")
        .about("JVM written in Rust.")
        .get_matches()
}

fn main() {
    let _matches = get_arguments();
}
