use msg_header::{MsgHeader, MsgId};
use uuid::uuid;

// From: https://www.uuidgenerator.net/version4
pub const MSG2_ID: MsgId = uuid!("4029b3c4-f380-488a-8560-8320cc8fb76e");

// Message 2
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Msg2 {
    pub header: MsgHeader,
}

impl Default for Msg2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Msg2 {
    pub fn new() -> Self {
        Self {
            header: MsgHeader { id: MSG2_ID },
        }
    }

    pub fn id(&self) -> MsgId {
        self.header.id
    }
}
