use clienttransport::ClientTransport;
use futures::future::Future;
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;
use urlbuilder::UrlBuilder;
use connection::Connection;

pub struct LongPollingTransport;

impl ClientTransport for LongPollingTransport {
    fn negotiate(
        &mut self,
        url: &str,
        connection_data: &str,
        protocol: &str,
    ) -> Box<Future<Item = NegotiationResponse, Error = ()>> {
        unimplemented!();
        let url = UrlBuilder::create_negotiate_url(url, connection_data, protocol);
        let response = self.http_client.get(url.as_str());
    }

    fn start(
        &self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str
    ) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
        //let url = UrlBuilder::create_connect_url(url, conn)
    }

    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
