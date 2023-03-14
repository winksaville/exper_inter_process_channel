use actor::{ActorId, ActorInstanceId};
use msg_serde_macro::{msg_serde_macro, paste};
use protocol_set::ProtocolSetId;

// From: https://www.uuidgenerator.net/version4
msg_serde_macro!(ConMgrAddActor "b0e83356-fd22-4389-9f2e-586be8ec9719" {
    name: String,
    id: ActorId,
    instance_id: ActorInstanceId,
    protocol_set_id: ProtocolSetId
});

impl ConMgrAddActor {
    pub fn new(
        name: &str,
        id: &ActorId,
        instance_id: &ActorInstanceId,
        protocol_set_id: &ProtocolSetId,
    ) -> Self {
        Self {
            header: msg_header::MsgHeader {
                id: CON_MGR_ADD_ACTOR_ID,
            },
            name: name.to_owned(),
            id: id.clone(),
            instance_id: instance_id.clone(),
            protocol_set_id: protocol_set_id.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use an_id::AnId;
    use protocol_set::ProtocolSetId;

    use super::*;

    #[test]
    fn test_con_mgr_add_actor() {
        let a_id = ActorId(AnId::new());
        let a_instance_id = ActorInstanceId(AnId::new());
        let a_protocol_set_id = ProtocolSetId(AnId::new());

        let msg = ConMgrAddActor::new("cmaa1", &a_id, &a_instance_id, &a_protocol_set_id);
        println!("test_echo_req msg={msg:#?}");

        assert_eq!(msg.header.id, CON_MGR_ADD_ACTOR_ID);
        assert_eq!(msg.name, "cmaa1");
        assert_eq!(msg.id, a_id);
        assert_eq!(msg.instance_id, a_instance_id);
        assert_eq!(msg.protocol_set_id, a_protocol_set_id);
    }
}
