use serde_json::Value;
use std::option::Option;

pub struct Subscription {
    //TODO should accept an iterator and not vec so that any iterable can be passed
    received : Option<fn(Vec<Value>) -> ()>
}

impl Subscription {
    fn new () -> Self {
        Subscription {
            received : None
        }
    }

    fn set (&mut self, f : fn(Vec<Value>)) {
        self.received = Some (f);
    }

    fn on_received (&self, items : Vec<Value>) {
        if self.received.is_some() {
            self.received.unwrap() (items);
        }
    }
}
