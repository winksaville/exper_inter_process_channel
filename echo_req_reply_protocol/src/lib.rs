use echo_reply::ECHO_REPLY_ID;
use echo_req::ECHO_REQ_ID;
use msg_header::MsgId;
use once_cell::sync::Lazy;
use protocol::{Protocol, ProtocolId};
use uuid::uuid;

const ECHO_REQ_REPLY_PROTOCOL_ID: ProtocolId =
    ProtocolId(uuid!("2084ca39-77f0-4ba0-b3f9-693f529e727b"));
const ECHO_REQ_REPLY_PROTOCOL_NAME: &str = "echo_req_reply_protocol";
static ECHO_REQ_REPLY_PROTOCOL_MESSAGES: Lazy<Vec<MsgId>> =
    Lazy::new(|| vec![ECHO_REQ_ID, ECHO_REPLY_ID]);

static ECHO_REQ_REPLY_PROTOCOL: Lazy<EchoReqReplyProtocol> = Lazy::new(|| {
    Protocol::new(
        ECHO_REQ_REPLY_PROTOCOL_NAME,
        ECHO_REQ_REPLY_PROTOCOL_ID,
        ECHO_REQ_REPLY_PROTOCOL_MESSAGES.clone(),
    )
});

pub type EchoReqReplyProtocol = Protocol;

pub fn echo_req_reply_protocol() -> &'static EchoReqReplyProtocol {
    &ECHO_REQ_REPLY_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_req_reply_protocol() {
        let errp = echo_req_reply_protocol();
        assert_eq!(&errp.id, &ECHO_REQ_REPLY_PROTOCOL_ID);
        assert_eq!(errp.name, ECHO_REQ_REPLY_PROTOCOL_NAME);
        assert_eq!(&errp.messages, &*ECHO_REQ_REPLY_PROTOCOL_MESSAGES);
    }
}
