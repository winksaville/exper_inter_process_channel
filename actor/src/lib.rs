use crossbeam_channel::Sender;
use msg_header::BoxMsgAny;
use protocol_set::ProtocolSet;
use std::fmt::Debug;
use uuid::Uuid;

pub type ProcessMsgFn<SM> = fn(&mut SM, Option<&Sender<BoxMsgAny>>, BoxMsgAny);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorId(pub Uuid);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorInstanceId(pub Uuid);

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

pub trait Actor: Send + Debug + Sync {
    fn get_name_and_short_instance_id(&self) -> String {
        let mut s: String = "".to_string();

        s += self.get_name();
        s += "-iid:";
        s += &self.get_instance_id().0.to_string()[0..8];

        s
    }
    fn get_name_and_long_instance_id(&self) -> String {
        let mut s: String = "".to_owned();

        s += self.get_name();
        s += "-iid:";
        s += &self.get_instance_id().0.to_string();

        s
    }
    fn get_name_and_short_actor_idid(&self) -> String {
        let mut s: String = "".to_string();

        s += self.get_name();
        s += "-aid:";
        s += &self.get_actor_id().0.to_string()[0..8];

        s
    }
    fn get_name_and_long_actor_id(&self) -> String {
        let mut s: String = "".to_owned();

        s += self.get_name();
        s += "-aid:";
        s += &self.get_actor_id().0.to_string();

        s
    }

    fn get_name(&self) -> &str;
    fn get_actor_id(&self) -> &ActorId;
    fn get_instance_id(&self) -> &ActorInstanceId;
    fn get_protocol_set(&self) -> &ProtocolSet;
    fn set_self_sender(&mut self, sender: Sender<BoxMsgAny>);
    fn process_msg_any(&mut self, reply_tx: Option<&Sender<BoxMsgAny>>, msg: BoxMsgAny);
    fn done(&self) -> bool;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_actor() {
        println!("test_actor");
    }
}
