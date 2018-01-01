use transports::clienttransport::ClientTransport;
use futures::future::Future;
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;
use urlbuilder::UrlBuilder;
use connection::Connection;
use serde_json::{Map, Value};
use std::sync::mpsc::Sender;

pub struct LongPollingTransport;

impl ClientTransport for LongPollingTransport {
    fn negotiate(
        &mut self,
        url: &str,
        connection_data: &str,
        protocol: &str,
    ) -> Box<Future<Item = NegotiationResponse, Error = ()>> {
        unimplemented!();
    }

    fn start(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        sender : Option<Sender<Vec<u8>>>
    ) -> Box<Future<Item = Map<String, Value>, Error = ()>> {
        unimplemented!();
        //let url = UrlBuilder::create_connect_url(url, conn)
    }

    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
