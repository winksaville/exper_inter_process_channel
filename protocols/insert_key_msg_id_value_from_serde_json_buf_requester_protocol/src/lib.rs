//! Protocol implemented by entities that sends requests
//! and receives responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use insert_key_msg_id_value_from_serde_json_buf_req::*;
pub use insert_key_msg_id_value_from_serde_json_buf_rsp::*;

// From: https://www.uuidgenerator.net/version4
const INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_ID: AnId =
    anid!("25932cfb-a193-4cc1-899b-a61300a3bcc4");
const INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_NAME: &str =
    "insert_key_msg_id_value_from_serde_json_buf_requester_protocol";
static INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| {
        vec![
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQ_ID,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_RSP_ID,
        ]
    });

static INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL: Lazy<
    InsertKeyMsgIdValueFromSerdeJsonBufRequesterProtocol,
> = Lazy::new(|| {
    Protocol::new(
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_NAME,
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_ID,
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_MESSAGES.clone(),
    )
});

pub type InsertKeyMsgIdValueFromSerdeJsonBufRequesterProtocol = Protocol;

pub fn insert_key_msg_id_value_from_serde_json_buf_requester_protocol(
) -> &'static InsertKeyMsgIdValueFromSerdeJsonBufRequesterProtocol {
    &INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_key_msg_id_value_from_serde_json_buf_requester_protocol() {
        let errp = insert_key_msg_id_value_from_serde_json_buf_requester_protocol();
        assert_eq!(
            errp.id,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_ID
        );
        assert_eq!(
            errp.name,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_NAME
        );
        assert_eq!(
            errp.messages,
            *INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTER_PROTOCOL_MESSAGES
        );
    }
}
