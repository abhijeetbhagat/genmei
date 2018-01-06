use hubproxy::Proxy;
use hubresult::HubResult;
use std::collections::HashMap;
use erased_serde::Serialize;
use serde;
use serde_json;
use serde_json::Result;
use futures::future::*;
use futures::future::Future;
use version::Version;
use message::InvocationMessage;
use transports::clienttransport::ClientTransport;
use transports::autotransport::AutoTransport;
use httpclient::DefaultHttpClient;
use std::ops::Deref;
use std::rc::Rc;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::sync::mpsc::channel;

pub trait Connection {
    fn get_url(&self) -> String;
    fn get_connection_token(&self) -> String;
    fn get_connection_data(&self) -> String;
    fn get_protocol(&self) -> String;
    fn json_serialize(&self, &InvocationMessage) -> String;
    fn get_transport(&self) -> &ClientTransport;
    fn send(&mut self, data: String);
    fn start(&mut self) -> Box<Future<Item = (), Error = ()>>;
}

trait HubConnect {
    fn register_callback(&mut self, fn(HubResult));
    fn remove_callback(&mut self, id: String);
}

pub struct HubConnection {
    url: String,
    use_default_url: bool,
    query_string: String,
    query_string_map: HashMap<String, String>,
    callbacks_map: HashMap<String, fn(HubResult)>,
    proxies_map: HashMap<String, Rc<RefCell<Proxy>>>,
    pub headers: HashMap<String, String>,
    on_received: Option<Box<Fn(String)>>,
    on_closed: Option<Box<Fn(String)>>,
    on_connectionslow: Option<Box<Fn(String)>>,
    on_reconnecting: Option<Box<Fn(String)>>,
    on_reconnected: Option<Box<Fn(String)>>,
    on_statechanged: Option<Box<Fn(String)>>,
    protocol: Version,
    connection_token: String,
    connection_id: String,
    client_transport: Option<Box<ClientTransport>>,
}

impl HubConnection {
    /*pub fn new (url : String) -> Self {
        HubConnection {
            url : url,
            callbacks_map : HashMap::new()
        }
    }*/

    pub fn create_hub_proxy(&mut self, hub_name: String) -> Rc<RefCell<Proxy>> {
        //self.hub_name = hub_name.clone();
        let proxy = Proxy::new(/*self,*/ hub_name.clone());
        self.proxies_map
            .insert(hub_name.clone(), Rc::new(RefCell::new(proxy)));
        Rc::clone(self.proxies_map.get(&hub_name).unwrap())
    }

    /*pub fn start<T, E> (&self) -> FutureResult<T, E> {
        //TODO abhi initiate a connection to the server here
        unimplemented!();
    }*/

    pub fn json_serialize_object<T: serde::Serialize>(&self, object: &T) -> Result<String> {
        serde_json::to_string(object)
    }

    pub fn on_received(&mut self, handler: Box<Fn(String)>) {
        self.on_received = Some(handler);
    }

    pub fn on_closed(&mut self, handler: Box<Fn(String)>) {
        self.on_closed = Some(handler);
    }

    pub fn on_connectionslow(&mut self, handler: Box<Fn(String)>) {
        self.on_connectionslow = Some(handler);
    }

    pub fn on_reconnecting(&mut self, handler: Box<Fn(String)>) {
        self.on_reconnecting = Some(handler);
    }

    pub fn on_reconnected(&mut self, handler: Box<Fn(String)>) {
        self.on_reconnected = Some(handler);
    }

    pub fn on_statechanged(&mut self, handler: Box<Fn(String)>) {
        self.on_statechanged = Some(handler);
    }

    fn start_transport(&mut self) {
        let url = self.get_url();
        let protocol = self.get_protocol();
        let connection_data = self.get_connection_data();
        let connection_token = self.get_connection_token();
        let (tx, rx) = channel();
        let response = self.client_transport
            .as_mut()
            .unwrap()
            .start(
                url.as_str(),
                connection_data.as_str(),
                connection_token.as_str(),
                protocol.as_str(),
                Some(tx),
            )
            .map(|r| r)
            .wait()
            .unwrap();

        //self.process_response(response);
        loop {
            let vec = rx.recv().unwrap();
            println!("oc: chunk: {:?}", vec);

            if vec.len() > 19 {
                use std;
                let data = std::str::from_utf8(&vec).unwrap();
                /*{
                    "C": "d-A2D08C-B,1|C,0|D,1",
                    "M": [
                            {
                            "H": "MyHub",
                            "M": "send",
                            "A": [
                            "client message"
                            ]
                            }
                    ]
                }*/
                if data.contains("data:") {
                    //we do not deal with "data:{}"
                    let mut map: Map<String, Value> = serde_json::from_str(&data[5..]).unwrap();
                    if map.contains_key(&String::from("S"))
                        && map.get(&String::from("S")).unwrap().as_u64().unwrap() == 1u64
                    {
                        //TODO abhi: initiate a 'start' request
                    }
                    //TODO abhi: this whole thing needs to be re-written
                    if let Some(messages) = map.remove(&String::from("M")) {
                        let messages = messages.as_array().unwrap();
                        for mut message in messages {
                            let hub = message[&String::from("H")].as_str().unwrap();
                            println!("{:?}", hub);
                            let hub = &String::from(hub);
                            if self.proxies_map.contains_key(hub) {
                                let proxy = &self.proxies_map[hub];
                                let method = message[&String::from("M")].as_str().unwrap();
                                let method = &String::from(method);
                                println!("{:?}", method);
                                let args = &message[&String::from("A")];
                                println!("{:?}", args);
                                proxy
                                    .borrow()
                                    .handle_message(method, args.as_array().unwrap().clone());
                            }
                        }
                    }
                }
            }
        } //loop ends
    }

