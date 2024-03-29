//! `The msg_macro!` provides a simple way to create msgs.
pub use paste::paste;

// TODO: Ugly, too much repeated code, I tired to add
// msg_macro_base which what the first arm is and then
// invoke it on both arms and adding fn default and fn new.
// But I couldn't get that to work. Also, needs to handle
// nested structs!
#[macro_export]
macro_rules! msg_local_macro_not_cloneable {
    ($name:ident $id_str:literal { $( $field:ident : $field_ty:ty ),* }) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : an_id::AnId = an_id::anid!($id_str);
        }

        #[derive(Debug)]
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

            pub fn dst_id(&self) -> &an_id::AnId {
                &self.header.dst_id
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
        }
    };
}

#[macro_export]
macro_rules! msg_local_macro {
    ($name:ident $id_str:literal { $( $field:ident : $field_ty:ty ),* }) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : an_id::AnId = an_id::anid!($id_str);
        }

        #[derive(Debug, Clone)]
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

            pub fn dst_id(&self) -> &an_id::AnId {
                &self.header.dst_id
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
        }
    };

    ($name:ident $id_str:literal) => {
        paste! {
            #[allow(unused)]
            pub const [ <$name:snake:upper _ID_STR> ] : &str = $id_str;

            #[allow(unused)]
            pub const [ <$name:snake:upper _ID> ] : msg_header::MsgId = msg_header::MsgId(an_id::anid!($id_str));
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        #[repr(C)]
        pub struct $name {
            pub header: msg_header::MsgHeader,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        #[allow(unused)]
        impl $name {
            pub fn msg_id(&self) -> &an_id::AnId {
                &self.header.msg_id
            }

            pub fn dst_id(&self) -> &Option<an_id::AnId> {
                &self.header.dst_id
            }

            pub fn src_id(&self) -> &Option<an_id::AnId> {
                &self.header.src_id
            }

            pub fn from_box_msg_any(msg: &box_msg_any::BoxMsgAny) -> Option<&$name> {
                if let Some(m) = msg.downcast_ref::<$name>() {
                    Some(m)
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
    use crossbeam_channel::{unbounded, Sender};

    use box_msg_any::BoxMsgAny;
    use msg_header::MsgHeader;

    use super::*;

    // From: https://www.uuidgenerator.net/version4
    msg_local_macro!(MsgLclA "0d061a84-69aa-4bd0-a43f-ef3efd971d26" {
        a_u64: u64,
        a_string: String,
        a_sender: Sender<box_msg_any::BoxMsgAny> // Last field must not have a comma (need to fix macro)
    });

    impl MsgLclA {
        pub fn new(
            dst_id: &AnId,
            src_id: &AnId,
            num: u64,
            a_str: &str,
            tx: Sender<BoxMsgAny>,
        ) -> Self {
            Self {
                header: MsgHeader::new(MSG_LCL_A_ID, *dst_id, *src_id),
                a_u64: num,
                a_string: a_str.to_string(),
                a_sender: tx,
            }
        }
    }

    #[test]
    fn test_with_fields_including_a_sender() {
        let (tx, rx) = unbounded();
        let dst_id = AnId::new();
        let src_id = AnId::new();

        let msg_lcl_a_0 = Box::new(MsgLclA::new(&dst_id, &src_id, 123, "hi", tx.clone()));
        println!("test_with_fields msg_lcl_a_0={msg_lcl_a_0:?}");
        assert_eq!(msg_lcl_a_0.msg_id(), &MSG_LCL_A_ID);
        assert_eq!(msg_lcl_a_0.dst_id(), &dst_id);
        assert_eq!(msg_lcl_a_0.src_id(), &src_id);
        assert_eq!(msg_lcl_a_0.a_u64, 123);
        assert_eq!(msg_lcl_a_0.a_string, "hi");
        assert_eq!(msg_lcl_a_0.msg_id().to_string(), MSG_LCL_A_ID_STR);

        // Send the tx
        tx.send(msg_lcl_a_0.clone()).unwrap();
        let msg_lcl_a_any = rx.recv().unwrap();
        let msg_lcl_a_1 = MsgLclA::from_box_msg_any(&msg_lcl_a_any).unwrap();
        println!("test_with_fields msg_lcl_a_1={msg_lcl_a_1:?}");
        assert_eq!(msg_lcl_a_1.msg_id(), &MSG_LCL_A_ID);
        assert_eq!(msg_lcl_a_0.dst_id(), &dst_id);
        assert_eq!(msg_lcl_a_0.src_id(), &src_id);
        assert_eq!(msg_lcl_a_1.a_u64, 123);
        assert_eq!(msg_lcl_a_1.a_string, "hi");
        assert_eq!(msg_lcl_a_1.msg_id().to_string(), MSG_LCL_A_ID_STR);

        // Use the sent tx
        let sent_tx = msg_lcl_a_1.a_sender.clone();
        let msg_lcl_a_2 = Box::new(msg_lcl_a_1.clone());
        sent_tx.send(msg_lcl_a_2).unwrap();

        // Verifify again
        let msg_lcl_a_any = rx.recv().unwrap();
        let msg_lcl_a_2 = MsgLclA::from_box_msg_any(&msg_lcl_a_any).unwrap();
        println!("test_with_fields msg_lcl_a_2={msg_lcl_a_2:?}");
        assert_eq!(msg_lcl_a_2.msg_id(), &MSG_LCL_A_ID);
        assert_eq!(msg_lcl_a_0.dst_id(), &dst_id);
        assert_eq!(msg_lcl_a_0.src_id(), &src_id);
        assert_eq!(msg_lcl_a_2.a_u64, 123);
        assert_eq!(msg_lcl_a_2.a_string, "hi");
        assert_eq!(msg_lcl_a_2.msg_id().to_string(), MSG_LCL_A_ID_STR);
    }
}
