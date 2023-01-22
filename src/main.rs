use msg1::Msg1;
use msg2::Msg2;
use msg_header::MsgHeader;
use sm::ProcessMsgAny;
use sm_channel_to_network::SmChannelToNetwork;
use sm_network_to_channel::SmNetworkToChannel;

fn main() {
    println!("main:+");

    let mut c2n = SmChannelToNetwork::new("c2n", SmChannelToNetwork::state0);
    c2n.add_state(SmChannelToNetwork::state0, "state0");
    c2n.add_state(SmChannelToNetwork::state1, "state1");
    println!("c2n={c2n:?}");

    let hdr = MsgHeader { id: 0 };
    println!("hdr: {hdr:?}");

    let msg1 = Box::new(Msg1 {
        header: MsgHeader { id: 1 },
    });
    println!("msg1: {msg1:?}");

    let msg2 = Box::new(Msg2 {
        header: MsgHeader { id: 2 },
    });
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
    n2c.process_msg_any(msg2);
    n2c.process_msg_any(msg1);

    println!("main:-");
}
