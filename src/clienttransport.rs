use futures::future::{FutureResult, Future};
use negotiationresponse::NegotiationResponse;
use connection::Connection;

pub trait ClientTransport {
    fn negotiate(&mut self, conn : &Connection, connection_data : String) -> Box<Future<Item=NegotiationResponse, Error=()>>;
    fn start(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>>;
    fn send(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>>;
    fn abort(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>>;
}

fn foo(ct : &ClientTransport) {}
