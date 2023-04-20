//! Protocol implemented by entities that sends requests
//! and receives responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use msg_deser_req::*;
pub use msg_deser_rsp::*;

// From: https://www.uuidgenerator.net/version4
const MSG_DESER_REQUESTER_PROTOCOL_ID: AnId = anid!("25932cfb-a193-4cc1-899b-a61300a3bcc4");
const MSG_DESER_REQUESTER_PROTOCOL_NAME: &str = "msg_deser_requester_protocol";
static MSG_DESER_REQUESTER_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![MSG_DESER_REQ_ID, MSG_DESER_RSP_ID]);

static MSG_DESER_REQ_RSP_PROTOCOL: Lazy<MsgDeserRequesterProtocol> = Lazy::new(|| {
    Protocol::new(
        MSG_DESER_REQUESTER_PROTOCOL_NAME,
        MSG_DESER_REQUESTER_PROTOCOL_ID,
        MSG_DESER_REQUESTER_PROTOCOL_MESSAGES.clone(),
    )
});

pub type MsgDeserRequesterProtocol = Protocol;

pub fn msg_deser_requester_protocol() -> &'static MsgDeserRequesterProtocol {
    &MSG_DESER_REQ_RSP_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_deser_requester_protocol() {
        let errp = msg_deser_requester_protocol();
        assert_eq!(errp.id, MSG_DESER_REQUESTER_PROTOCOL_ID);
        assert_eq!(errp.name, MSG_DESER_REQUESTER_PROTOCOL_NAME);
        assert_eq!(errp.messages, *MSG_DESER_REQUESTER_PROTOCOL_MESSAGES);
    }
}
