use std::any::Any;
use futures::prelude::*;
//use futures::future::Future;
use connection::HubConnection;
use message::{Message, InvocationMessage};
use serde_json::Value;
use erased_serde::Serialize;
use subscription::Subscription;
use std::collections::HashMap;

/* Due to E0038, we aren't using this trait
 * TODO abhi: need to revisit if necessary
 * pub trait HubProxy {
    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E>;
}*/

pub struct Proxy<'a> {
    connection : &'a HubConnection,
    hub_name : String,
    subscriptions : HashMap<String, Subscription>
}

impl<'a> Proxy<'a> {
    pub fn new (connection : &'a HubConnection, name : String) -> Self {
        Proxy {
            connection : connection,
            hub_name : name,
            subscriptions : HashMap::new()
        }
    }

    //not part of HubProxy because it would cause E0038
    //this will now make the api usage awkward since the trait object now needs a downcast
    //TODO abhi: rework on moving this to an appropriate place
    pub fn on<T> (&self, event_name : String, f : fn(T)) {

    }

    //converts a Box<HubProxy> to Proxy.
    //there's difficulty in implementing the From trait because downcast_ref works with -
    //static lifetimes only.
    /*pub fn from (a : &Any) -> &Proxy {
        a.downcast_ref::<Self>().unwrap()
    }*/

    pub fn invoke (&self,
                   method : String,
                   args : Vec<&Serialize>) -> Box<Future<Item=(), Error=()>> {
        //TODO abhi : remove macro after implementation
        unimplemented!();
        let mut _args = vec![];
        for a in args {
            _args.push (json! (a));
        }

        let message = InvocationMessage {
            callback_id : String::from ("9"),
            hub : self.hub_name.clone(),
            method : method,
            args : _args
        };

        let data = self.connection.json_serialize_object (&message).unwrap();
        self.connection.send (data);
    }

    pub fn subscribe (event : String) {

    }
}

