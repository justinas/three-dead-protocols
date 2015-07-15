extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("qotd")
        .author("Justinas Stankevicius")
        .arg_required_else_help(true)
        .arg(
            Arg::with_name("quote_file").index(1)
            .help("The file with the quotes to use, one per line.")
        )
        .get_matches();

    // unwrap() should be fine
    // as the existence of the argument is enforced by arg_required_else_help()
    let filename = matches.value_of("quote_file").unwrap();
    println!("{}", filename)
}
