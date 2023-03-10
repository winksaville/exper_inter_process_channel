//! Protocol implemented by entities that send requests
//! and receive responses.
use msg_header::MsgId;
use once_cell::sync::Lazy;
use protocol::{Protocol, ProtocolId};
use uuid::uuid;

// Re-exports
pub use echo_reply::*;
pub use echo_req::*;

const ECHO_REQUESTEE_PROTOCOL_ID: ProtocolId =
    ProtocolId(uuid!("16e9c5a6-cf3f-4813-b0e2-1c3c54058183"));
const ECHO_REQUESTEE_PROTOCOL_NAME: &str = "echo_requestee_protocol";
static ECHO_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<MsgId>> =
    Lazy::new(|| vec![ECHO_REQ_ID, ECHO_REPLY_ID]);

static ECHO_REQ_REPLY_PROTOCOL: Lazy<EchoRequesteeProtocol> = Lazy::new(|| {
    Protocol::new(
        ECHO_REQUESTEE_PROTOCOL_NAME,
        ECHO_REQUESTEE_PROTOCOL_ID,
        ECHO_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type EchoRequesteeProtocol = Protocol;

pub fn echo_requestee_protocol() -> &'static EchoRequesteeProtocol {
    &ECHO_REQ_REPLY_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_requestee_protocol() {
        let errp = echo_requestee_protocol();
        assert_eq!(errp.id, ECHO_REQUESTEE_PROTOCOL_ID);
        assert_eq!(errp.name, ECHO_REQUESTEE_PROTOCOL_NAME);
        assert_eq!(errp.messages, *ECHO_REQUESTEE_PROTOCOL_MESSAGES);
    }
}
