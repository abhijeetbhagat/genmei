use std::any::Any;
use futures::future::FutureResult;

/* Due to E0038, we aren't using this trait
 * TODO abhi: need to revisit if necessary
 * pub trait HubProxy {
    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E>;
}*/

pub struct Proxy {
    
}

impl Proxy {
    pub fn new () -> Self {
        Proxy {}
    }

    //not part of HubProxy because it would cause E0038
    //this will now make the api usage awkward since the trait object now needs a downcast
    //TODO abhi: rework on moving this to an appropriate place
    pub fn on<T> (&self, event_name : String, f : fn(T)) {

    }

    //converts a Box<HubProxy> to Proxy.
    //there's difficulty in implementing the From trait because downcast_ref works with -
    //static lifetimes only.
    pub fn from (a : &Any) -> &Proxy {
        a.downcast_ref::<Self>().unwrap()
    }

    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E> {
        unimplemented!();

    }
}

/* Due to E0038, no trait used
 * impl HubProxy for Proxy { 
    fn invoke<T, E> (&self, method : String) -> FutureResult<T, E> {
        
    } 
}*/
