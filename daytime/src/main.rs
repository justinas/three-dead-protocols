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

struct EchoHandler {
    listener: TcpListener,
    next_token: usize,
}

impl EchoHandler {
    fn new(l: TcpListener) -> EchoHandler {
        EchoHandler {
            listener: l,
            next_token: 1
        }
    }
}

impl Handler for EchoHandler {
    type Message = ();
    type Timeout = ();

    fn ready(&mut self, event_loop: &mut EventLoop<Self>, token: Token, events: EventSet) {
        match token {
            SERVER => {
                let client = match self.listener.accept() {
                    // As per documentation, Ok(None) means WOULDBLOCK.
                    Ok(None) => return,
                    Ok(Some(c)) => {
                        let addr = c.peer_addr().unwrap();
                        write!(stderr(), "Succesful connection from {}\n", addr);
                        c
                    }
                    Err(e) => {
                        write!(stderr(), "Error while accepting: {}\n", e);
                        return;
                    }
                };

                event_loop.register(&client, Token(self.next_token));
                self.next_token += 1;
            }
            _ => {}
        }
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
    event_loop.run(&mut EchoHandler::new(listener)).unwrap();
}
