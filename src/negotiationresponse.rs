#[derive(Serialize, Deserialize)]
pub struct NegotiationResponse {
    #[serde(rename = "Url")] pub url: String,
    #[serde(rename = "ConnectionToken")] pub connection_token: String,
    #[serde(rename = "ConnectionId")] pub connection_id: String,
    #[serde(rename = "KeepAliveTimeout")] keep_alive_timeout: f64,
    #[serde(rename = "DisconnectTimeout")] disconnect_timeout: f64,
    #[serde(rename = "ConnectionTimeout")] connection_timeout: f64,
    #[serde(rename = "TryWebSockets")] try_web_sockets: bool,
    #[serde(rename = "ProtocolVersion")] protocol_version: String,
    #[serde(rename = "TransportConnectTimeout")] transport_connect_timeout: f64,
    #[serde(rename = "LongPollDelay")] long_poll_delay: f64,
}
