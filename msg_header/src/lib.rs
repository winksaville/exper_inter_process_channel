//-----------------------------------------------
#![feature(downcast_unchecked)] // This can be commented out and
// will compile if rust-toolchain.toml is changed to `channel = stable`.
// BUT THE RESULT WILL NOT RUN, as the stable version of
// get_msg_id_from_boxed_msg_any and get_dst_id_from_boxed_msg_any
// are just stubs and return the wrong values. The nightly version
// of these functions work correctly.
//
// I've did this to debug the issue with Rust-Analyzer
// reporting a false error for sender_map_insert.
// See:
//  https://users.rust-lang.org/t/ra-reports-errors-but-compile-succeeds/92127
//  https://github.com/rust-lang/rust-analyzer/pull/14475
//
// Athough this bug appears to be fixed in the lastest version, 0.3.1533,
// I'm leaving this here as I do want this to compile under stable someday.
// By leaving this it is "trivial" to switch back to stable and hopefully
// get this not only compiling but acutally running and passing `cargo test --all`.
#[rustversion::stable]
use an_id::anid;
#[rustversion::stable]
use paste::paste;
#[rustversion::stable]
const DEBUG_ANID: AnId = anid!("def26b5a-a462-492a-acdd-85aac7a1f2ac");
//-----------------------------------------------

use actor_channel::ActorSender;
use an_id::AnId;
use box_msg_any::BoxMsgAny;
use sender_map_by_instance_id::sender_map_get;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
mod get_msg_id_str_from_buf;
pub use get_msg_id_str_from_buf::{get_msg_id_str_from_buf, FromSerdeJsonBuf, ToSerdeJsonBuf};


pub const MSG_ID_STR_LEN: usize = "00000000-0000-0000-0000-000000000000".len();

// Message Header
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub msg_id: AnId, // Message ID
    pub dst_id: AnId, // Destination ID this is needed routing when the message
    // is sent via a network and one network connection is
    // multiplexing messages to multiple actors. ATM, this is
    // not needed with local actors connected with plain channels.
    pub src_id: AnId, // Source ID
}

impl MsgHeader {
    pub fn new(msg_id: AnId, dst_id: AnId, src_id: AnId) -> Self {
        //println!("MsgHeader::new");
        Self {
            msg_id,
            dst_id,
            src_id,
        }
    }

    #[rustversion::nightly]
    pub fn get_msg_id_from_boxed_msg_any(msg: &BoxMsgAny) -> &AnId {
        // TODO: Consider validating that the msg_header or AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg.downcast_ref_unchecked() };
        &mh.msg_id
    }

    #[rustversion::stable]
    pub fn get_msg_id_from_boxed_msg_any(_msg: &BoxMsgAny) -> &AnId {
        &DEBUG_ANID
    }

    #[rustversion::nightly]
    pub fn get_dst_id_from_boxed_msg_any(msg_any: &BoxMsgAny) -> &AnId {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg_any.downcast_ref_unchecked() };

        &mh.dst_id
    }

    #[rustversion::stable]
    pub fn get_dst_id_from_boxed_msg_any(_msg_any: &BoxMsgAny) -> &AnId {
        &DEBUG_ANID
    }

    pub fn get_dst_sndr_from_boxed_msg_any(msg_any: &BoxMsgAny) -> Option<ActorSender> {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let dst_id = MsgHeader::get_dst_id_from_boxed_msg_any(msg_any);

        sender_map_get(dst_id)
    }

    #[rustversion::nightly]
    pub fn get_src_id_from_boxed_msg_any(msg_any: &BoxMsgAny) -> &AnId {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let mh: &MsgHeader = unsafe { msg_any.downcast_ref_unchecked() };

        &mh.src_id
    }

    #[rustversion::stable]
    pub fn get_src_id_from_boxed_msg_any(_msg_any: &BoxMsgAny) -> &AnId {
        &DEBUG_ANID
    }

    pub fn get_src_tx_from_boxed_msg_any(msg_any: &BoxMsgAny) -> Option<ActorSender> {
        // TODO: Consider validating that this is AnId. One way
        // would be to have a "global" hashmap of valid values another
        // way would be to add a "check-sum"?
        // See https://doc.rust-lang.org/std/any/trait.Any.html#method.downcast_ref_unchecked
        let src_id = MsgHeader::get_src_id_from_boxed_msg_any(msg_any);

        sender_map_get(src_id)
    }

    pub fn simple_display(&self) -> String {
        format!(
            "mh {{ msg_id: {} dst_id: {} src_id: {} }}",
            &self.msg_id.to_string()[0..8],
            &self.dst_id.to_string()[0..8],
            &self.src_id.to_string()[0..8],
        )
    }
}

impl Debug for MsgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.debug_struct("MsgHeader")
                .field("msg_id", &self.msg_id)
                .field("dst_id", &self.dst_id)
                .field("src_id", &self.src_id)
                .finish()
        } else {
            write!(f, "{}", self.simple_display())
        }
    }
}

impl Display for MsgHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_display())
    }
}

#[cfg(test)]
mod test {
    use an_id::{anid, paste};

    use super::*;

    #[test]
    fn test_size() {
        println!("\n");
        let header = MsgHeader::new(AnId::nil(), AnId::nil(), AnId::nil());
        let size = std::mem::size_of_val(&header);
        println!("test_default: size_of_val(&header)={size}    {{header}}={header}");
        println!("test_default: size_of_val(&header)={size}  {{header:?}}={header:?}");
        println!("test_default: size_of_val(&header)={size} {{header:#?}}={header:#?}");
        assert_eq!(size, 48);
    }

    #[test]
    fn test_new() {
        println!("\n");
        let msg_id = AnId::nil();
        let dst_id = anid!("ad376277-0f23-4f4e-bbe6-4f76708da4fe");
        let src_id = anid!("31d1ee24-0dfd-49cd-906a-857aa67e59f4");

        let header = MsgHeader::new(msg_id, dst_id, src_id);
        println!("test_new:    {{header}}={header}");
        println!("test_new:  {{header:?}}={header:?}");
        println!("test_new: {{header:#?}}={header:#?}");
        assert_eq!(header.msg_id, msg_id);
        assert_eq!(header.dst_id, dst_id);
        assert_eq!(header.src_id, src_id);
    }

    #[test]
    fn test_msg_id_utf8_len() {
        println!("\n");
        let nill_utf8: String = AnId::nil().to_string();
        println!("test_msg_id_utf8_len: nil_utf8={nill_utf8}");
        assert_eq!(MSG_ID_STR_LEN, nill_utf8.len());
    }
}
