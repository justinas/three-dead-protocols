use std::fs::File;
use std::io::{BufRead, BufReader, stderr, Write};

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("qotd")
        .author("Justinas Stankevicius")
        .arg_required_else_help(true)
        .arg(
            Arg::with_name("quote_file").index(1).required(true)
            .help("The file with the quotes to use, one per line.")
        )
        .arg(Arg::with_name("port").short("p").takes_value(true))
        .get_matches();

    // unwrap() should be fine
    // as the existence of the argument is enforced by arg_required_else_help()
    let filename = matches.value_of("quote_file").unwrap();
    let port = matches.value_of("port").unwrap_or("17")
        .parse::<u16>().ok().unwrap_or_else(|| {
            write!(stderr(), "{}", "An invalid port number supplied, defaulting to 17.\n");
            17
        });

    let reader = BufReader::new(File::open(filename).ok().expect("Failed to open the quote file."));
    let quotes: Vec<String> = reader.lines().map(|x| x.unwrap()).filter(|x| x.len() != 0).collect();

    println!("{}", filename);
    println!("{}", port);
    println!("{:?}", quotes);
}
