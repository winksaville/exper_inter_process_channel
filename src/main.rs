use crossbeam_channel::{unbounded, Sender};
use msg1::{Msg1, MSG1_ID};
use msg2::{Msg2, MSG2_ID};
use msg_header::MsgId;
use sm::{MsgAny, ProcessMsgAny};
use sm_channel_to_network::SmChannelToNetwork;
use sm_network_to_channel::SmNetworkToChannel;

fn inter_process_channel(_msg_list: Vec<MsgId>) -> Sender<Box<MsgAny>> {
    let (tx, _rx) = unbounded::<Box<MsgAny>>();

    tx
}

fn main() {
    println!("main:+");

    let mut c2n = SmChannelToNetwork::new("c2n", SmChannelToNetwork::state0);
    c2n.add_state(SmChannelToNetwork::state0, "state0");
    c2n.add_state(SmChannelToNetwork::state1, "state1");
    println!("c2n={c2n:?}");

    let msg1 = Box::<Msg1>::default();
    println!("msg1: {msg1:?}");

    let msg2 = Box::<Msg2>::default();
    println!("msg1: {msg2:?}");

    c2n.process_msg_any(msg1.clone());
    c2n.process_msg_any(msg2.clone());
    c2n.process_msg_any(msg2.clone());
    c2n.process_msg_any(msg1.clone());

    let mut n2c = SmNetworkToChannel::new("n2c", SmNetworkToChannel::state0);
    n2c.add_state(SmNetworkToChannel::state0, "state0");
    n2c.add_state(SmNetworkToChannel::state1, "state1");
    println!("n2c={n2c:?}");

    n2c.process_msg_any(msg1.clone());
    n2c.process_msg_any(msg2.clone());
    n2c.process_msg_any(msg2.clone());
    n2c.process_msg_any(msg1.clone());

    let msg_ids = vec![MSG1_ID, MSG2_ID];
    let tx = inter_process_channel(msg_ids);

    // Send message, no receivers yet so ignore results
    _ = tx.send(msg1.clone());
    _ = tx.send(msg2.clone());

    drop(msg1);
    drop(msg2);
    println!("main:-");
}
