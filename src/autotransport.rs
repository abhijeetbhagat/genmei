use clienttransport::ClientTransport;
use futures::future::Future;
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;
use urlbuilder::UrlBuilder;
use connection::Connection;
use serversenteventstransport::ServerSentEventsTransport;
use longpollingtransport::LongPollingTransport;

type TransportList = Vec<Box<ClientTransport>>;

pub struct AutoTransport {
    http_client: Box<HttpClient>,
    transports: TransportList,
}

impl AutoTransport {
    pub fn new(http_client: Box<HttpClient>) -> Self {
        AutoTransport {
            http_client: http_client,
            transports: vec![ServerSentEventsTransport, LongPollingTransport],
        }
    }
}

impl ClientTransport for AutoTransport {
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
        protocol: &str,
    ) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
        //TODO abhi: iterate over all the transports until a connection is made
        let url = UrlBuilder::create_connect_url(
            url,
            Some("auto"),
            connection_data,
            connection_token,
            protocol
        ); 
        let response = self.http_client.get(url.as_str());
    }

    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
