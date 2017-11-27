#![feature(use_extern_macros)]
#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate erased_serde;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
mod message;
mod protocol;
mod hubproxy; 
mod connection; 
mod hubresult; 
mod httpbasedtransport; 
mod httpclient;
mod subscription;
mod urlbuilder;
mod version;
mod negotiationresponse;


#[cfg(test)]
mod tests {
    use serde_json;
    use message;
    use message::{InvocationMessage};
    use connection::{HubConnection, HubConnectionBuilder};
    use hubproxy::Proxy;
    use std::mem;
    use futures::future::Future;
    use version::Version;
    use negotiationresponse::NegotiationResponse;
    use httpclient::HttpClient;
    use hyper;


    //http://localhost:8080/signalr/negotiate?clientProtocol=1.4&connectionData=[%7B%22Name%22:%22MyHub%22%7D]
    /*{
        "Url": "/signalr",
        "ConnectionToken": "AQAAANCMnd8BFdERjHoAwE/Cl+sBAAAAJKIyAZi0e08Sl079QEAAAAAAACAAAAAAADZgAAwAAAABAAAACS4RdIo2SoYaPSfMgvcGE2AAAAAASAAACgAAAAEAAAAGZvAyT3V82W9ccsIVJY6bYoAAAAaFgu3M01wkQoR6yG5ePZ/jDnrhzhh5fwNaaABi3qD89zE6xEgF+PahQAAACD2D9WSLwmGHvzjdQ+K6je4ZX6KA==",
        "ConnectionId": "d9eb13e8-aabd-4184-964e-52570ba54663",
        "KeepAliveTimeout": 20,
        "DisconnectTimeout": 30,
        "ConnectionTimeout": 110,
        "TryWebSockets": false,
        "ProtocolVersion": "1.4",
        "TransportConnectTimeout": 5,
        "LongPollDelay": 0
    }*/
    //http://localhost:8080/signalr/send?clientProtocol=1.4&transport=serverSentEvents&connectionData=[%7B%22Name%22:%22MyHub%22%7D]&connectionToken=AQAAANCMnd8BFdERjHoAwE%2FCl%2BsBAAAAJKIyAZXvi0e08Sl079QEAAAAAAACAAAAAAADZgAAwAAAABAAAABKuV%2Bxe15SC20qoS1GIkm0AAAAAASAAACgAAAAEAAAANuqwbda%2FDjBwm7ikQKzgCwoAAAAkgvwaH5thyZv8X9ug41XupjSvsRPTX9XV0Np2QnUA3xpEI6mtigCXRQAAADTkkV58tskB3sVw1IBT%2FoxWDt8IQ%3D%3D
    #[test]
    fn test_message_serialization_to_json() {
        assert_eq!(serde_json::to_string(&message::Message::StreamItem {_type:0, invocationId:String::from("a"), item:serde_json::json!(1)}).unwrap(), 
                   "{\"StreamItem\":{\"_type\":0,\"invocationId\":\"a\",\"item\":1}}");
    }

    #[test]
    fn test_connection_create() {
        let connection = HubConnectionBuilder::new (String::from("http://localhost:8080"))
                            .use_default_url (false)
                            .finish();
        let mut proxy = connection.create_hub_proxy (String::from ("MyHub"));

        proxy.on::<String> (String::from ("addMessage"), Box::new (|s| {}));
        proxy.invoke (String::from ("addMessage"), vec![&String::from ("abhi"), &1]);
        connection.start::<(), ()>().wait();
    }

    #[test]
    fn test_invocation_message_serialize () {
        let message = InvocationMessage {
            callback_id : String::from ("9"),
            hub : String::from ("MyHub"),
            method : String::from ("send"),
            args : vec![]
        };
        assert_eq! (serde_json::to_string (&message).unwrap(), 
                    "{\"I\":\"9\",\"H\":\"MyHub\",\"M\":\"send\",\"A\":[]}");
    }

    #[test]
    fn test_connection_headers_set() {
        let mut connection = HubConnectionBuilder::new (String::from("http://localhost:8080"))
            .use_default_url (false)
            .finish();
        connection.headers.insert(String::from("header"), String::from("value"));
    }

    #[test]
    fn test_on_received_handler() {
        let mut connection = HubConnectionBuilder::new (String::from("http://localhost:8080"))
            .finish();
        connection.on_received (Box::new (|s| { assert_eq! (s, String::from ("Hello from server"));}));
    }

    #[test]
    fn test_versions() {
        assert_eq!(Version::new(1,4), Version::new(1,4));
        assert_ne!(Version::new(1,4), Version::new(1,3));
        assert!(Version::new(1,4) < Version::new(1,5));
        assert!(Version::new(1,4) < Version::new(2,5));
        assert!(Version::new(1,4) > Version::new(1,3));
        assert!(Version::new(1,4) >= Version::new(1,3));
    }

    #[test]
    fn test_deserialize_negotiationresponse() {
        let j = "{
        \"Url\": \"/signalr\",
        \"ConnectionToken\": \"AQAAANCMnd8BFdERjHoAwE/Cl+sBAAAAJKIyAZi0e08Sl079QEAAAAAAACAAAAAAADZgAAwAAAABAAAACS4RdIo2SoYaPSfMgvcGE2AAAAAASAAACgAAAAEAAAAGZvAyT3V82W9ccsIVJY6bYoAAAAaFgu3M01wkQoR6yG5ePZ/jDnrhzhh5fwNaaABi3qD89zE6xEgF+PahQAAACD2D9WSLwmGHvzjdQ+K6je4ZX6KA==\",
        \"ConnectionId\": \"d9eb13e8-aabd-4184-964e-52570ba54663\",
        \"KeepAliveTimeout\": 20,
        \"DisconnectTimeout\": 30,
        \"ConnectionTimeout\": 110,
        \"TryWebSockets\": false,
        \"ProtocolVersion\": \"1.4\",
        \"TransportConnectTimeout\": 5,
        \"LongPollDelay\": 0
        }";
       
        let n : NegotiationResponse = serde_json::from_str(j).unwrap();
        assert_eq! (n.url, "/signalr");

    }

    #[test]
    fn test_http_client(){
        let mut http_client = HttpClient::new();
        let uri = "https://www.rust-lang.org/en-US/".parse().unwrap();
        let work = http_client.client.get(uri).map(|res|{
            assert_eq!(res.status(),hyper::StatusCode::Ok);
            println!("{}",res.status());
        });
        http_client.core.run(work);
    }
}
