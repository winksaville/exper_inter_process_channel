use msg_macro::{msg_macro, paste};

// https://www.uuidgenerator.net/version4
msg_macro!(EchoStart "f13f7f26-40eb-4c94-a408-c455677f6730" {
    ping_count: u64
});

impl EchoStart {
    pub fn new(ping_count: u64) -> Self {
        Self {
            header: msg_header::MsgHeader { id: ECHO_START_ID },
            ping_count,
        }
    }
}
