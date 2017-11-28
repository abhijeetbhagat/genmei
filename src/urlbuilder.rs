use connection::Connection;

pub struct UrlBuilder;

impl UrlBuilder {
    pub fn create_base_url<'a>(connection : &'a Connection, 
                               command : &str,
                               transport : &str,
                               connection_data : &str) -> String {
        let mut url = String::new();
        url.push_str (connection.get_url().as_str());
        url.push_str (command);
        url.push ('?');
        UrlBuilder::append_client_protocol (&mut url, connection);
        UrlBuilder::append_transport (&mut url, transport);
        UrlBuilder::append_connection_data (&mut url, connection.get_connection_token().as_str());
        url 
    }

    fn append_client_protocol (url : &mut String, connection : &Connection) {
        url.push_str ("clientProtocol=");
        url.push_str (connection.get_protocol().as_str());
        url.push ('&');
    }

    fn append_transport (url : &mut String, transport : &str) {
        url.push_str ("transport=");
        url.push_str (transport);
        url.push ('&');
    }

    fn append_connection_data (url : &mut String, connection_data : &str) {
        url.push_str ("connectionData=");
        url.push_str (connection_data);
        url.push ('&');
    }

    fn append_connection_token (url : &mut String, connection_token : &str) {
        url.push_str ("connectionToken=");
        url.push_str (connection_token);
        url.push ('&');

    }
}
