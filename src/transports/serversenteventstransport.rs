use transports::clienttransport::ClientTransport;
use futures::future::{ok, Future};
use negotiationresponse::NegotiationResponse;
use httpclient::{DefaultHttpClient, HttpClient};
use urlbuilder::UrlBuilder;
use connection::Connection;
use serde_json;
use serde_json::{Map, Value};
use std::sync::{Arc, Mutex};
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
        sender : Option<Sender<Vec<u8>>>
    ) {
        let url = UrlBuilder::create_connect_url(
            url,
            Some("serverSentEvents"),
            connection_data,
            Some(connection_token),
            protocol,
        );

        let (tx, rx) = mpsc::channel();
        let response = self.http_client.get_stream(
            &url,
            Some(vec![
                ("Accept", "text/event-stream"),
                ("User-Agent", "genmei"),
            ]),
            tx
        );
        loop {
            let vec = rx.recv().unwrap();
            println!("oc: chunk: {:?}", vec);

            if vec.len() > 19{
                use std;
                let data = std::str::from_utf8(&vec).unwrap();
                /*{
                    "C": "d-A2D08C-B,1|C,0|D,1",
                    "M": [
                            {
                            "H": "MyHub",
                            "M": "send",
                            "A": [
                            "client message"
                            ]
                            }
                    ]
                }*/
                if data.contains("data:") { //we do not deal with "data:{}"
                    let map : Map<String, Value> = serde_json::from_str(&data[5..]).unwrap();
                    if map.contains_key(&String::from("S")) && map.get(&String::from("S")).unwrap().as_u64().unwrap() == 1u64 {
                        //TODO abhi: initiate a 'start' request
                    }
                    if let Some(messages) = map.get(&String::from("M")) { 
                        let messages = messages.as_array().unwrap();
                        for message in messages {
                            let hub = &message[&String::from("H")];
                            println!("{:?}", hub);
                            let method = &message[&String::from("M")];
                            println!("{:?}", method);
                            let args = &message[&String::from("A")];
                            println!("{:?}", args);
                        }
                    }
                }
            }
        }
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
        sender : Option<Sender<Vec<u8>>>
    ) -> Box<Future<Item = Map<String, Value>, Error = ()>> {
        let mut map = Map::new();
        self.open_connection(url, connection_data, connection_token, protocol, &mut map, sender);
        Box::new(ok::<_, _>(map))
    }


    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
