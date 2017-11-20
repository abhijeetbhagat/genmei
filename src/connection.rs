use hubproxy::{Proxy};
use hubresult::HubResult;
use std::collections::HashMap;
use serde::ser::Serialize;
use serde_json;
use serde_json::Result;

trait HubConnect {
    fn register_callback (&mut self, fn(HubResult));
    fn remove_callback (&mut self, id : String);
}

pub struct HubConnection {
    url : String,
    callbacks_map : HashMap<String, fn(HubResult)>
}

impl HubConnection {
    pub fn new (url : String) -> Self {
        HubConnection {
            url : url,
            callbacks_map : HashMap::new()
        }
    }

    pub fn create_hub_proxy (&self, hub_name : String) -> Proxy {
        Proxy::new (self, hub_name)
    }

    pub fn send(&self, data : String) {

    }

    fn start (&self) {
        //TODO abhi initiate a connection to the server here

    }

    pub fn json_serialize_object<T : Serialize> (&self, object : &T) -> Result<String> {
        serde_json::to_string (object)
    }
}

impl HubConnect for HubConnection {
    fn register_callback (&mut self, cb : fn(HubResult)) {
        self.callbacks_map.insert (String::from("1"), cb);
    }

    fn remove_callback (&mut self, id : String) {
        self.callbacks_map.remove (&id);
    }
}
