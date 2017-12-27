use transports::clienttransport::ClientTransport;
use futures::future::{ok, Future};
use negotiationresponse::NegotiationResponse;
use httpclient::HttpClient;
use urlbuilder::UrlBuilder;
use connection::Connection;
use transports::serversenteventstransport::ServerSentEventsTransport;
use transports::longpollingtransport::LongPollingTransport;
use serde_json;
use serde_json::{Map, Value};

type TransportList = Vec<Box<ClientTransport>>;

pub struct AutoTransport {
    http_client: Box<HttpClient>,
    transports: TransportList,
}

impl AutoTransport {
    pub fn new(http_client: Box<HttpClient>) -> Self {
        AutoTransport {
            http_client: http_client,
            transports: vec![
                Box::new(ServerSentEventsTransport::new()),
                Box::new(LongPollingTransport),
            ],
        }
    }

    fn resolve_transport(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
        i: usize,
    ) -> Box<Future<Item = Map<String, Value>, Error = ()>> {
        {
            if i < self.transports.len() {
                let transport = &mut self.transports[i];
                //TODO abhi: check error returned from start() and try another transport
                return transport.start(url, connection_data, connection_token, protocol);
            }
        }
        self.resolve_transport(url, connection_data, connection_token, protocol, i + 1)
    }
}

impl ClientTransport for AutoTransport {
    fn negotiate(
        &mut self,
        url: &str,
        connection_data: &str,
        protocol: &str,
    ) -> Box<Future<Item = NegotiationResponse, Error = ()>> {
        let url = UrlBuilder::create_negotiate_url(url, connection_data, protocol);
        //TODO abhi: get should return a future; so process accordingly
        let response = self.http_client.get(url.as_str());
        let response = serde_json::from_str(&response).unwrap();
        Box::new(ok::<_, _>(response))
    }

    fn start(
        &mut self,
        url: &str,
        connection_data: &str,
        connection_token: &str,
        protocol: &str,
    ) -> Box<Future<Item = Map<String, Value>, Error = ()>> {
        self.resolve_transport(url, connection_data, connection_token, protocol, 0)
    }


    fn send(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }

    fn abort(&self) -> Box<Future<Item = (), Error = ()>> {
        unimplemented!();
    }
}
