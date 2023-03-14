//! Protocol implemented by entities that send requests
//! and receive responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use echo_req::*;
pub use echo_rsp::*;

const ECHO_REQUESTEE_PROTOCOL_ID: AnId = anid!("16e9c5a6-cf3f-4813-b0e2-1c3c54058183");
const ECHO_REQUESTEE_PROTOCOL_NAME: &str = "echo_requestee_protocol";
static ECHO_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![ECHO_REQ_ID, ECHO_RSP_ID]);

static ECHO_REQ_RSP_PROTOCOL: Lazy<EchoRequesteeProtocol> = Lazy::new(|| {
    Protocol::new(
        ECHO_REQUESTEE_PROTOCOL_NAME,
        ECHO_REQUESTEE_PROTOCOL_ID,
        ECHO_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type EchoRequesteeProtocol = Protocol;

pub fn echo_requestee_protocol() -> &'static EchoRequesteeProtocol {
    &ECHO_REQ_RSP_PROTOCOL
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
