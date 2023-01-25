use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use msg1::{Msg1, MSG1_ID};
use msg2::{Msg2, MSG2_ID};
use msg_header::MsgId;
use sm::{BoxMsgAny, ProcessMsgAny};
use sm_channel_to_network::SmChannelToNetwork;

fn buf_u8_le_to_u16(buf: &[u8; 2]) -> u16 {
    let b0 = buf[0] as u16;
    let b1 = buf[1] as u16;
    b0 + (b1 << 8)
}

fn u16_to_buf_u8_le(v: u16) -> Vec<u8> {
    let b0 = (v & 0xff) as u8;
    let b1 = ((v >> 8) & 0xff) as u8;
    vec![b0, b1]
}

// Return string representing the remote "ip_address:port"
fn inter_process_channel_reciver() -> (String, Receiver<String>) {
    let ip_address_port = "127.0.0.1:54321";

    let (status_tx, status_rx) = bounded(1);

    thread::spawn(move || {
        println!("inter_process_channel_reciver:+");

        // Ignore errors for the moment
        let listener = TcpListener::bind(ip_address_port).unwrap();

        // Indicate we're ready
        status_tx
            .send("ready".to_owned())
            .expect("inter_process_channel_receiver: Unable to indicate we're ready");

        for stream in listener.incoming() {
            match stream {
                Ok(mut tcp_stream) => {
                    // For now spin up a separate thread for each connection
                    // ALTHOUGH, there is only one connection ATM
                    let stream_status_tx = status_tx.clone();
                    thread::spawn(move || {
                        println!("inter_process_channel_reciver stream:+");

                        loop {
                            // TODO: Probably need a signature and version indicator too.
                            let mut msg_len_buf = [0u8; 2];
                            if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
                                println!("inter_process_channel_reciver stream: stream closed reading msg_len, stopping");
                                break;
                            }
                            println!("inter_process_channel_reciver stream: msg_len_buf={msg_len_buf:x?}");

                            let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
                            println!("inter_process_channel_reciver stream: msg_len={msg_len}");

                            // We need to initialize the Vec so read_exact knows how much to read.
                            // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
                            let mut msg_buf = vec![0; msg_len];
                            if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
                                println!("inter_process_channel_reciver stream: stream close reading msg_buf, stopping");
                                break;
                            }
                            println!("inter_process_channel_reciver stream: msg_buf={msg_buf:?}");

                            stream_status_tx.clone().send("completed".to_owned()).expect("inter_process_channel_receiver: Unable to indicate we're completed");
                        }
                        println!("inter_process_channel_reciver stream:-");
                    });
                }
                Err(why) => {
                    println!(
                        "inter_process_channel_reciver stream: Error accepting connection: {why}"
                    );
                }
            }
        }

        println!("inter_process_channel_reciver:-");
    });

    (ip_address_port.to_owned(), status_rx)
}

fn inter_process_channel(_msg_list: Vec<MsgId>) -> (Sender<BoxMsgAny>, Receiver<String>) {
    let (tx, rx) = unbounded::<BoxMsgAny>();

    let (complete_tx, status_rx) = bounded(1);
    thread::spawn(move || {
        println!("c2n_ipchnl:+");
        let mut c2n = SmChannelToNetwork::new("c2n", SmChannelToNetwork::state0);

        // Indicate we're ready
        complete_tx
            .send("ready".to_owned())
            .expect("inter_process_channel: Unable to indicate we're ready");

        println!("c2n_ipchnl: Waiting  msg");
        while let Ok(msg) = rx.recv() {
            println!("c2n_ipchnl: Received msg");
            c2n.process_msg_any(msg);
            println!("c2n_ipchnl: Waiting  msg");
            complete_tx
                .send("completed".to_owned())
                .expect("inter_process_channel: Unable to indicate we're completed processing");
        }
        println!("c2n_ipchnl:-");
    });

    (tx, status_rx)
}

fn main() {
    println!("main:+");

    let msg1 = Box::<Msg1>::default();
    let msg2 = Box::<Msg2>::default();

    // Start inter_process_channel
    let msg_ids = vec![MSG1_ID, MSG2_ID];
    let (tx, status_rx) = inter_process_channel(msg_ids);

    let msg = status_rx
        .recv()
        .expect("main: Error waiting for inter_process_channel to be ready");
    assert_eq!("ready", msg.as_str());
    println!("main: inter_process_channel is READY");

    // Send msg1 wait for it to be processed
    _ = tx.send(msg1.clone());
    let msg = status_rx
        .recv()
        .expect("main: Error waiting for inter_process_channel to be ready");
    assert_eq!("completed", msg.as_str());
    println!("main: completed msg1");

    // Send msg2 wait for it to be processed
    _ = tx.send(msg2.clone());
    status_rx
        .recv()
        .expect("main: Error waiting for inter_process_channel to be ready");
    assert_eq!("completed", msg.as_str());
    println!("main: completed msg2");

    // Start inter_process_channel_receiver
    let (ip_address_port, status_rx) = inter_process_channel_reciver();
    let msg = status_rx
        .recv()
        .expect("main: Error waiting for inter_process_channel_receiver to be ready");
    assert_eq!("ready", msg.as_str());

    let mut stream = TcpStream::connect(ip_address_port)
        .expect("main: Could not connect to inter_process_channel_receiver");
    let data: Vec<u8> = vec![1, 2, 3, 4];
    let buf_len_data = u16_to_buf_u8_le(data.len() as u16);
    stream
        .write_all(buf_len_data.as_ref())
        .expect("main: Couldn't write length");
    stream
        .write_all(data.as_ref())
        .expect("main: Couldn't write data");

    let msg = status_rx
        .recv()
        .expect("main: Error waiting for inter_process_channel_receiver to receive the msg");
    assert_eq!("completed", msg.as_str());

    //println!("\nmain: Type Ctrl-C to stop\n");
    //ctrlc_channel().recv().expect("main: Could not receive from ctrl_channel");
    //println!();

    drop(msg1);
    drop(msg2);
    println!("main:-");
}
