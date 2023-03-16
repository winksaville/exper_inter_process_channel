//! Protocol for initialization
//! and send responses.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use cmd_init::*;

const CMD_INIT_PROTOCOL_ID: AnId = anid!("151ae493-3b66-433d-8797-68d1029ec3e9");
const CMD_INIT_PROTOCOL_NAME: &str = "cmd_init_protocol";
static CMD_INIT_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> = Lazy::new(|| vec![CMD_INIT_ID]);

static CMD_INIT_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CMD_INIT_PROTOCOL_NAME,
        CMD_INIT_PROTOCOL_ID,
        CMD_INIT_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn cmd_init_protocol() -> &'static Protocol {
    &CMD_INIT_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_init_protocol() {
        let p = cmd_init_protocol();
        assert_eq!(p.id, CMD_INIT_PROTOCOL_ID);
        assert_eq!(p.name, CMD_INIT_PROTOCOL_NAME);
        assert_eq!(p.messages, *CMD_INIT_PROTOCOL_MESSAGES);
    }
}
