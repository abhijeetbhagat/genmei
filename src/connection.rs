use hubproxy::{HubProxy, Proxy};
use hubresult::HubResult;

trait HubConnect {
    fn register_callback (Fn(HubResult));
    fn remove_callback (id : String);
}

struct HubConnection {
    url : String
}

impl HubConnection {
    fn new (url : String) -> Self {
        HubConnection {
            url : url
        }
    }

    fn create_hub_proxy () -> Box<HubProxy> {
        Box::new (Proxy::new ())
    }
}
