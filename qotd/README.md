# Quote of the Day server

This server implements the Quote of the Day protocol
as per [RFC 865](https://tools.ietf.org/html/rfc865)
and is made concurrent by the use of Rust's
[threading facilities](http://doc.rust-lang.org/std/thread/index.html).

## Running

    $ cargo run -- -p 1717 quotes.txt

Then connect with nc (or a similar client).

    $ nc 127.0.0.1 1717
    "I can't be as confident about computer science as I can about biology. Biology easily has 500 years of exciting problems to work on. It's at that level." – Donald Knuth
