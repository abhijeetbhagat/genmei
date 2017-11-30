use futures::future::Future;
use negotiationresponse::NegotiationResponse;

pub trait ClientTransport {
    fn negotiate(&self) -> Future<Item=NegotiationResponse, Error=()>;
    fn start(&self) -> Future<Item=(), Error=()>;
    fn send(&self) -> Future<Item=(), Error=()>;
    fn abort(&self) -> Future<Item=(), Error=()>;
}

fn foo(ct : &ClientTransport) {}
