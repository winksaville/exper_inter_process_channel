use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use custom_logger::env_logger_init;
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

fn write_msg_str_to_tcp_stream(stream: &mut TcpStream, msg_str: &str) {
    let buf_len_data = u16_to_buf_u8_le(msg_str.len() as u16);

    stream
        .write_all(buf_len_data.as_ref())
        .expect("tickle_ipchnlr: Couldn't write length");
    stream
        .write_all(msg_str.as_bytes())
        .expect("tickle_ipchnlr: Couldn't write data");
}

// Return string representing the remote "ip_address:port"
fn ipchnlr() -> (String, Receiver<String>) {
    let ip_address_port = "127.0.0.1:54321";

    let (status_tx, status_rx) = bounded(1);

    thread::spawn(move || {
        println!("ipchnlr:+");

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
                        println!("ipchnlr stream:+");

                        loop {
                            // TODO: Probably need a signature and version indicator too.
                            let mut msg_len_buf = [0u8; 2];
                            if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
                                println!("ipchnlr stream: stream closed reading msg_len, stopping");
                                break;
                            }

                            let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
                            println!("ipchnlr stream: msg_len={msg_len}");

                            // We need to initialize the Vec so read_exact knows how much to read.
                            // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
                            let mut msg_buf = vec![0; msg_len];
                            if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
                                println!("ipchnlr stream: stream close reading msg_buf, stopping");
                                break;
                            }
                            print!("ipchnlr stream: msg_buf=");
                            if let Ok(msg_buf_str) = from_utf8(&msg_buf) {
                                println!("{msg_buf_str}");
                            } else {
                                println!("{msg_buf:x?}");
                            }

                            if let Some(msg1) = Msg1::from_serde_json_buf(&msg_buf) {
                                println!("msg1={msg1:?}");
                            } else if let Some(msg2) = Msg2::from_serde_json_buf(&msg_buf) {
                                println!("msg2={msg2:?}");
                            } else {
                                println!("Error converting serde_json");
                            }

                            stream_status_tx.clone().send("completed".to_owned()).expect("inter_process_channel_receiver: Unable to indicate we're completed");
                        }
                        println!("ipchnlr stream:-");
                    });
                }
                Err(why) => {
                    println!("ipchnlr stream: Error accepting connection: {why}");
                }
            }
        }

        println!("ipchnlr:-");
    });

    (ip_address_port.to_owned(), status_rx)
}

fn ipchnl(_msg_list: Vec<MsgId>) -> (Sender<BoxMsgAny>, Receiver<String>) {
    let (tx, rx) = unbounded::<BoxMsgAny>();

    let (complete_tx, status_rx) = bounded(1);
    thread::spawn(move || {
        println!("c2n_ipchnl:+");
        let mut c2n = SmChannelToNetwork::new("c2n", SmChannelToNetwork::state0);

        // Indicate we're ready
        complete_tx
            .send("ready".to_owned())
            .expect("c2n_ipchnl: Unable to indicate we're ready");

        println!("c2n_ipchnl: Waiting  msg");
        while let Ok(msg) = rx.recv() {
            println!("c2n_ipchnl: Received msg");
            c2n.process_msg_any(msg);
            println!("c2n_ipchnl: Waiting  msg");
            complete_tx
                .send("completed".to_owned())
                .expect("c2n_ipchnl: Unable to indicate we're completed processing");
        }
        println!("c2n_ipchnl:-");
    });

    (tx, status_rx)
}

fn tickle_ipchnl() {
    println!("tickle_ipchnl:+");
    let msg1 = Box::<Msg1>::default();
    let msg2 = Box::<Msg2>::default();

    // Start inter_process_channel
    let msg_ids = vec![MSG1_ID, MSG2_ID];
    let (tx, status_rx) = ipchnl(msg_ids);

    let msg = status_rx
        .recv()
        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
    assert_eq!("ready", msg.as_str());
    println!("tickle_ipchnl: inter_process_channel is READY");

    // Send msg1 wait for it to be processed
    _ = tx.send(msg1.clone());
    let msg = status_rx
        .recv()
        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
    assert_eq!("completed", msg.as_str());
    println!("tickle_ipchnl: completed msg1");

    // Send msg2 wait for it to be processed
    _ = tx.send(msg2.clone());
    status_rx
        .recv()
        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
    assert_eq!("completed", msg.as_str());
    println!("tickle_ipchnl: completed msg2");

    drop(msg1);
    drop(msg2);
    println!("tickle_ipchnl:-");
}

fn tickle_ipchnlr() {
    println!("tickle_ipchnlr:+");

    // Start inter_process_channel_receiver
    let (ip_address_port, status_rx) = ipchnlr();
    let msg = status_rx
        .recv()
        .expect("tickle_ipchnlr: Error waiting for ipchnlr to be ready");
    assert_eq!("ready", msg.as_str());

    let mut stream =
        TcpStream::connect(ip_address_port).expect("tickle_ipchnlr: Could not connect to ipchnlr");

    let msg1 = Box::<Msg1>::default();
    let msg_str = serde_json::to_string(&msg1).expect("tickle_ipchnlr: Could not serialize msg1");
    write_msg_str_to_tcp_stream(&mut stream, &msg_str);

    let msg = status_rx
        .recv()
        .expect("tickle_ipchnlr: Error waiting for ipchnlr to receive the msg");
    assert_eq!("completed", msg.as_str());

    let msg2 = Box::<Msg2>::default();
    let msg_str = serde_json::to_string(&msg2).expect("tickle_ipchnlr: Could not serialize msg2");
    write_msg_str_to_tcp_stream(&mut stream, &msg_str);

    let msg = status_rx
        .recv()
        .expect("tickle_ipchnlr: Error waiting for ipchnlr to receive the msg");
    assert_eq!("completed", msg.as_str());

    drop(msg1);
    println!("tickle_ipchnlr:-");
}

fn main() {
    println!("main:+");
    env_logger_init("error");

    tickle_ipchnl();

    tickle_ipchnlr();

    println!("main:-");
}
