pub trait HubProxy {
    fn invoke (&self, method : String);
}

pub struct Proxy {
    
}

impl Proxy {
    pub fn new () -> Self {
        Proxy {}
    }
}

impl HubProxy for Proxy { 
    fn invoke (&self, method : String) {

    } 
}
