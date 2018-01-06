use transports::clienttransport::ClientTransport;
use futures::future::{ok, Future};
use negotiationresponse::NegotiationResponse;
use httpclient::{DefaultHttpClient, HttpClient};
use urlbuilder::UrlBuilder;
use serde_json;
use serde_json::{Map, Value};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::marker::Send;

pub struct ServerSentEventsTransport {
    http_client: Box<HttpClient>,
}

impl ServerSentEventsTransport {
    pub fn new() -> Self {
        ServerSentEventsTransport {
            http_client: Box::new(DefaultHttpClient::new()),
        }
    }

    fn open_connection(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        map: &mut Map<String, Value>,
        sender: Option<Sender<Vec<u8>>>,
    ) {
        let url = UrlBuilder::create_connect_url(
            url,
            Some("serverSentEvents"),
            connection_data,
            Some(connection_token),
            protocol,
        );

        let response = self.http_client.get_stream(
            &url,
            Some(vec![
                ("Accept", "text/event-stream"),
                ("User-Agent", "genmei"),
            ]),
            sender.unwrap(),
        );
        //ServerSentEventsTransport::process_response(response)
    }

    fn process_response(response: String) {
        println!("serversent: process_response - {}", response);
        serde_json::from_str(&response).unwrap()
        //if map.contains_key(String::from("I"))
    }
}

impl ClientTransport for ServerSentEventsTransport {
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
        sender: Option<Sender<Vec<u8>>>,
    ) -> Box<Future<Item = Map<String, Value>, Error = ()>> {
        let mut map = Map::new();
        self.open_connection(
            url,
            connection_data,
            connection_token,
            protocol,
            &mut map,
            sender,
        );
        Box::new(ok::<_, _>(map))
    }


    fn send(
        &self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        data: String,
    ) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
