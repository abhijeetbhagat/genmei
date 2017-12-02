use futures::future::{FutureResult, Future};
use negotiationresponse::NegotiationResponse;

pub trait ClientTransport {
    fn negotiate(&self) -> Box<Future<Item=NegotiationResponse, Error=()>>;
    fn start(&self) -> Box<Future<Item=(), Error=()>>;
    fn send(&self) -> Box<Future<Item=(), Error=()>>;
    fn abort(&self) -> Box<Future<Item=(), Error=()>>;
}

fn foo(ct : &ClientTransport) {}
