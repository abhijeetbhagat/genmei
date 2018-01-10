use futures::future::{Future, FutureResult};
use negotiationresponse::NegotiationResponse;
use connection::Connection;
use serde_json::{Map, Value};
use std::sync::mpsc::Sender;

pub trait ClientTransport {
    fn name(&self) -> &str;
    fn negotiate(
        &mut self,
        url: &str,
        connection_data: &str,
        protocol: &str,
    ) -> Box<Future<Item = NegotiationResponse, Error = ()>>;
    fn start(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        sender: Option<Sender<Vec<u8>>>,
    ) -> Box<Future<Item = (), Error = ()>>;
    fn send(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        data: String,
    ) -> Box<Future<Item = (), Error = ()>>;
    fn abort(&self) -> Box<Future<Item = (), Error = ()>>;
}
