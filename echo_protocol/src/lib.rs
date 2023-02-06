use std::collections::HashMap;

use echo_complete::ECHO_COMPLETE_ID;
use echo_reply::ECHO_REPLY_ID;
use echo_req::ECHO_REQ_ID;
use echo_start::ECHO_START_ID;
use msg_header::MsgId;
use once_cell::sync::Lazy;
use protocol::{ProtocolId, ProtocolRec};
use uuid::uuid;

const ECHO_PROTOCOL_ID: ProtocolId = ProtocolId(uuid!("2084ca39-77f0-4ba0-b3f9-693f529e727b"));
const ECHO_PROTOCOL_NAME: &str = "echo_protocol";
static ECHO_PROTOCOL_PROTOCOLS: Lazy<Vec<ProtocolId>> = Lazy::new(|| vec![ECHO_PROTOCOL_ID]);
static ECHO_PROTOCOL_MESSAGES: Lazy<Vec<MsgId>> =
    Lazy::new(|| vec![ECHO_START_ID, ECHO_REQ_ID, ECHO_REPLY_ID, ECHO_COMPLETE_ID]);

static ECHO_PROTOCOL_MESSAGES_MAP: Lazy<HashMap<ProtocolId, Vec<MsgId>>> = Lazy::new(|| {
    let messages = ECHO_PROTOCOL_MESSAGES.clone();
    let mut hm = HashMap::new();
    hm.insert(ECHO_PROTOCOL_ID, messages);

    hm
});

static ECHO_PROTOCOL: Lazy<EchoProtocol> = Lazy::new(|| {
    let mp = ECHO_PROTOCOL_MESSAGES_MAP.clone();
    ProtocolRec::new(
        ECHO_PROTOCOL_NAME,
        ECHO_PROTOCOL_ID,
        ECHO_PROTOCOL_PROTOCOLS.clone(),
        mp,
    )
});

pub type EchoProtocol = ProtocolRec;

pub fn echo_protocol() -> &'static EchoProtocol {
    &ECHO_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;
    use protocol::Protocol;

    #[test]
    fn test_echo_protocol() {
        let ep = echo_protocol();
        assert_eq!(ep.id(), &ECHO_PROTOCOL_ID);
        assert_eq!(ep.name(), ECHO_PROTOCOL_NAME);
        assert_eq!(ep.protocols(), &*ECHO_PROTOCOL_PROTOCOLS);
        let ep_id = &ep.protocols()[0];
        let ep_msgs = ep.messages(ep_id).unwrap();
        assert_eq!(ep_msgs, &*ECHO_PROTOCOL_MESSAGES);
    }
}
