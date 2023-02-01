use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Messages are things that implement trait std::any::Any
// which is most anything
pub type BoxMsgAny = Box<dyn std::any::Any + Send>;

pub type MsgId = Uuid;

// Message Header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[repr(C)]
pub struct MsgHeader {
    pub id: MsgId,
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::uuid;

    const AN_ID: MsgId = uuid!("3ab7c2f7-6445-4529-a675-5e3246217452");

    #[test]
    fn test_id() {
        let header = MsgHeader { id: AN_ID };
        println!("test_id: header={header:?}");
        assert_eq!(AN_ID, header.id);
    }
}
