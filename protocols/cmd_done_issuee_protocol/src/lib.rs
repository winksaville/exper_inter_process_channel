//! Protocol for actors that receives CmdDone from issuer.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use cmd_done::*;

const CMD_DONE_ISSUEE_PROTOCOL_ID: AnId = anid!("3fc97255-f992-48ca-ac8f-e910e63d6f6a");
const CMD_DONE_ISSUEE_PROTOCOL_NAME: &str = "cmd_done_issuee_protocol";
static CMD_DONE_ISSUEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> = Lazy::new(|| vec![CMD_DONE_ID]);

static CMD_DONE_ISSUEE_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CMD_DONE_ISSUEE_PROTOCOL_NAME,
        CMD_DONE_ISSUEE_PROTOCOL_ID,
        CMD_DONE_ISSUEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn cmd_done_issuee_protocol() -> &'static Protocol {
    &CMD_DONE_ISSUEE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_done_issuee_protocol() {
        let p = cmd_done_issuee_protocol();
        assert_eq!(p.id, CMD_DONE_ISSUEE_PROTOCOL_ID);
        assert_eq!(p.name, CMD_DONE_ISSUEE_PROTOCOL_NAME);
        assert_eq!(p.messages, *CMD_DONE_ISSUEE_PROTOCOL_MESSAGES);
    }
}
