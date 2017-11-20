#![feature(use_extern_macros)]
#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate futures;
mod message;
mod protocol;
mod hubproxy; 
mod connection; 
mod hubresult; 

#[cfg(test)]
mod tests {
    use serde_json;
    use message;
    use connection::HubConnection;
    use hubproxy::Proxy;
    use std::mem;

    #[test]
    fn test_message_serialization_to_json() {
        assert_eq!(serde_json::to_string(&message::Message::StreamItem {_type:0, invocationId:String::from("a"), item:serde_json::json!(1)}).unwrap(), 
                   "{\"StreamItem\":{\"_type\":0,\"invocationId\":\"a\",\"item\":1}}");
    }

    #[test]
    fn test_connection_create() {
        let connection = HubConnection::new (String::from("http://localhost:8080"));
        let proxy = connection.create_hub_proxy ();

        //let p = Proxy::from (&*proxy) ;
        proxy.on::<String> (String::from ("addMessage"), |s| {});
        proxy.invoke (String::from ("addMessage"));
    }
}
