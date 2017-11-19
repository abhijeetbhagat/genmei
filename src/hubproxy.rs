pub trait HubProxy {
    fn invoke (&self, method : String);
}

pub struct Proxy {
    
}

impl Proxy {
    pub fn new () -> Self {
        Proxy {}
    }

    //not part of HubProxy because it would cause E0038
    //this will now make the api usage awkward since the trait object now needs a downcast
    //TODO abhi: rework on moving this to an appropriate place
    fn on<T> (&self, event_name : String, f : fn(T)) {

    }
}

impl HubProxy for Proxy { 
    fn invoke (&self, method : String) {
        
    } 
}
