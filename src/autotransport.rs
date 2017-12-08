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
    fn negotiate(&mut self, conn : &Connection, connection_data : String) -> Box<Future<Item=NegotiationResponse, Error=()>> {
        unimplemented!();
        let url = UrlBuilder::create_base_url(conn, "negotiate", None, connection_data.as_str());
        let response = self.http_client.get(url.as_str());
    }

    fn start(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn send(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn abort(&self, conn : &Connection) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }
}


