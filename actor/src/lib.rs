use actor_channel::{ActorChannel, ActorSender};
use an_id::AnId;
use box_msg_any::BoxMsgAny;
use std::fmt::Debug;

pub type ProcessMsgFn<SM> = fn(&mut SM, context: &dyn ActorContext, BoxMsgAny);

// These methods may only be invoked from a single threaded
// entity, which by definition Actors are.
pub trait ActorContext {
    /// Returns a reference to this actors executor tx
    fn actor_executor_sndr(&self) -> &ActorSender;

    /// Send message to connection manager
    fn send_con_mgr(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    /// Get ConMgr instance id
    fn get_con_mgr_instance_id(&self) -> &AnId;

    /// Send a message to yourself, in a test case it could be a NOP!
    fn send_self(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    /// Send a response message to the entity that issued request, if there
    /// is no Sender then the message will be silently dropped????
    fn send_dst(&self, msg_any: BoxMsgAny) -> Result<(), Box<dyn std::error::Error>>;

    /// Get Rsp instance id
    fn get_dst_instance_id(&self) -> &AnId;

    /// Clone rsp_tx
    fn clone_dst_sndr(&self) -> ActorSender;
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
    fn get_actor_id(&self) -> &AnId;
    fn get_instance_id(&self) -> &AnId;
    fn get_chnl(&self) -> &ActorChannel;
    fn process_msg_any(&mut self, context: &dyn ActorContext, msg: BoxMsgAny);
    fn done(&self) -> bool;
}

#[cfg(test)]
mod test {

    #[test]
    fn test_actor() {
        println!("test_actor: empty ATM");
    }
}
