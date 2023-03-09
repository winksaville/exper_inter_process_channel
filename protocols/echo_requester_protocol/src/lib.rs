//! Protocol implemented by entities that that receive requests
//! and send responses.
use an_id::AnId;
use msg_header::MsgId;
use once_cell::sync::Lazy;
use protocol::{Protocol, ProtocolId};
use uuid::uuid;

// Re-exports
pub use echo_reply::*;
pub use echo_req::*;

const ECHO_REQUESTER_PROTOCOL_ID: ProtocolId =
    ProtocolId(AnId(uuid!("2084ca39-77f0-4ba0-b3f9-693f529e727b")));
const ECHO_REQUESTER_PROTOCOL_NAME: &str = "echo_requester_protocol";
static ECHO_REQUESTER_PROTOCOL_MESSAGES: Lazy<Vec<MsgId>> =
    Lazy::new(|| vec![ECHO_REQ_ID, ECHO_REPLY_ID]);

static ECHO_REQUESTER_PROTOCOL: Lazy<EchoRequesterProtocol> = Lazy::new(|| {
    Protocol::new(
        ECHO_REQUESTER_PROTOCOL_NAME,
        ECHO_REQUESTER_PROTOCOL_ID,
        ECHO_REQUESTER_PROTOCOL_MESSAGES.clone(),
    )
});

pub type EchoRequesterProtocol = Protocol;

pub fn echo_requester_protocol() -> &'static EchoRequesterProtocol {
    &ECHO_REQUESTER_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_requester_protocol() {
        let errp = echo_requester_protocol();
        assert_eq!(errp.id, ECHO_REQUESTER_PROTOCOL_ID);
        assert_eq!(errp.name, ECHO_REQUESTER_PROTOCOL_NAME);
        assert_eq!(errp.messages, *ECHO_REQUESTER_PROTOCOL_MESSAGES);
    }
}
