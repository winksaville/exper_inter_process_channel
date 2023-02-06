use msg_header::BoxMsgAny;
use protocol::ProtocolId;
use std::{fmt, sync::mpsc::Sender};
use uuid::Uuid;

// Actors process messages
pub trait ProcessMsgAny {
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny);
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorId(Uuid);

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Actor {
    id: ActorId,
    protocols: Vec<ProtocolId>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorInstanceId(Uuid);

#[allow(unused)]
pub struct ActorInstance {
    instance_id: ActorInstanceId,
    actor_id: ActorId,
    channel: Box<dyn ActorChannel>, // Make Vec as can be connected to multiple other actors.
}

// Manually implement Debug as ActorChannel does not yet implement Debug
impl fmt::Debug for ActorInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ActorInstance")
            .field("instance_id", &self.instance_id)
            .field("actor_id", &self.actor_id)
            .finish()
    }
}

pub trait ActorChannel {
    fn send(&self, _msg: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>> {
        Err("ActorChannel `fn send` not implemented".into())
    }

    fn recv(&self) -> Result<BoxMsgAny, Box<dyn std::error::Error>> {
        Err("ActorChannel `fn recv` not implemented".into())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_actor() {
        println!("test_actor");
    }
}
