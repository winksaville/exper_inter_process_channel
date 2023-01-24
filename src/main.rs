use std::{thread, time::Duration};

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

    let msg2 = Box::<Msg2>::default();
    println!("main: msg1: {msg2:?}");

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
