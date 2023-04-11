//! `The msg_macro!` provides a simple way to create msgs.
pub use paste::paste;

// TODO: Ugly, too much repeated code, I tired to add
// msg_macro_base which what the first arm is and then
// invoke it on both arms and adding fn default and fn new.
// But I couldn't get that to work. Also, needs to handle
// nested structs!
#[macro_export]
macro_rules! msg_serde_macro {
    ($name:ident $id_str:literal { $( $field:ident : $field_ty:ty ),* }) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : an_id::AnId = an_id::anid!($id_str);
        }

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[repr(C)]
        pub struct $name {
            pub header: msg_header::MsgHeader,
            $(
                pub $field: $field_ty,
            )*
        }

        #[allow(unused)]
        impl $name {
            pub fn msg_id(&self) -> &an_id::AnId {
                &self.header.msg_id
            }

            pub fn src_id(&self) -> &an_id::AnId {
                &self.header.src_id
            }

            pub fn from_box_msg_any(msg: &box_msg_any::BoxMsgAny) -> Option<&$name> {
                if let Some(m) = msg.downcast_ref::<$name>() {
                    Some(m)
                } else {
                    None
                }
            }

            pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<box_msg_any::BoxMsgAny> {
                let id = msg_header::get_msg_id_str_from_buf(buf);
                if id == $id_str {
                    if let Ok(s) = std::str::from_utf8(buf) {
                        match serde_json::from_str::<Self>(s) {
                            Ok(msg) => Some(Box::new(msg)),
                            Err(why) => {
                                log::error!("{}::from_serde_json_str: {why}", stringify!($name));
                                None
                            }
                        }
                    } else {
                        log::error!(
                            "{}::from_serde_json_buf: buf parameter was NOT UTF8",
                            stringify!($name)
                        );
                        None
                    }
                } else {
                    log::error!(
                        "{} id: {}, does not match buffer id: {id}",
                        stringify!($name),
                        $id_str
                    );
                    None
                }
            }

            pub fn to_serde_json_buf(
                boxed_msg_any: box_msg_any::BoxMsgAny,
            ) -> std::option::Option<Vec<u8>> {
                if let Some(m) = boxed_msg_any.downcast_ref::<Self>() {
                    match serde_json::to_vec(m) {
                        Ok(v) => Some(v),
                        Err(why) => {
                            log::error!("{}.to_serde_json_buf: Error {why}", stringify!($name));
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    };

    ($name:ident $id_str:literal) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : an_id::AnId = an_id::anid!($id_str);
        }

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #[repr(C)]
        pub struct $name {
            pub header: msg_header::MsgHeader,
        }

        #[allow(unused)]
        impl $name {
            pub fn msg_id(&self) -> &an_id::AnId {
                &self.header.msg_id
            }

            pub fn src_id(&self) -> &an_id::AnId {
                &self.header.src_id
            }

            pub fn from_box_msg_any(msg: &box_msg_any::BoxMsgAny) -> Option<&$name> {
                if let Some(m) = msg.downcast_ref::<$name>() {
                    Some(m)
                } else {
                    None
                }
            }

            pub fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<box_msg_any::BoxMsgAny> {
                let id = msg_header::get_msg_id_str_from_buf(buf);
                if id == $id_str {
                    if let Ok(s) = std::str::from_utf8(buf) {
                        match serde_json::from_str::<Self>(s) {
                            Ok(msg) => Some(Box::new(msg)),
                            Err(why) => {
                                log::error!("{}::from_serde_json_str: {why}", stringify!($name));
                                None
                            }
                        }
                    } else {
                        log::error!(
                            "{}::from_serde_json_buf: buf parameter was NOT UTF8",
                            stringify!($name)
                        );
                        None
                    }
                } else {
                    log::error!(
                        "{} id: {}, does not match buffer id: {id}",
                        stringify!($name),
                        $id_str
                    );
                    None
                }
            }

            pub fn to_serde_json_buf(
                boxed_msg_any: box_msg_any::BoxMsgAny,
            ) -> std::option::Option<Vec<u8>> {
                if let Some(m) = boxed_msg_any.downcast_ref::<Self>() {
                    match serde_json::to_vec(m) {
                        Ok(v) => Some(v),
                        Err(why) => {
                            log::error!("{}.to_serde_json_buf: Error {why}", stringify!($name));
                            None
                        }
                    }
                } else {
                    None
                }
            }
        }
    };
}

#[cfg(test)]
mod test {
    use an_id::AnId;
    use box_msg_any::BoxMsgAny;
    use msg_header::MsgHeader;

    use super::*;

    // From: https://www.uuidgenerator.net/version4
    msg_serde_macro!(MsgA "d122e9aa-0a69-4654-8e41-e2813bc40272");

    #[test]
    fn test_msg_a_to_from_serde_json_buf() {
        let src_id = AnId::new();
        let msg_a = Box::new(MsgA {
            header: MsgHeader::new(MSG_A_ID, src_id),
        });
        let msg_a_any_1: BoxMsgAny = msg_a.clone();
        let msg_a_vec = MsgA::to_serde_json_buf(msg_a_any_1).unwrap();
        let msg_a_any_2 = MsgA::from_serde_json_buf(&msg_a_vec).unwrap();
        let msg_a_deser = MsgA::from_box_msg_any(&msg_a_any_2).unwrap();
        assert_eq!(msg_a_deser.header.msg_id, MSG_A_ID);
        assert_eq!(msg_a_deser.header.msg_id.to_string(), MSG_A_ID_STR);
    }

    msg_serde_macro!(MsgB "5cd57392-151a-4460-8a2f-86c79ddad18a" {
        a_u64: u64,
        a_string: String  // Last field must not have a comma (need to fix macro)
    });

    impl MsgB {
        pub fn new(src_id: &AnId, num: u64, a_str: &str) -> Self {
            Self {
                header: MsgHeader::new(MSG_B_ID, *src_id),
                a_u64: num,
                a_string: a_str.to_string(),
            }
        }
    }

    #[test]
    fn test_with_fields() {
        let src_id = AnId::new();
        let msg_b = Box::new(MsgB::new(&src_id, 123, "hi"));
        println!("test_with_fields msg_b={msg_b:?}");
        assert_eq!(msg_b.header.msg_id, MSG_B_ID);
        assert_eq!(msg_b.header.src_id, src_id);
        assert_eq!(msg_b.a_u64, 123);
        assert_eq!(msg_b.a_string, "hi");
        assert_eq!(msg_b.header.msg_id.to_string(), MSG_B_ID_STR);

        let msg_b_any_1: BoxMsgAny = msg_b.clone();
        let msg_b_vec = MsgB::to_serde_json_buf(msg_b_any_1).unwrap();
        let msg_b_any_2 = MsgB::from_serde_json_buf(&msg_b_vec).unwrap();
        let msg_b_deser = MsgB::from_box_msg_any(&msg_b_any_2).unwrap();
        assert_eq!(msg_b_deser.header.msg_id, MSG_B_ID);
        assert_eq!(msg_b_deser.header.src_id, src_id);
        assert_eq!(msg_b_deser.a_u64, 123);
        assert_eq!(msg_b_deser.a_string, "hi");
        assert_eq!(msg_b_deser.header.msg_id.to_string(), MSG_B_ID_STR);
    }
}
