extern crate hyper;
extern crate tokio_core;
use transports::httpbasedtransport::HttpBasedTransport;
use hyper::{Client, Error, Response};
use hyper::client::{FutureResponse, HttpConnector};
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;
use std::io::{self, Write};
use connection::HubConnection;
use std::any::Any;
use futures::prelude::*;

pub trait HttpClient {
    fn get(&mut self, url: &str) -> String;
    fn post(&self) -> Response;
}

pub struct DefaultHttpClient {
    pub client: Client<HttpConnector>,
    pub core: Core,
}

impl DefaultHttpClient {
    pub fn new() -> Self {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        DefaultHttpClient {
            client: client,
            core: core,
        }
    }
}

impl HttpClient for DefaultHttpClient {
    fn get(&mut self, url: &str) -> String {
        //unimplemented!();
        println!("{}", url);
        let work = self.client.get(url.parse().unwrap()).and_then(|res| {
            res.body()
                .fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                })
                .and_then(|v| {
                    let s = String::from_utf8(v).unwrap();
                    future::ok::<_, Error>(s)
                })
        });
        //TODO abhi: work should not be run here
        self.core.run(work).unwrap()
    }

    fn post(&self) -> Response {
        unimplemented!();
    }
}
