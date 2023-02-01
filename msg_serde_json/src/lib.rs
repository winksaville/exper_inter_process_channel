use msg_header::{BoxMsgAny, MSG_ID_STR_LEN};

pub type FromSerdeJsonBuf = fn(&[u8]) -> std::option::Option<BoxMsgAny>;
pub type ToSerdeJsonBuf = fn(BoxMsgAny) -> std::option::Option<Vec<u8>>;

pub fn get_id_str_from_buf(serde_json_msg_header: &[u8]) -> &str {
    const SERDE_JSON_MSG_HEADER_PREFIX: &str = r#"{"header":{"id":""#;
    //println!("MsgHeader::get_id_str_from_buf:+");
    if serde_json_msg_header.starts_with(SERDE_JSON_MSG_HEADER_PREFIX.as_bytes())
        && serde_json_msg_header.len() >= (SERDE_JSON_MSG_HEADER_PREFIX.len() + MSG_ID_STR_LEN)
    {
        let id_buf = &serde_json_msg_header[SERDE_JSON_MSG_HEADER_PREFIX.len()
            ..(SERDE_JSON_MSG_HEADER_PREFIX.len() + MSG_ID_STR_LEN)];
        if let Ok(id_str) = std::str::from_utf8(id_buf) {
            //println!("MsgHeader::get_id_str_from_buf: {id_str}");
            id_str
        } else {
            //println!("MsgHeader::get_id_str_from_buf: NOT a utf8 buffer: {serde_json_msg_header:x?}");
            ""
        }
    } else {
        //println!("MsgHeader::get_id_str_from_buf: too short buffer: {serde_json_msg_header:x?}");
        ""
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const AN_ID_STR: &str = "3ab7c2f7-6445-4529-a675-5e3246217452";

    #[test]
    fn test_get_id_utf8_str() {
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}}"#.as_bytes();
        assert_eq!(AN_ID_STR, get_id_str_from_buf(msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}"#.as_bytes();
        assert_eq!(AN_ID_STR, get_id_str_from_buf(msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452""#.as_bytes();
        assert_eq!(AN_ID_STR, get_id_str_from_buf(msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#.as_bytes();
        assert_eq!(AN_ID_STR, get_id_str_from_buf(msg));

        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e324621745"#.as_bytes();
        assert_ne!(AN_ID_STR, get_id_str_from_buf(msg));
    }
}
