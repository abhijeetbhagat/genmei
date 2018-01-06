use connection::Connection;

pub struct UrlBuilder;

//http://localhost:8080negotiate?clientProtocol=1.4&connectionData=[%7B%22Name%22:%22%22%7D]&    pub fn create_base_url<'a>(
impl UrlBuilder {
    pub fn create_base_url<'a>(
        url: &str,
        command: &str,
        transport: Option<&str>,
        connection_data: &str,
        connection_token: Option<&str>,
        protocol: &str,
    ) -> String {
        let mut _url = String::new();
        _url.push_str(url.trim());
        if _url.as_bytes()[_url.len() - 1] != '/' as u8 {
            _url.push('/')
        }
        _url.push_str(command);
        _url.push('?');
        UrlBuilder::append_client_protocol(&mut _url, protocol);
        UrlBuilder::append_transport(&mut _url, transport);
        UrlBuilder::append_connection_data(&mut _url, connection_data);
        UrlBuilder::append_connection_token(&mut _url, connection_token);
        _url.pop(); //remove the trailing '&'
        _url
    }

    fn append_client_protocol(url: &mut String, protocol: &str) {
        url.push_str("clientProtocol=");
        url.push_str(protocol);
        url.push('&');
    }

    fn append_transport(url: &mut String, transport: Option<&str>) {
        if let Some(transport) = transport {
            url.push_str("transport=");
            url.push_str(transport);
            url.push('&');
        }
    }

    fn append_connection_data(url: &mut String, connection_data: &str) {
        url.push_str("connectionData=");
        url.push_str(connection_data);
        url.push('&');
    }

    fn append_connection_token(url: &mut String, connection_token: Option<&str>) {
        if let Some(connection_token) = connection_token {
            url.push_str("connectionToken=");
            url.push_str(UrlBuilder::escape_token(connection_token).as_str());
            url.push('&');
        }
    }

    fn escape_token(connection_token: &str) -> String {
        let mut escaped_token = String::with_capacity(connection_token.len());
        for c in connection_token.chars() {
            match c {
                '/' => escaped_token.push_str("%2F"),
                '+' => escaped_token.push_str("%2B"),
                '=' => escaped_token.push_str("%3D"),
                _ => escaped_token.push(c),
            }
        }
        escaped_token
    }

    pub fn create_negotiate_url(url: &str, connection_data: &str, protocol: &str) -> String {
        UrlBuilder::create_base_url(url, "negotiate", None, connection_data, None, protocol)
    }

    pub fn create_connect_url(
        url: &str,
        transport: Option<&str>,
        connection_data: &str,
        connection_token: Option<&str>,
        protocol: &str,
    ) -> String {
        //http://localhost:8080/signalr/connect?clientProtocol=1.4&transport=serverSentEvents&connectionData=[%7B%22Name%22:%22MyHub%22%7D]&connectionToken=AQAAANCMnd8BFdERjHoAwE%2FCl%2BsBAAAAJKIyAZXvi0e08Sl079QEAAAAAAACAAAAAAADZgAAwAAAABAAAACS4RdIo2SoYaPSfMgvcGE2AAAAAASAAACgAAAAEAAAAGZvAyT3V82W9ccsIVJY6bYoAAAAaFgu3M01wkQoR6yG5ePZ%2FjDnrhzhh5fwNaaABi3qD89zE6xEgF%2BPahQAAACD2D9WSLwmGHvzjdQ%2BK6je4ZX6KA%3D%3D
        UrlBuilder::create_base_url(
            url,
            "connect",
            transport,
            connection_data,
            connection_token,
            protocol,
        )
    }

    pub fn create_send_url(
        url: &str,
        transport: Option<&str>,
        connection_data: &str,
        connection_token: Option<&str>,
        protocol: &str,
    ) -> String {
        UrlBuilder::create_base_url(
            url,
            "send",
            transport,
            connection_data,
            connection_token,
            protocol,
        )
    }
}
