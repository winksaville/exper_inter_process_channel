use std::{
    any::{Any, TypeId},
    thread,
    time::Duration,
};

use crossbeam_channel::{unbounded, Sender};
use msg1::{Msg1, MSG1_ID};
use msg2::{Msg2, MSG2_ID};
use msg_header::MsgId;
use sm::{BoxMsgAny, ProcessMsgAny};
use sm_channel_to_network::SmChannelToNetwork;

fn inter_process_channel(_msg_list: Vec<MsgId>) -> Sender<BoxMsgAny> {
    let (tx, rx) = unbounded::<BoxMsgAny>();

    thread::spawn(move || {
        println!("c2n_ipchnl:+");
        let mut c2n = SmChannelToNetwork::new("c2n", SmChannelToNetwork::state0);

        println!("c2n_ipchnl: Waiting  msg");
        while let Ok(msg) = rx.recv() {
            println!("c2n_ipchnl: Received msg");
            c2n.process_msg_any(msg);
            println!("c2n_ipchnl: Waiting  msg");
        }
        println!("c2n_ipchnl:-");
    });

    tx
}

fn main() {
    println!("main:+");

    let msg1 = Box::<Msg1>::default();
    println!("main: msg1: {msg1:?}");
    let ser_msg1 = serde_json::to_string(&msg1).unwrap();
    println!("main: ser_msg1={ser_msg1}");
    let deser_msg1: Box<Msg1> = serde_json::from_str(&ser_msg1).unwrap();
    println!("main: deser_msg1={deser_msg1:?}");
    assert_eq!(msg1.header.id, MSG1_ID);
    assert_eq!(msg1.header.id, deser_msg1.header.id);
    println!(
        "main: TypeId::of::<Msg1>()={:?} msg1.type_id()={:?}",
        TypeId::of::<Msg1>(),
        (*deser_msg1).type_id()
    );
    assert_eq!(TypeId::of::<Msg1>(), (*deser_msg1).type_id());

    let msg2 = Box::<Msg2>::default();
    println!("main: msg1: {msg2:?}");
    let ser_msg2 = serde_json::to_string(&msg2).unwrap();
    println!("main: ser_msg2={ser_msg2}");
    let deser_msg2: Box<Msg2> = serde_json::from_str(&ser_msg2).unwrap();
    println!("main: deser_msg2={deser_msg2:?}");
    assert_eq!(msg2.header.id, MSG2_ID);
    assert_eq!(msg2.header.id, deser_msg2.header.id);
    println!(
        "main: TypeId::of::<Msg2>()={:?} msg2.type_id()={:?}",
        TypeId::of::<Msg2>(),
        (*deser_msg2).type_id()
    );
    assert_eq!(TypeId::of::<Msg2>(), (*deser_msg2).type_id());

    // CAREFUL: Deserializing ser_msg2 to a Msg1 "works" because their json represation is idential
    let msg2 = Box::<Msg2>::default();
    let ser_msg2 = serde_json::to_string(&msg2).unwrap();
    let deser_bad_msg2: Box<Msg1> = serde_json::from_str(&ser_msg2).unwrap();
    assert_ne!(TypeId::of::<Msg2>(), (*deser_bad_msg2).type_id()); // This is NOT EQUAl
    assert_eq!(TypeId::of::<Msg1>(), (*deser_bad_msg2).type_id()); // This is EQUAL
    assert_eq!(deser_bad_msg2.header.id, MSG2_ID); // BUT THE header.id is from MSG2_ID!!!!!!

    let msg_ids = vec![MSG1_ID, MSG2_ID];
    let tx = inter_process_channel(msg_ids);

    // Send two messages
    _ = tx.send(msg1.clone());
    _ = tx.send(msg2.clone());

    println!("main: Waiting for 1 sec");
    thread::sleep(Duration::from_secs(1));

    drop(msg1);
    drop(msg2);
    println!("main:-");
}
