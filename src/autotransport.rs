use clienttransport::ClientTransport;
use futures::future::Future;
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;
use urlbuilder::UrlBuilder;
use connection::Connection;

pub struct AutoTransport {
    http_client : Box<HttpClient>
}

impl AutoTransport {
    pub fn new(http_client : Box<HttpClient>) -> Self {
        AutoTransport {
            http_client : http_client
        }
    }
}

impl ClientTransport for AutoTransport {
    fn negotiate(&mut self, url : &str, connection_data : &str, protocol : &str) -> Box<Future<Item=NegotiationResponse, Error=()>> {
        unimplemented!();
        let url = UrlBuilder::create_negotiate_url(url, connection_data, protocol);
        let response = self.http_client.get(url.as_str());
    }

    fn start(&self) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn send(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn abort(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }
}


