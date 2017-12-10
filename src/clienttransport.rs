use futures::future::{FutureResult, Future};
use negotiationresponse::NegotiationResponse;
use connection::Connection;

pub trait ClientTransport {
    fn negotiate(&mut self, url : &str, connection_data : &str, protocol : &str) -> Box<Future<Item=NegotiationResponse, Error=()>>;
    fn start(&self) -> Box<Future<Item=(), Error=()>>;
    fn send(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>>;
    fn abort(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>>;
}

fn foo(ct : &ClientTransport) {}