    fn process_response(&mut self, response: Map<String, Value>) {
        panic!("{:?}", response);
        if response.contains_key(&String::from("I")) {
            println!("{}", response.get(&String::from("I")).unwrap());
        }
    }
}

impl Connection for HubConnection {
    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_protocol(&self) -> String {
        self.protocol.to_string()
    }

    fn get_connection_token(&self) -> String {
        self.connection_token.clone()
    }

    fn get_connection_data(&self) -> String {
        //TODO abhi use this line when proxies_map is used
        //For now, just use the hub_name
        //String::from()
        format!(
            "[%7B%22Name%22:%22{}%22%7D]",
            self.proxies_map.keys().take(1).next().unwrap()
        )
    }

    fn json_serialize(&self, message: &InvocationMessage) -> String {
        self.json_serialize_object(message).unwrap()
    }

    fn send(&mut self, data: String) {
        let url = self.get_url();
        let protocol = self.get_protocol();
        let connection_data = self.get_connection_data();
        let connection_token = self.get_connection_token();
        self.client_transport.as_mut().unwrap().send(
            url.as_str(),
            connection_data.as_str(),
            connection_token.as_str(),
            protocol.as_str(),
            data,
        );
    }

    fn get_transport(&self) -> &ClientTransport {
        self.client_transport.as_ref().unwrap().deref()
    }

    fn start(&mut self) -> Box<Future<Item = (), Error = ()>> {
        let url = self.get_url();
        let protocol = self.get_protocol();
        let connection_data = self.get_connection_data();
        self.client_transport
            .as_mut()
            .unwrap()
            .negotiate(url.as_str(), connection_data.as_str(), protocol.as_str())
            .map(|r| {
                self.connection_token = r.connection_token;
                self.connection_id = r.connection_id;
                self.start_transport()
            })
            .wait(); //TODO abhi: remove wait(); this is called only for testing
        Box::new(result(Ok(())))
        /*Box::new(self.client_transport.as_mut().unwrap().negotiate().map(|response|{
            self.connection_token = response.connection_token;
            ()
        })) */
    }

    //fn negotiate
}

impl HubConnect for HubConnection {
    fn register_callback(&mut self, cb: fn(HubResult)) {
        self.callbacks_map.insert(String::from("1"), cb);
    }

    fn remove_callback(&mut self, id: String) {
        self.callbacks_map.remove(&id);
    }
}

pub struct HubConnectionBuilder {
    url: String,
    use_default_url: bool,
    query_string: Option<String>,
    query_string_map: Option<HashMap<String, String>>,
}

impl HubConnectionBuilder {
    pub fn new(url: String) -> Self {
        HubConnectionBuilder {
            url,
            use_default_url: false,
            query_string: None,
            query_string_map: None,
        }
    }

    pub fn with_query_string(mut self, query_string: String) -> HubConnectionBuilder {
        self.query_string = Some(query_string);
        self
    }

    pub fn use_default_url(mut self, use_default_url: bool) -> HubConnectionBuilder {
        self.use_default_url = use_default_url;
        self
    }

    pub fn with_query_map(
        mut self,
        query_string_map: HashMap<String, String>,
    ) -> HubConnectionBuilder {
        self.query_string_map = Some(query_string_map);
        self
    }

    //TODO abhi: do we want to consume the builder?
    pub fn finish(self) -> HubConnection {
        HubConnection {
            url: self.url,
            use_default_url: self.use_default_url,
            query_string: self.query_string.unwrap_or(String::from("")),
            query_string_map: self.query_string_map.unwrap_or(HashMap::new()),
            callbacks_map: HashMap::new(),
            proxies_map: HashMap::new(),
            headers: HashMap::new(),
            on_received: None,
            on_closed: None,
            on_connectionslow: None,
            on_reconnecting: None,
            on_reconnected: None,
            on_statechanged: None,
            protocol: Version::new(1, 4), //TODO abhi: should this be read from a config file?
            connection_token: String::new(),
            connection_id: String::new(),
            client_transport: Some(Box::new(AutoTransport::new(Box::new(
                DefaultHttpClient::new(),
            )))),
        }
    }
}
