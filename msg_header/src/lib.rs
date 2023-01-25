use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type MsgId = Uuid;

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}

const SIZE_ID: usize = "3ab7c2f7-6445-4529-a675-5e3246217452".len();

impl MsgHeader {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::uuid;

    const AN_ID_STR: &str = "3ab7c2f7-6445-4529-a675-5e3246217452";
    const AN_ID: MsgId = uuid!("3ab7c2f7-6445-4529-a675-5e3246217452");

    #[test]
    fn test_id() {
        let header = MsgHeader { id: AN_ID };
        println!("test_id: header={header:?}");
        assert_eq!(AN_ID, header.id);
    }

    #[test]
    fn test_cmp_str_id_an_serde_json_msg_header() {
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}}"#;
        assert!(MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"}"#;
        assert!(MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452""#;
        assert!(MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
    }

    #[test]
    fn test_cmp_str_id_and_serde_json_msg_header_with_short_id_in_header() {
        let msg = r#"{"header":{"id":"3ab7c2f7-6445-4529-a675-5e324621745"#;
        assert!(!MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
    }

    #[test]
    fn test_cmp_serde_json_msg_header_with_bad_msg_header() {
        // Has "hader" instead of "header"
        let msg = r#"{"hader":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));

        // Has extra space, with a regex we could handle this.
        let msg = r#"{ "header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));

        // Has capitialization error, with a regex we could handle this.
        let msg = r#"{ "Header":{"id":"3ab7c2f7-6445-4529-a675-5e3246217452"#;
        assert!(!MsgHeader::cmp_str_id_and_serde_json_msg_header(
            AN_ID_STR, msg
        ));
    }
}
