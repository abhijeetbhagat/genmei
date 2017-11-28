extern crate tokio_core;
extern crate hyper;
use httpbasedtransport::HttpBasedTransport;
use hyper::Client;
use hyper::client::{HttpConnector, FutureResponse};
use tokio_core::reactor::Core;
use futures::{Future, Stream};
use std::io::{self, Write};
use connection::HubConnection;
use std::any::Any;
use futures::prelude::*;

pub struct HttpClient {
    pub client : Client<HttpConnector>,
    pub core : Core,
    url : String,
    hub_name : String
}

impl HttpClient {
    pub fn new (url: String,hub_name : String) -> Self {
        let mut core = Core::new().unwrap();
        let client = Client::new (&core.handle());
        
        HttpClient{
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

    pub fn create_negotiate_request(&mut self){
       let conn_name = &self.hub_name;
       println!("urlk :: {}",self.url); 
       let url1 = format!("{}/negotiate?clientProtocol={}&connectionData=[%7B%22name%22:%22{}%22%7D]", self.url, 1.4, conn_name).parse().unwrap();
      
       println!("doing get request {}",url1);
       let work = self.client.get(url1).map(|res|{
           println!("negotiation status {}",res.status());
          // println!("body : {:?}",res.body());
           assert_eq!(res.status(),hyper::StatusCode::Ok);
       });
       self.core.run(work);
    }
}

impl HttpBasedTransport for HttpClient {
    fn init () {

    } 
}
