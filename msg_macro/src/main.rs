use msg_macro::{msg_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_macro!(MsgA, "d122e9aa-0a69-4654-8e41-e2813bc40272");

fn main() {
    let msg_a = MsgA::default();
    println!("MSG_A_ID_STR={MSG_A_ID_STR} msg_a={msg_a:?}");

    let msg_a_serde_json_string = msg_a.to_serde_json_string().unwrap();
    println!("{msg_a_serde_json_string}");

    let msg_a_deser = MsgA::from_serde_json_str(&msg_a_serde_json_string).unwrap();
    println!("msg_a_deser={msg_a_deser:?}");
    assert_eq!(msg_a_deser, msg_a);
}
