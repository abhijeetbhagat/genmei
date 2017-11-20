use protocol::Protocol;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum Message {
    Negotiation ,
    Invocation { _type : i32, invocationId : String, nonblocking : bool, target : String, arguments : Vec<Value> },
    StreamInvocation { _type: i32, invocationId : String, target : String, arguments : Vec<Value> },
    StreamItem { _type : i32, invocationId : String, item : Value },
    Completion { _type : i32, invocationId : String, result : Value, error : String },
    CancelInvocation { _type : i32, invocationId : String },
    Ping
}

#[derive (Serialize, Deserialize)]
pub struct InvocationMessage {
    #[serde (rename = "I")]
    pub callback_id : String,
    #[serde (rename = "H")]
    pub hub : String,
    #[serde (rename = "M")]
    pub method : String, 
    #[serde (rename = "A")]
    pub args : Vec<Value>
}
