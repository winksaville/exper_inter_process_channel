//! Protocol for actors that receives CmdInit from issuer.
use an_id::{anid, paste, AnId};
use once_cell::sync::Lazy;
use protocol::Protocol;

// Re-exports
pub use cmd_init::*;

const CMD_INIT_ISSUEE_PROTOCOL_ID: AnId = anid!("151ae493-3b66-433d-8797-68d1029ec3e9");
const CMD_INIT_ISSUEE_PROTOCOL_NAME: &str = "cmd_init_issuee_protocol";
static CMD_INIT_ISSUEE_PROTOCOL_MESSAGES: Lazy<Vec<AnId>> = Lazy::new(|| vec![CMD_INIT_ID]);

static CMD_INIT_ISSUEE_PROTOCOL: Lazy<Protocol> = Lazy::new(|| {
    Protocol::new(
        CMD_INIT_ISSUEE_PROTOCOL_NAME,
        CMD_INIT_ISSUEE_PROTOCOL_ID,
        CMD_INIT_ISSUEE_PROTOCOL_MESSAGES.clone(),
    )
});

pub fn cmd_init_issuee_protocol() -> &'static Protocol {
    &CMD_INIT_ISSUEE_PROTOCOL
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cmd_init_issuee_protocol() {
        let p = cmd_init_issuee_protocol();
        assert_eq!(p.id, CMD_INIT_ISSUEE_PROTOCOL_ID);
        assert_eq!(p.name, CMD_INIT_ISSUEE_PROTOCOL_NAME);
        assert_eq!(p.messages, *CMD_INIT_ISSUEE_PROTOCOL_MESSAGES);
    }
}
