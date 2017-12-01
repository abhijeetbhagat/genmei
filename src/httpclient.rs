extern crate tokio_core;
extern crate hyper;
use httpbasedtransport::HttpBasedTransport;
use hyper::{Response, Client};
use hyper::client::{HttpConnector, FutureResponse};
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::io::{self, Write};
use connection::HubConnection;
use std::any::Any;
use futures::prelude::*;

pub trait HttpClient {
    fn get(&self) -> Response;
    fn post(&self) -> Response;
}

pub struct DefaultHttpClient {
    pub client : Client<HttpConnector>,
    pub core : Core,
    url : String,
    hub_name : String
}

impl DefaultHttpClient {
    pub fn new (url: String,hub_name : String) -> Self {
        let mut core = Core::new().unwrap();
        let client = Client::new (&core.handle());
        
        DefaultHttpClient{
            client : client,
            core : core,
            url : url,
            hub_name : hub_name
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

impl HttpClient for DefaultHttpClient {
    fn get (&self) -> Response {
        unimplemented!(); 
    } 

    fn post (&self) -> Response {
        unimplemented!(); 
    } 
}
