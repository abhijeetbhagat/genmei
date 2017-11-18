#![feature(use_extern_macros)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
mod message;
mod protocol;
mod hubproxy; 
mod connection; 
mod hubresult; 

#[cfg(test)]
mod tests {
    use serde_json;
    use message;
    #[test]
    fn test_message_serialization_to_json() {
        assert_eq!(serde_json::to_string(&message::Message::StreamItem {_type:0, invocationId:String::from("a"), item:serde_json::json!(1)}).unwrap(), 
                   "{\"StreamItem\":{\"_type\":0,\"invocationId\":\"a\",\"item\":1}}");
    }
}
