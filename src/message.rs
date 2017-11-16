use protocol::Protocol;
enum Message {
    Negotiation (Protocol),
    Invocation,
    StreamInvocation,
    StreamItem,
    Completion,
    CancelInvocation
}
