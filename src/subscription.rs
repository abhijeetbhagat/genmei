use serde_json::Value;
use std::option::Option;
use std::marker::{Send, Sync};

pub struct Subscription {
    //TODO should accept an iterator and not vec so that any iterable can be passed
    received: Option<Box<Fn(Vec<Value>) + Send + Sync>>,
}

impl Subscription {
    pub fn new() -> Self {
        Subscription { received: None }
    }

    pub fn set(&mut self, f: Box<Fn(Vec<Value>) + Send + Sync>) {
        self.received = Some(f);
    }

    pub fn on_received(&self, items: Vec<Value>) {
        if self.received.is_some() {
            (self.received.as_ref().unwrap())(items);
        }
    }
}
