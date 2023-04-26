//! Protocol for actors that sends CmdDone to issuee.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use cmd_done::*;

const CMD_DONE_ISSUER_PROTOCOL_ID: AnId = anid!("dc18a3b3-e3ce-4f55-877c-e838d849a001");
const CMD_DONE_ISSUER_PROTOCOL_NAME: &str = "cmd_done_issuer_protocol";
static CMD_DONE_ISSUER_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> = Lazy::new(|| vec![CMD_DONE_ID]);

static CMD_DONE_ISSUER_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CMD_DONE_ISSUER_PROTOCOL_NAME,
        CMD_DONE_ISSUER_PROTOCOL_ID,
        CMD_DONE_ISSUER_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn cmd_done_issuer_protocol() -> &'static Protocol {
    &CMD_DONE_ISSUER_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_done_issuer_protocol() {
        let p = cmd_done_issuer_protocol();
        assert_eq!(p.id, CMD_DONE_ISSUER_PROTOCOL_ID);
        assert_eq!(p.name, CMD_DONE_ISSUER_PROTOCOL_NAME);
        assert_eq!(p.messages, *CMD_DONE_ISSUER_PROTOCOL_MESSAGES);
    }
}
