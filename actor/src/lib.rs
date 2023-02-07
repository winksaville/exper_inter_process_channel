use msg_header::BoxMsgAny;
use protocol_set::ProtocolSet;
use std::sync::mpsc::Sender;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorId(pub Uuid);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorInstanceId(Uuid);

impl Default for ActorInstanceId {
    fn default() -> Self {
        ActorInstanceId::new()
    }
}

impl ActorInstanceId {
    pub fn new() -> Self {
        ActorInstanceId(Uuid::new_v4())
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

pub trait Actor {
    fn get_name(&self) -> &str;
    fn get_id(&self) -> &ActorId;
    fn get_instance_id(&self) -> &ActorInstanceId;
    fn get_protocol_set(&self) -> &ProtocolSet;
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_actor() {
        println!("test_actor");
    }
}
