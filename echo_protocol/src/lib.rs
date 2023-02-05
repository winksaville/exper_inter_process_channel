use echo_complete::ECHO_COMPLETE_ID;
use echo_reply::ECHO_REPLY_ID;
use echo_req::ECHO_REQ_ID;
use echo_start::ECHO_START_ID;
use msg_header::MsgId;
use protocol::{Protocol, ProtocolId};
use uuid::uuid;

// TODO: change Protocol to be Generaic or create a macro_rules??

#[derive(Clone, Debug)]
pub struct EchoProtocol(Protocol);

impl Default for EchoProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl EchoProtocol {
    pub fn new() -> Self {
        let messages = vec![ECHO_START_ID, ECHO_REQ_ID, ECHO_REPLY_ID, ECHO_COMPLETE_ID];
        let id = ProtocolId(uuid!("f05d4751-f878-4297-a7f4-54895b8a707d"));

        EchoProtocol(Protocol::new("echo_protocol", id, messages))
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn messages(&self) -> &[MsgId] {
        &self.0.messages
    }

    pub fn id(&self) -> &ProtocolId {
        &self.0.id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_protocol() {
        println!("test_echo_protocol");

        let ep = EchoProtocol::new();
        println!("test_echo_protocol: {ep:?}");
        assert_eq!(ep.name(), "echo_protocol");
        assert_eq!(
            ep.messages(),
            vec![ECHO_START_ID, ECHO_REQ_ID, ECHO_REPLY_ID, ECHO_COMPLETE_ID]
        );
        assert_eq!(
            ep.id(),
            &ProtocolId(uuid!("f05d4751-f878-4297-a7f4-54895b8a707d"))
        );
    }

    #[test]
    fn test_default_echo_protocol() {
        println!("test_default_echo_protocol");

        let ep = EchoProtocol::default();
        println!("test_echo_protocol: {ep:?}");
        assert_eq!(ep.name(), "echo_protocol");
        assert_eq!(
            ep.messages(),
            vec![ECHO_START_ID, ECHO_REQ_ID, ECHO_REPLY_ID, ECHO_COMPLETE_ID]
        );
        assert_eq!(
            ep.id(),
            &ProtocolId(uuid!("f05d4751-f878-4297-a7f4-54895b8a707d"))
        );
    }
}
