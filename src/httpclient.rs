extern crate tokio_core;
use httpbasedtransport::HttpBasedTransport;
use hyper::Client;
use hyper::client::{HttpConnector, FutureResponse};
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::io::{self, Write};

pub struct HttpClient {
    pub client : Client<HttpConnector>,
    pub core : Core
}

impl HttpClient {
    pub fn new () -> Self {
        let mut core = Core::new().unwrap();
        let client = Client::new (&core.handle());
        
        HttpClient{
            client : client,
            core : core
        }
    }

    pub fn get (&mut self, url : &str) {
        let work = self.client.get (url.parse().unwrap()).and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
            })
        });
        self.core.run(work).unwrap();
    } 
}

impl HttpBasedTransport for HttpClient {
    fn init () {

    } 
}
