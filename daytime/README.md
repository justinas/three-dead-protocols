# Daytime protocol server

This server implements the Daytime protocol
as per [RFC 867](https://tools.ietf.org/html/rfc867)
and is made concurrent with the help of
[mio](https://github.com/carllerche/mio)
crate written by Carl Lerche.

## Running

    $ cargo run -- -p 7777

Then connect with nc (or a similar client).

## Limitations

This server does not fully conform to the protocol,
as conforming servers are expected to
read any data sent by the client,
while this server does not perform any reads.
