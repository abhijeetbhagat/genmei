use futures::future::{Future, FutureResult};
use negotiationresponse::NegotiationResponse;
use connection::Connection;

pub trait ClientTransport {
    fn negotiate(
        &mut self,
        url: &str,
        connection_data: &str,
        protocol: &str,
    ) -> Box<Future<Item = NegotiationResponse, Error = ()>>;
    fn start(
        &self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
    ) -> Box<Future<Item = (), Error = ()>>;
    fn send(&self) -> Box<Future<Item = (), Error = ()>>;
    fn abort(&self) -> Box<Future<Item = (), Error = ()>>;
}

fn foo(ct: &ClientTransport) {}
