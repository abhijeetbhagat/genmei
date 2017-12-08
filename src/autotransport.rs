use clienttransport::ClientTransport;
use futures::future::Future;
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;

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
    fn negotiate(&mut self) -> Box<Future<Item=NegotiationResponse, Error=()>> {
        unimplemented!();
    }

    fn start(&self, hub_name : String) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn send(&self) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item=(), Error=()>> {

        unimplemented!();
    }
}


