use std::any::Any;
use futures::future::FutureResult;
use connection::HubConnection;
use message::{Message, InvocationMessage};

/* Due to E0038, we aren't using this trait
 * TODO abhi: need to revisit if necessary
 * pub trait HubProxy {
    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E>;
}*/

pub struct Proxy<'a> {
    connection : &'a HubConnection,
    hub_name : String
}

impl<'a> Proxy<'a> {
    pub fn new (connection : &'a HubConnection, name : String) -> Self {
        Proxy {
            connection : connection,
            hub_name : name
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

    pub fn invoke<T, E> (&self, method : String) -> FutureResult<T, E> {
        //TODO abhi : remove macro after implementation
        unimplemented!();
        let message = InvocationMessage {
            callback_id : String::from ("9"),
            hub : self.hub_name.clone(),
            method : method,
            args : vec![]
        };

        let data = self.connection.json_serialize_object (&message).unwrap();
        self.connection.send (data);
    }
}

