use std::collections::HashMap;
use std::io::{stderr, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;

extern crate clap;
use clap::{App, Arg};

extern crate mio;
use mio::{EventLoop, EventSet, Handler, Token};
use mio::tcp::TcpListener;

const SERVER: Token = Token(0);

struct EchoHandler(TcpListener);

impl Handler for EchoHandler {
    type Message = ();
    type Timeout = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) {
    }
}

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

    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let addr = SocketAddrV4::new(ip, port);
    let listener = TcpListener::bind(&SocketAddr::V4(addr)).unwrap();

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.register(&listener, SERVER).unwrap();
    event_loop.run(&mut EchoHandler(listener)).unwrap();
}
