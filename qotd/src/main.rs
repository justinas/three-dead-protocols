use std::fs::File;
use std::io::{BufRead, BufReader, stderr, Write};
use std::net::{TcpListener};
use std::sync::Arc;
use std::thread;

extern crate clap;
extern crate time;
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
            write!(stderr(), "An invalid port number supplied, defaulting to 17.\n");
            17
        });

    let reader = BufReader::new(File::open(filename).ok().expect("Failed to open the quote file."));
    let quotes = Arc::new(reader.lines()
                           .map(|x| x.unwrap())
                           .filter(|x| x.len() != 0)
                           .collect::<Vec<String>>()
                          );

    let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
    let mut thread_handles = Vec::new();

    for inc in listener.incoming() {
        let mut conn = match inc {
            Ok(c) => c,
            Err(e) => {
                write!(stderr(), "Errror while trying to accept a connection: {}\n", e);
                continue;
            },
        };
        let peer_addr = conn.peer_addr().unwrap();
        write!(stderr(), "Successful connection from: {}\n", peer_addr);

        let local_quotes = quotes.clone();

        thread_handles.push(thread::spawn(move || {
            let tm = time::now_utc();
            let day = tm.tm_yday as usize;
            let quote = &local_quotes[day % local_quotes.len()];
            match conn.write(quote.as_bytes()) {
                Ok(_) => write!(stderr(), "Sent the quote successfully to {}.\n", peer_addr),
                Err(e) => write!(stderr(), "Error while sending the quote to {}: {}\n", peer_addr, e),
            };
        }));
    }

    for h in thread_handles {
        h.join();
    }
}
