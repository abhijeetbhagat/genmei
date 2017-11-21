use hubproxy::{Proxy};
use hubresult::HubResult;
use std::collections::HashMap;
use serde::ser::Serialize;
use serde_json;
use serde_json::Result;
use futures::future::FutureResult;

trait HubConnect {
    fn register_callback (&mut self, fn(HubResult));
    fn remove_callback (&mut self, id : String);
}

pub struct HubConnection {
    url : String,
    use_default_url : bool,
    query_string : String,
    query_string_map : HashMap<String, String>,
    callbacks_map : HashMap<String, fn(HubResult)>
}

impl HubConnection {
    /*pub fn new (url : String) -> Self {
        HubConnection {
            url : url,
            callbacks_map : HashMap::new()
        }
    }*/

    pub fn create_hub_proxy (&self, hub_name : String) -> Proxy {
        Proxy::new (self, hub_name)
    }

    pub fn send(&self, data : String) {

    }

    fn start<T, E> (&self) -> FutureResult<T, E> {
        //TODO abhi initiate a connection to the server here
        unimplemented!();

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

pub struct HubConnectionBuilder {
    url : String,
    use_default_url : bool,
    query_string : Option<String>,
    query_string_map : Option<HashMap<String, String>>
}

impl HubConnectionBuilder {
    fn new(url : String) -> Self {
        HubConnectionBuilder {
            url : url, 
            use_default_url : false,
            query_string : None,
            query_string_map : None
        }
    }

    fn with_query_string (mut self, query_string : String) -> HubConnectionBuilder {
        self.query_string = Some(query_string);
        self
    }

    fn use_default_url (mut self, use_default_url : bool) -> HubConnectionBuilder {
        self.use_default_url = use_default_url;
        self
    }

    fn with_query_map (mut self,
                       query_string_map : HashMap<String, String>) -> HubConnectionBuilder {
        self.query_string_map = Some (query_string_map);
        self
    }

    //TODO abhi: do we want to consume the builder?
    fn finish (self) -> HubConnection {
        HubConnection {
            url : self.url,
            use_default_url : self.use_default_url,
            query_string : self.query_string.unwrap(),
            query_string_map : self.query_string_map.unwrap(),
            callbacks_map : HashMap::new()

        }
    }
}
