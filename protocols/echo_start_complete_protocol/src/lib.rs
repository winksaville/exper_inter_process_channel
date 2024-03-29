use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use echo_complete::*;
pub use echo_start::*;

const ECHO_START_COMPLETE_PROTOCOL_ID: AnId = anid!("e46194bc-65a4-4b8e-884a-4272fd8fae99");
const ECHO_START_COMPLETE_PROTOCOL_NAME: &str = "echo_start_complete_protocol";
static ECHO_START_COMPLETE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> =
    Lazy::new(|| vec![ECHO_START_ID, ECHO_COMPLETE_ID]);

static ECHO_START_COMPLETE_PROTOCOL: Lazy<EchoStartCompleteProtocol> = Lazy::new(|| {
    Protocol::new(
        ECHO_START_COMPLETE_PROTOCOL_NAME,
        ECHO_START_COMPLETE_PROTOCOL_ID,
        ECHO_START_COMPLETE_PROTOCOL_MESSAGES.clone(),
    )
});

pub type EchoStartCompleteProtocol = Protocol;

pub fn echo_start_complete_protocol() -> &'static EchoStartCompleteProtocol {
    &ECHO_START_COMPLETE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_echo_start_complete_protocol() {
        let ep = echo_start_complete_protocol();
        assert_eq!(&ep.id, &ECHO_START_COMPLETE_PROTOCOL_ID);
        assert_eq!(ep.name, ECHO_START_COMPLETE_PROTOCOL_NAME);
        assert_eq!(&ep.messages, &*ECHO_START_COMPLETE_PROTOCOL_MESSAGES);
    }
}
