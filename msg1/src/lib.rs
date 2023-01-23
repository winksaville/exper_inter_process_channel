use msg_header::{MsgHeader, MsgId};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG1_ID: MsgId = uuid!("a88ba7e7-0930-4df6-bb24-240338bf8eb5");

// Message 1
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg1 {
    pub header: MsgHeader,
}

impl Default for Msg1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Msg1 {
    pub fn new() -> Self {
        Self {
            header: MsgHeader { id: MSG1_ID },
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }
}
