//! Protocol implemented by entities that receives requests
//! and sends responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use msg_deser_req::*;
pub use msg_deser_rsp::*;

// From: https://www.uuidgenerator.net/version4
const MSG_DESER_REQUESTEE_PROTOCOL_ID: AnId = anid!("3195c0ef-0818-40ba-811e-fdc64bbe3458");
const MSG_DESER_REQUESTEE_PROTOCOL_NAME: &str = "msg_deser_requestee_protocol";
static MSG_DESER_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![MSG_DESER_REQ_ID, MSG_DESER_RSP_ID]);

static MSG_DESER_REQ_RSP_PROTOCOL: Lazy<MsgDeserRequesteeProtocol> = Lazy::new(|| {
    Protocol::new(
        MSG_DESER_REQUESTEE_PROTOCOL_NAME,
        MSG_DESER_REQUESTEE_PROTOCOL_ID,
        MSG_DESER_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type MsgDeserRequesteeProtocol = Protocol;

pub fn msg_deser_requestee_protocol() -> &'static MsgDeserRequesteeProtocol {
    &MSG_DESER_REQ_RSP_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_deser_requestee_protocol() {
        let errp = msg_deser_requestee_protocol();
        assert_eq!(errp.id, MSG_DESER_REQUESTEE_PROTOCOL_ID);
        assert_eq!(errp.name, MSG_DESER_REQUESTEE_PROTOCOL_NAME);
        assert_eq!(errp.messages, *MSG_DESER_REQUESTEE_PROTOCOL_MESSAGES);
    }
}
