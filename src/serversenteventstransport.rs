use clienttransport::ClientTransport;
use futures::future::{Future, ok};
use negotiationresponse::NegotiationResponse;
use httpclient::{HttpClient, DefaultHttpClient};
use urlbuilder::UrlBuilder;
use connection::Connection;

pub struct ServerSentEventsTransport {
    http_client : Box<HttpClient>
}

impl ServerSentEventsTransport {
    pub fn new () -> Self {
        ServerSentEventsTransport {
            http_client : Box::new(DefaultHttpClient::new())
        }
    }

    fn open_connection(&self, url : &str, connection_data : &str, connection_token : &str, protocol : &str) {
        let url = UrlBuilder::create_connect_url(url, Some("serversentevent"), connection_data, Some(connection_token), protocol);
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
        &self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
    ) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
        self.open_connection(url, connection_data, connection_token, protocol);
        Box::new(ok::<_,_>(()))

    }


    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
