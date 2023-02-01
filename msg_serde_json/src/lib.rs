use msg_header::BoxMsgAny;

pub type FromSerdeJsonBuf = fn(&[u8]) -> std::option::Option<BoxMsgAny>;
pub type ToSerdeJsonBuf = fn(BoxMsgAny) -> std::option::Option<Vec<u8>>;

const SIZE_ID: usize = "3ab7c2f7-6445-4529-a675-5e3246217452".len();

pub fn cmp_str_id_and_serde_json_msg_header(str_id: &str, serde_json_msg_header: &str) -> bool {
    const SERDE_JSON_MSG_HEADER: &str = r#"{"header":{"id":""#;
    //println!("MsgHeader::cmp_str_id_and_serde_json_msg_header:+ SERDE_JSON_MSG_HEADER.len={} serde_json_msg_header len={} {serde_json_msg_header} serde_json_msg_header len={} {serde_json_msg_header}", SERDE_JSON_MSG_HEADER.len(), str_id.len(), serde_json_msg_header.len());
    if serde_json_msg_header.starts_with(SERDE_JSON_MSG_HEADER)
        && serde_json_msg_header.len() >= (SERDE_JSON_MSG_HEADER.len() + SIZE_ID)
    {
        let rhs_uuid_str = &serde_json_msg_header
            [SERDE_JSON_MSG_HEADER.len()..SERDE_JSON_MSG_HEADER.len() + str_id.len()];
        //println!("MsgHeader::cmp_str_id_and_serde_json_msg_header: r={:5} serde_json_msg_header={serde_json_msg_header} rhs_uuid_str={rhs_uuid_str}", str_id == rhs_uuid_str);
        str_id == rhs_uuid_str
    } else {
        //println!("MsgHeader::cmp_str_id_and_serde_json_msg_header: r=false serde_json_msg_header={serde_json_msg_header} is to short");

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const AN_ID_STR: &str = "3ab7c2f7-6445-4529-a675-5e3246217452";

    #[test]
    fn test_cmp_str_id_an_serde_json_msg_header() {
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}}"#;
        assert!(cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}"#;
        assert!(cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452""#;
        assert!(cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
    }

    #[test]
    fn test_cmp_str_id_and_serde_json_msg_header_with_short_id_in_header() {
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e324621745"#;
        assert!(!cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
    }

    #[test]
    fn test_cmp_serde_json_msg_header_with_bad_msg_header() {
        // Has "hader" instead of "header"
        let msg = r#"{"hader":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));

        // Has extra space, with a regex we could handle this.
        let msg = r#"{ "header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));

        // Has capitialization error, with a regex we could handle this.
        let msg = r#"{ "Header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!cmp_str_id_and_serde_json_msg_header(AN_ID_STR, msg));
    }
}
