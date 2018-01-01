use std::any::Any;
use futures::prelude::*;
//use futures::future::Future;
use connection::{Connection, HubConnection};
use message::{InvocationMessage, Message};
use serde_json::Value;
use erased_serde::Serialize;
use subscription::Subscription;
use std::collections::HashMap;
use httpclient::HttpClient;

/* Due to E0038, we aren't using this trait
 * TODO abhi: need to revisit if necessary
 * pub trait HubProxy {
    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E>;
}*/

pub struct Proxy {
    //connection : &'a Connection,
    hub_name: String,
    subscriptions: HashMap<String, Subscription>,
}

impl Proxy {
    pub fn new(/*connection : &Connection,*/ name: String) -> Self {
        //let url = connection.get_url().to_string();
        let hub_name = name;
        let hub_name1 = hub_name.clone();
        Proxy {
            //connection : connection,
            hub_name,
            subscriptions: HashMap::new(),
        }
    }

    //not part of HubProxy because it would cause E0038
    //this will now make the api usage awkward since the trait object now needs a downcast
    //TODO abhi: rework on moving this to an appropriate place
    pub fn on<T>(&mut self, event_name: String, f: Box<Fn(T)>) {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(|l| { }));
    }

    //converts a Box<HubProxy> to Proxy.
    //there's difficulty in implementing the From trait because downcast_ref works with -
    //static lifetimes only.
    /*pub fn from (a : &Any) -> &Proxy {
        a.downcast_ref::<Self>().unwrap()
    }*/

    pub fn invoke(&self, method: String, args: Vec<&Serialize>) /*-> Box<Future<Item=(), Error=()>>*/
    {
        //TODO abhi : remove macro after implementation
        unimplemented!();
        let mut _args = vec![];
        for a in args {
            _args.push(json!(a));
        }

        let message = InvocationMessage {
            callback_id: String::from("9"),
            hub: self.hub_name.clone(),
            method: method,
            args: _args,
        };

        //let data = self.connection.json_serialize(&message);
        //self.connection.send (data);
    }

    pub fn subscribe(&mut self, event: String) -> &mut Subscription {
        if !self.subscriptions.contains_key(&event) {
            self.subscriptions
                .insert(event.clone(), Subscription::new());
        }
        self.subscriptions.get_mut(&event).unwrap()
    }
}
