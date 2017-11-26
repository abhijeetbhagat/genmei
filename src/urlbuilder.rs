use connection::HubConnection;

pub struct UrlBuilder;

impl UrlBuilder {
    pub fn create_base_url<'a>(connection : &'a HubConnection, command : &str) -> String {
        let mut url = String::new();
        url.push_str (connection.get_url().as_str());
        url.push_str (command);
        url.push ('?');
        UrlBuilder::append_client_protocol (&mut url, connection);
        UrlBuilder::append_transport (&mut url, "serverSentEvents");
        url 
    }

    fn append_client_protocol (url : &mut String, connection : &HubConnection) {
        url.push_str ("clientProtocol=");
        url.push_str (connection.get_protocol().as_str());
        url.push ('&');
    }

    fn append_transport (url : &mut String, transport : &str) {
        url.push_str ("transport=");
        url.push_str (transport);
        url.push ('&');
    }

}
