use an_id::{anid, paste, AnId};
use msg_header::{BoxMsgAny, MsgHeader};
use msg_serde_json::get_msg_id_str_from_buf;
use serde::{Deserialize, Serialize};

// From: https://www.uuidgenerator.net/version4
pub const MSG1_ID_STR: &str = "a88ba7e7-0930-4df6-bb24-240338bf8eb5";
pub const MSG1_ID: AnId = anid!("a88ba7e7-0930-4df6-bb24-240338bf8eb5");
pub const MSG1_NAME: &str = "Msg1";

// Message 1
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct Msg1 {
    pub header: MsgHeader,
    pub v: u64,
}

impl Default for Msg1 {
    fn default() -> Self {
        Self::new(0x1234567890ABCDEF)
    }
}

// Allow `clippy::uninlined_format_args`  because in msg_macro
// we need to use stringify!($name) which can't be used in a
// format string. Also this is caught by `cargo +nightly clippy`.
#[allow(clippy::uninlined_format_args)]
impl Msg1 {
    pub fn new(v: u64) -> Self {
        Self {
            header: MsgHeader::new_msg_id_only(MSG1_ID),
            v,
        }
    }

    pub fn msg_id(&self) -> &AnId {
        &self.header.msg_id
    }

    pub fn src_id(&self) -> &Option<AnId> {
        &self.header.src_id
    }

    pub fn from_box_msg_any(msg: &BoxMsgAny) -> Option<&Msg1> {
        if let Some(msg1) = msg.downcast_ref::<Self>() {
            Some(msg1)
        } else {
            None
        }
    }

    pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<BoxMsgAny> {
        let id = get_msg_id_str_from_buf(buf);
        //if id == &*MSG1_ID_STRING {
        if id == MSG1_ID_STR {
            if let Ok(s) = std::str::from_utf8(buf) {
                match serde_json::from_str::<Self>(s) {
                    Ok(msg) => Some(Box::new(msg)),
                    Err(why) => {
                        log::error!("{}::from_serde_json_str: {why}", MSG1_NAME);
                        None
                    }
                }
            } else {
                log::error!(
                    "{}::from_serde_json_buf: buf parameter was NOT UTF8",
                    MSG1_NAME
                );
                None
            }
        } else {
            log::error!(
                "{MSG1_NAME} id: {}, does not match buffer id: {id}",
                MSG1_ID_STR
            );
            None
        }
    }

    pub fn to_serde_json_buf(boxed_msg_any: BoxMsgAny) -> std::option::Option<Vec<u8>> {
        if let Some(m) = boxed_msg_any.downcast_ref::<Self>() {
            match serde_json::to_vec(m) {
                Ok(v) => Some(v),
                Err(why) => {
                    log::error!("{}.to_serde_json_buf: Error {}", MSG1_NAME, why);
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use msg_serde_json::{get_msg_id_str_from_buf, FromSerdeJsonBuf, ToSerdeJsonBuf};
    use std::{
        any::{Any, TypeId},
        collections::HashMap,
    };

    #[test]
    fn test_msg1_to_from_serde_json_buf() {
        let msg1 = Msg1::default();
        println!("test_msg1_to_from_serde_json_buf: msg1: {msg1:?}");
        let bma1: BoxMsgAny = Box::new(msg1.clone());
        let ser_msg1 = Msg1::to_serde_json_buf(bma1).unwrap();
        let bma1_deser = Msg1::from_serde_json_buf(&ser_msg1).unwrap();
        let msg1_deser = bma1_deser.downcast_ref::<Msg1>().unwrap();
        println!("test_msg1_to_from_serde_json_buf: msg1_deser={msg1_deser:?}");
        assert_eq!(&msg1, msg1_deser);
        assert_eq!(msg1.header.msg_id, MSG1_ID);
        assert_eq!(msg1.header.msg_id, msg1_deser.header.msg_id);
        println!(
            "test_msg1_to_from_serde_json_buf: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
            TypeId::of::<Msg1>(),
            msg1_deser.type_id()
        );
        assert_eq!(TypeId::of::<Msg1>(), msg1_deser.type_id());
    }

    #[test]
    fn test_hash_map_to_from_serde_json_buf() {
        let msg1 = Msg1::default();
        println!("MSG1_ID_STR={}", MSG1_ID_STR);
        println!("msg1={msg1:?}");

        // Use HashMap to serialize
        let ser = Msg1::to_serde_json_buf;
        let mut hm_ser = HashMap::<AnId, ToSerdeJsonBuf>::new();
        hm_ser.insert(*msg1.msg_id(), ser);
        println!("hm_ser.len()={}", hm_ser.len());

        // Use another HashMap to deserialize
        let deser = Msg1::from_serde_json_buf;
        let mut hm_deser = HashMap::<String, FromSerdeJsonBuf>::new();
        let msg1_id_str = MSG1_ID_STR;
        hm_deser.insert(msg1_id_str.to_string(), deser);
        println!("hm_deser.len()={}", hm_deser.len());

        // Instantiate msg1 as a BoxMsgAny
        let msg1_box_msg_any: BoxMsgAny = Box::new(msg1.clone());

        // Serialize using the to_serde_json_buf funtion
        let fn_to_serde_json_buf = hm_ser.get(&msg1.msg_id()).unwrap();
        let msg1_buf = (*fn_to_serde_json_buf)(msg1_box_msg_any).unwrap();
        println!("msg1_buf {:p} {msg1_buf:x?}", &*msg1_buf);
        println!("msg1_buf uf8={:?}", std::str::from_utf8(&msg1_buf).unwrap());

        // Deserialize using the from_serde_json_buf function
        let id_str = get_msg_id_str_from_buf(&msg1_buf);
        println!("id_str={id_str}");
        let fn_from_serde_json_buf = hm_deser.get(id_str).unwrap();
        let msg1_serde_box_msg_any = (*fn_from_serde_json_buf)(&msg1_buf).unwrap();
        println!(
            "msg1_serde_box_msg_any: {:p} {:p} {} {msg1_serde_box_msg_any:?}",
            msg1_serde_box_msg_any,
            &*msg1_serde_box_msg_any,
            std::mem::size_of::<BoxMsgAny>()
        );

        let msg1_serde = if let Some(msg) = Msg1::from_box_msg_any(&msg1_serde_box_msg_any) {
            msg
        } else {
            panic!("msg1_serde was not a Msg1");
        };
        println!(
            "msg1_serde: {:p} {:?} {} {msg1_serde:?}",
            msg1_serde,
            msg1_serde.type_id(),
            std::mem::size_of::<Msg1>()
        );
        assert_eq!(&msg1, msg1_serde);
    }
}
