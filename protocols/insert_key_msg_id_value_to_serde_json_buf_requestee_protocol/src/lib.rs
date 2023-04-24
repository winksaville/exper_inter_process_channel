//! Protocol implemented by entities that receives requests
//! and sends responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use insert_key_msg_id_value_to_serde_json_buf_req::*;
pub use insert_key_msg_id_value_to_serde_json_buf_rsp::*;

// From: https://www.uuidgenerator.net/version4
const INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID: AnId =
    anid!("e10c6277-063b-4688-a40c-57d594c1d02c");
const INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME: &str =
    "insert_key_msg_id_value_to_serde_json_buf_requestee_protocol";
static INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| {
        vec![
            INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQ_ID,
            INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_RSP_ID,
        ]
    });

static INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL: Lazy<
    InsertKeyMsgIdValueToSerdeJsonBufRequesteeProtocol,
> = Lazy::new(|| {
    Protocol::new(
        INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME,
        INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID,
        INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type InsertKeyMsgIdValueToSerdeJsonBufRequesteeProtocol = Protocol;

pub fn insert_key_msg_id_value_to_serde_json_buf_requestee_protocol(
) -> &'static InsertKeyMsgIdValueToSerdeJsonBufRequesteeProtocol {
    &INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_key_msg_id_value_to_serde_json_buf_requestee_protocol() {
        let protocol = insert_key_msg_id_value_to_serde_json_buf_requestee_protocol();
        assert_eq!(
            protocol.id,
            INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID
        );
        assert_eq!(
            protocol.name,
            INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME
        );
        assert_eq!(
            protocol.messages,
            *INSERT_KEY_MSG_ID_VALUE_TO_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES
        );
    }
}
