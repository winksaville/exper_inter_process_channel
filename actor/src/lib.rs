use an_id::AnId;
use crossbeam_channel::Sender;
use msg_header::BoxMsgAny;
use protocol_set::ProtocolSet;
use std::fmt::Debug;

pub type ProcessMsgFn<SM> = fn(&mut SM, context: &dyn ActorContext, BoxMsgAny);

// These methods may only be invoked from a single threaded
// entity, which by definition Actors are.
pub trait ActorContext {
    // There must always be a connection manager
    fn send_conn_mgr(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    // You must always be able to send a message to yourself
    // although, maybe in a test case it could be a NOP?
    fn send_self(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    // You can always send a response, but if there is no
    // rsp_tx then the message will just be dropped.
    // Guard this with a `context.has_rsp_tx()` to check.
    fn send_rsp(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    // The rsp_tx can be missing if so return Option
    fn clone_rsp_tx(&self) -> Option<Sender<BoxMsgAny>>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorId(pub AnId);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ActorInstanceId(pub AnId);

impl Default for ActorInstanceId {
    fn default() -> Self {
        ActorInstanceId::new()
    }
}

impl ActorInstanceId {
    pub fn new() -> Self {
        ActorInstanceId(AnId::new())
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
    fn process_msg_any(&mut self, context: &dyn ActorContext, msg: BoxMsgAny);
    fn done(&self) -> bool;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_actor() {
        println!("test_actor");
    }
}
