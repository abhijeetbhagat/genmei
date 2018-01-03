use std;
use std::any::Any;
use futures::prelude::*;
//use futures::future::Future;
use connection::{Connection, HubConnection};
use message::{InvocationMessage, Message};
use serde_json::{self, Value};
use erased_serde::Serialize;
use subscription::Subscription;
use std::collections::HashMap;
use httpclient::HttpClient;
use serde;

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

    pub fn on(&mut self, event_name: String, f: Box<Fn()>) {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |l| {
            f();
        }));
    }

    //not part of HubProxy because it would cause E0038
    //this will now make the api usage awkward since the trait object now needs a downcast
    //TODO abhi: rework on moving this to an appropriate place
    pub fn on_1_arg<T>(&mut self, event_name: String, f: Box<Fn(T)>)
    where
        T: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v = std::mem::replace(&mut l[0], serde_json::json!(0));
            f(serde_json::from_value(v).unwrap());
        }));
    }

    pub fn on_2_args<T1, T2>(&mut self, event_name: String, f: Box<Fn(T1, T2)>)
    where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
            );
        }));
    }

    pub fn on_3_args<T1, T2, T3>(&mut self, event_name: String, f: Box<Fn(T1, T2, T3)>)
    where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
        T3: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            let v3 = std::mem::replace(&mut l[2], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
                serde_json::from_value(v3).unwrap(),
            );
        }));
    }

    pub fn on_4_args<T1, T2, T3, T4>(&mut self, event_name: String, f: Box<Fn(T1, T2, T3, T4)>)
    where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
        T3: 'static + serde::de::DeserializeOwned,
        T4: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            let v3 = std::mem::replace(&mut l[2], serde_json::json!(0));
            let v4 = std::mem::replace(&mut l[3], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
                serde_json::from_value(v3).unwrap(),
                serde_json::from_value(v4).unwrap(),
            );
        }));
    }

    pub fn on_5_args<T1, T2, T3, T4, T5>(
        &mut self,
        event_name: String,
        f: Box<Fn(T1, T2, T3, T4, T5)>,
    ) where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
        T3: 'static + serde::de::DeserializeOwned,
        T4: 'static + serde::de::DeserializeOwned,
        T5: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            let v3 = std::mem::replace(&mut l[2], serde_json::json!(0));
            let v4 = std::mem::replace(&mut l[3], serde_json::json!(0));
            let v5 = std::mem::replace(&mut l[4], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
                serde_json::from_value(v3).unwrap(),
                serde_json::from_value(v4).unwrap(),
                serde_json::from_value(v5).unwrap(),
            );
        }));
    }

    pub fn on_6_args<T1, T2, T3, T4, T5, T6>(
        &mut self,
        event_name: String,
        f: Box<Fn(T1, T2, T3, T4, T5, T6)>,
    ) where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
        T3: 'static + serde::de::DeserializeOwned,
        T4: 'static + serde::de::DeserializeOwned,
        T5: 'static + serde::de::DeserializeOwned,
        T6: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            let v3 = std::mem::replace(&mut l[2], serde_json::json!(0));
            let v4 = std::mem::replace(&mut l[3], serde_json::json!(0));
            let v5 = std::mem::replace(&mut l[4], serde_json::json!(0));
            let v6 = std::mem::replace(&mut l[5], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
                serde_json::from_value(v3).unwrap(),
                serde_json::from_value(v4).unwrap(),
                serde_json::from_value(v5).unwrap(),
                serde_json::from_value(v6).unwrap(),
            );
        }));
    }

    pub fn on_7_args<T1, T2, T3, T4, T5, T6, T7>(
        &mut self,
        event_name: String,
        f: Box<Fn(T1, T2, T3, T4, T5, T6, T7)>,
    ) where
        T1: 'static + serde::de::DeserializeOwned,
        T2: 'static + serde::de::DeserializeOwned,
        T3: 'static + serde::de::DeserializeOwned,
        T4: 'static + serde::de::DeserializeOwned,
        T5: 'static + serde::de::DeserializeOwned,
        T6: 'static + serde::de::DeserializeOwned,
        T7: 'static + serde::de::DeserializeOwned,
    {
        let mut subscription = self.subscribe(event_name);
        subscription.set(Box::new(move |mut l| {
            println!("callback invoked with args {}", l[0]);
            //TODO abhi: this is done to avoid shifting of elements from a Vec:remove()
            let v1 = std::mem::replace(&mut l[0], serde_json::json!(0));
            let v2 = std::mem::replace(&mut l[1], serde_json::json!(0));
            let v3 = std::mem::replace(&mut l[2], serde_json::json!(0));
            let v4 = std::mem::replace(&mut l[3], serde_json::json!(0));
            let v5 = std::mem::replace(&mut l[4], serde_json::json!(0));
            let v6 = std::mem::replace(&mut l[5], serde_json::json!(0));
            let v7 = std::mem::replace(&mut l[6], serde_json::json!(0));
            f(
                serde_json::from_value(v1).unwrap(),
                serde_json::from_value(v2).unwrap(),
                serde_json::from_value(v3).unwrap(),
                serde_json::from_value(v4).unwrap(),
                serde_json::from_value(v5).unwrap(),
                serde_json::from_value(v6).unwrap(),
                serde_json::from_value(v7).unwrap(),
            );
        }));
    }

    pub fn handle_message(&self, event_name: &String, args: Vec<Value>) {
        let subscription = self.subscriptions.get(event_name);
        subscription.unwrap().on_received(args);
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
