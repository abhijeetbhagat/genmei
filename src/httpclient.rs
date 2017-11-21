extern crate tokio_core;
use httpbasedtransport::HttpBasedTransport;
use hyper::Client;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;

pub struct HttpClient {
    client : Client<HttpConnector>,
    core : Core
}

impl HttpClient {
    fn new () -> Self {
        unimplemented!()
        //let mut core = Core::new()?;
        //let client = Client::new (&core.handle());
        //TODO abhi get the signalR uri here and use it
        //let work = client.get ("https://localhost:8080/signalr/...");

    }

}

impl HttpBasedTransport for HttpClient {
    fn init () {

    } 
}
