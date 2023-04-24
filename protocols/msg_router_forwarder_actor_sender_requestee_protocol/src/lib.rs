//! Protocol implemented by entities that receives requests
//! and sends responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use msg_router_forwarder_actor_sender_req::*;
pub use msg_router_forwarder_actor_sender_rsp::*;

// From: https://www.uuidgenerator.net/version4
const MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_ID: AnId =
    anid!("9a86053a-33a6-4e86-bffe-09c03d7f51fc");
const MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_NAME: &str =
    "msg_router_forwarder_actor_sender_requestee_protocol";
static MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| {
        vec![
            MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQ_ID,
            MSG_ROUTER_FORWARDER_ACTOR_SENDER_RSP_ID,
        ]
    });

static MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL: Lazy<
    MsgRouterForwarderActorSenderRequesteeProtocol,
> = Lazy::new(|| {
    Protocol::new(
        MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_NAME,
        MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_ID,
        MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type MsgRouterForwarderActorSenderRequesteeProtocol = Protocol;

pub fn msg_router_forwarder_actor_sender_requestee_protocol(
) -> &'static MsgRouterForwarderActorSenderRequesteeProtocol {
    &MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_msg_router_forwarder_actor_sender_requestee_protocol() {
        let protocol = msg_router_forwarder_actor_sender_requestee_protocol();
        assert_eq!(
            protocol.id,
            MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_ID
        );
        assert_eq!(
            protocol.name,
            MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_NAME
        );
        assert_eq!(
            protocol.messages,
            *MSG_ROUTER_FORWARDER_ACTOR_SENDER_REQUESTEE_PROTOCOL_MESSAGES
        );
    }
}
