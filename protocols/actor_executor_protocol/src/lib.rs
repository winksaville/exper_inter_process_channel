//! Protocol implemented by entities that that receive requests
//! and send responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use req_add_actor::*;
pub use rsp_add_actor::*;

const ACTOR_EXECUTOR_PROTOCOL_ID: AnId = anid!("907ee4b7-2819-4211-84b1-e01fc940e2f6");
const ACTOR_EXECUTOR_PROTOCOL_NAME: &str = "actor_executor_protocol";
static ACTOR_EXECUTOR_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![REQ_ADD_ACTOR_ID, RSP_ADD_ACTOR_ID]);

static ACTOR_EXECUTOR_PROTOCOL: Lazy<ActorExecutorProtocol> = Lazy::new(|| {
    Protocol::new(
        ACTOR_EXECUTOR_PROTOCOL_NAME,
        ACTOR_EXECUTOR_PROTOCOL_ID,
        ACTOR_EXECUTOR_PROTOCOL_MESSAGES.clone(),
    )
});

pub type ActorExecutorProtocol = Protocol;

pub fn actor_executor_protocol() -> &'static ActorExecutorProtocol {
    &ACTOR_EXECUTOR_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_actor_executor_protocol() {
        let errp = actor_executor_protocol();
        assert_eq!(errp.id, ACTOR_EXECUTOR_PROTOCOL_ID);
        assert_eq!(errp.name, ACTOR_EXECUTOR_PROTOCOL_NAME);
        assert_eq!(errp.messages, *ACTOR_EXECUTOR_PROTOCOL_MESSAGES);
    }
}
