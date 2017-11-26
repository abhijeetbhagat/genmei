use hubproxy::{Proxy};
use hubresult::HubResult;
use std::collections::HashMap;
use serde::ser::Serialize;
use serde_json;
use serde_json::Result;
use futures::future::FutureResult;
use version::Version;

trait HubConnect {
    fn register_callback (&mut self, fn(HubResult));
    fn remove_callback (&mut self, id : String);
}

pub struct HubConnection {
    url : String,
    use_default_url : bool,
    query_string : String,
    query_string_map : HashMap<String, String>,
    callbacks_map : HashMap<String, fn(HubResult)>,
    pub headers : HashMap<String, String>,
    on_received : Option<Box<Fn(String)>>,
    on_closed : Option<Box<Fn(String)>>,
    on_connectionslow : Option<Box<Fn(String)>>,
    on_reconnecting : Option<Box<Fn(String)>>,
    on_reconnected : Option<Box<Fn(String)>>,
    on_statechanged : Option<Box<Fn(String)>>,
    protocol : Version,
    connection_token : String
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

    pub fn start<T, E> (&self) -> FutureResult<T, E> {
        //TODO abhi initiate a connection to the server here
        unimplemented!();

    }

    pub fn json_serialize_object<T : Serialize> (&self, object : &T) -> Result<String> {
        serde_json::to_string (object)
    }

    pub fn on_received (&mut self, handler : Box<Fn(String)>) {
        self.on_received = Some(handler);
    }

    pub fn on_closed (&mut self, handler : Box<Fn(String)>) {
        self.on_closed = Some(handler);
    }

    pub fn on_connectionslow (&mut self, handler : Box<Fn(String)>) {
        self.on_connectionslow = Some(handler);
    }

    pub fn on_reconnecting (&mut self, handler : Box<Fn(String)>) {
        self.on_reconnecting = Some(handler);
    }

    pub fn on_reconnected (&mut self, handler : Box<Fn(String)>) {
        self.on_reconnected = Some(handler);
    }

    pub fn on_statechanged (&mut self, handler : Box<Fn(String)>) {
        self.on_statechanged = Some(handler);
    }

    pub fn get_url (&self) -> &String {
        &self.url
    }

    pub fn get_protocol (&self) -> String {
        self.protocol.to_string()
    }

    pub fn get_connection_token (&self) -> &String {
        &self.connection_token
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
    pub fn new(url : String) -> Self {
        HubConnectionBuilder {
            url : url, 
            use_default_url : false,
            query_string : None,
            query_string_map : None
        }
    }

    pub fn with_query_string (mut self, query_string : String) -> HubConnectionBuilder {
        self.query_string = Some(query_string);
        self
    }

    pub fn use_default_url (mut self, use_default_url : bool) -> HubConnectionBuilder {
        self.use_default_url = use_default_url;
        self
    }

    pub fn with_query_map (mut self,
                       query_string_map : HashMap<String, String>) -> HubConnectionBuilder {
        self.query_string_map = Some (query_string_map);
        self
    }

    //TODO abhi: do we want to consume the builder?
    pub fn finish (self) -> HubConnection {
        HubConnection {
            url : self.url,
            use_default_url : self.use_default_url,
            query_string : self.query_string.unwrap_or (String::from ("")),
            query_string_map : self.query_string_map.unwrap_or ( HashMap::new()),
            callbacks_map : HashMap::new(),
            headers : HashMap::new(),
            on_received : None,
            on_closed : None,
            on_connectionslow : None,
            on_reconnecting : None,
            on_reconnected : None,
            on_statechanged : None,
            protocol : Version::new (1, 4), //TODO abhi: should this be read from a config file?
            connection_token : String::new()
        }
    }
}
