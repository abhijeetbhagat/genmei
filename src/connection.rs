use hubproxy::{HubProxy, Proxy};
use hubresult::HubResult;
use std::collections::HashMap;

trait HubConnect {
    fn register_callback (&mut self, fn(HubResult));
    fn remove_callback (&mut self, id : String);
}

struct HubConnection {
    url : String,
    callbacks_map : HashMap<String, fn(HubResult)>
}

impl HubConnection {
    fn new (url : String) -> Self {
        HubConnection {
            url : url,
            callbacks_map : HashMap::new()
        }
    }

    fn create_hub_proxy () -> Box<HubProxy> {
        Box::new (Proxy::new ())
    }
}

impl HubConnect for HubConnection {
    fn register_callback (&mut self, cb : fn(HubResult)) {
        self.callbacks_map.insert (String::from("1"), cb);
    }

    fn remove_callback (&mut self, id : String) {
        self.callbacks_map.remove (&id);
    }
}
