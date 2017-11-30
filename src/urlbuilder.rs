use connection::Connection;

pub struct UrlBuilder;

impl UrlBuilder {
    pub fn create_base_url<'a>(connection : &'a Connection, 
                               command : &str,
                               transport : Option<&str>,
                               connection_data : &str) -> String {
        let mut url = String::new();
        url.push_str (connection.get_url().as_str());
        url.push_str (command);
        url.push ('?');
        UrlBuilder::append_client_protocol (&mut url, connection);
        UrlBuilder::append_transport (&mut url, transport);
        UrlBuilder::append_connection_data (&mut url, connection_data);
        UrlBuilder::append_connection_token (&mut url, Some(connection.get_connection_token().as_str()));
        url 
    }

    fn append_client_protocol (url : &mut String, connection : &Connection) {
        url.push_str ("clientProtocol=");
        url.push_str (connection.get_protocol().as_str());
        url.push ('&');
    }

    fn append_transport (url : &mut String, transport : Option<&str>) {
        if let Some(transport) = transport {
            url.push_str ("transport=");
            url.push_str (transport);
            url.push ('&');
        }
    }

    fn append_connection_data (url : &mut String, connection_data : &str) {
        url.push_str ("connectionData=");
        url.push_str (connection_data);
        url.push ('&');
    }

    fn append_connection_token (url : &mut String, connection_token : Option<&str>) {
        if let Some(connection_token) = connection_token {
            url.push_str ("connectionToken=");
            url.push_str (connection_token);
            url.push ('&');
        }
    }

    fn create_negotiate_url (connection : &Connection, connection_data : &str) -> String {
        UrlBuilder::create_base_url (connection, "negotiate", None, connection_data)
    }

    fn create_connect_url (connection : &Connection, transport : Option<&str>, connection_data : &str) -> String {
        //http://localhost:8080/signalr/connect?clientProtocol=1.4&transport=serverSentEvents&connectionData=[%7B%22Name%22:%22MyHub%22%7D]&connectionToken=AQAAANCMnd8BFdERjHoAwE%2FCl%2BsBAAAAJKIyAZXvi0e08Sl079QEAAAAAAACAAAAAAADZgAAwAAAABAAAACS4RdIo2SoYaPSfMgvcGE2AAAAAASAAACgAAAAEAAAAGZvAyT3V82W9ccsIVJY6bYoAAAAaFgu3M01wkQoR6yG5ePZ%2FjDnrhzhh5fwNaaABi3qD89zE6xEgF%2BPahQAAACD2D9WSLwmGHvzjdQ%2BK6je4ZX6KA%3D%3D
        UrlBuilder::create_base_url (connection, "connect", transport, connection_data)
    }
}
