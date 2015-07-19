use std::io::{stderr, Write};

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("qotd")
        .author("Justinas Stankevicius")
        .arg(Arg::with_name("port").short("p").takes_value(true))
        .get_matches();

    let port = matches.value_of("port").unwrap_or("7")
        .parse::<u16>().ok().unwrap_or_else(|| {
            write!(stderr(), "An invalid port number supplied, defaulting to 7.\n");
            7
        });

    println!("{:?}", port);
}
