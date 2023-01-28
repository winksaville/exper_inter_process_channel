use custom_logger::env_logger_init;
use msg_header::MsgId;
use msg_macro::{msg_macro, paste};
use std::collections::HashMap;

// From: https://www.uuidgenerator.net/version4
msg_macro!(MsgA, "d122e9aa-0a69-4654-8e41-e2813bc40272");

fn main() {
    env_logger_init("error");

    let msg_a = MsgA::default();
    println!("MSG_A_ID_STR={MSG_A_ID_STR}");
    println!("msg_a={msg_a:?}");

    let msg_a_serde_json_string = msg_a.to_serde_json_string().unwrap();
    println!("{msg_a_serde_json_string}");

    let msg_a_deser = MsgA::from_serde_json_str(&msg_a_serde_json_string).unwrap();
    println!("msg_a_deser={msg_a_deser:?}");
    assert_eq!(msg_a_deser, msg_a);

    let deser = MsgA::from_serde_json_str;

    // Use HashMap to deserialize
    let mut hm = HashMap::<MsgId, fn(&str) -> Option<MsgA>>::new();
    hm.insert(msg_a.id(), deser);
    println!("hm.len()={}", hm.len());
    let fn_from_serde_json_str = hm.get(&msg_a.id()).unwrap();
    let msg_a_deser2 = fn_from_serde_json_str(&msg_a_serde_json_string).unwrap();
    println!("msg_a_deser2={msg_a_deser2:?}");
    assert_eq!(msg_a_deser2, msg_a);

    // Question If the value of the hashmap is a trait
    // object can different return Types be returned?
}
