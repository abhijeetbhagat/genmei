extern crate tokio_core;
use httpbasedtransport::HttpBasedTransport;
use hyper::Client;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;

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

}

impl HttpBasedTransport for HttpClient {
    fn init () {

    } 
}
