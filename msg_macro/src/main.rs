use custom_logger::env_logger_init;
use msg_header::BoxMsgAny;
use msg_macro::{msg_macro, paste};

// From: https://www.uuidgenerator.net/version4
msg_macro!(MsgA, "d122e9aa-0a69-4654-8e41-e2813bc40272");

fn main() {
    env_logger_init("error");

    let msg_a = MsgA::default();
    println!("MSG_A_ID_STR={MSG_A_ID_STR}");
    println!("msg_a={msg_a:?}");

    let msg_a_any_1: BoxMsgAny = Box::new(msg_a.clone());
    let msg_a_vec = MsgA::to_serde_json_buf(msg_a_any_1).unwrap();
    println!("msg_a_vec={msg_a_vec:x?}");
    println!(
        "msg_a_vec as utf8={:?}",
        std::str::from_utf8(&msg_a_vec).unwrap()
    );
    let msg_a_any_2 = MsgA::from_serde_json_buf(&msg_a_vec).unwrap();
    let msg_a_any_2_deser = MsgA::from_box_msg_any(&msg_a_any_2).unwrap();
    println!("msg_a_any_2_deser={msg_a_any_2_deser:?}");
    assert_eq!(&msg_a, msg_a_any_2_deser);
}
