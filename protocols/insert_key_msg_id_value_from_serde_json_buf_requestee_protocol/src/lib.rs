//! Protocol implemented by entities that receives requests
//! and sends responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use insert_key_msg_id_value_from_serde_json_buf_req::*;
pub use insert_key_msg_id_value_from_serde_json_buf_rsp::*;

// From: https://www.uuidgenerator.net/version4
const INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID: AnId =
    anid!("3195c0ef-0818-40ba-811e-fdc64bbe3458");
const INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME: &str =
    "insert_key_msg_id_value_from_serde_json_buf_requestee_protocol";
static INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| {
        vec![
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQ_ID,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_RSP_ID,
        ]
    });

static INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL: Lazy<
    InsertKeyMsgIdValueFromSerdeJsonBufRequesteeProtocol,
> = Lazy::new(|| {
    Protocol::new(
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME,
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID,
        INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type InsertKeyMsgIdValueFromSerdeJsonBufRequesteeProtocol = Protocol;

pub fn insert_key_msg_id_value_from_serde_json_buf_requestee_protocol(
) -> &'static InsertKeyMsgIdValueFromSerdeJsonBufRequesteeProtocol {
    &INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_key_msg_id_value_from_serde_json_buf_requestee_protocol() {
        let errp = insert_key_msg_id_value_from_serde_json_buf_requestee_protocol();
        assert_eq!(
            errp.id,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_ID
        );
        assert_eq!(
            errp.name,
            INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_NAME
        );
        assert_eq!(
            errp.messages,
            *INSERT_KEY_MSG_ID_VALUE_FROM_SERDE_JSON_BUF_REQUESTEE_PROTOCOL_MESSAGES
        );
    }
}
