use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type MsgId = Uuid;

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}
