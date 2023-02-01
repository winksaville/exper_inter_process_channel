//use std::{
//    io::{Read, Write},
//    net::{TcpListener, TcpStream},
//    str::from_utf8,
//    thread, collections::HashMap, error::Error,
//};
//
//use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
//use custom_logger::env_logger_init;
//use msg1::{Msg1, MSG1_ID};
//use msg2::{Msg2, MSG2_ID};
//use msg_header::MsgId;
//use msg_serde_json::FromSerdeJsonBuf;
//use sm::{BoxMsgAny, ProcessMsgAny};
//use client::Client;

//fn buf_u8_le_to_u16(buf: &[u8; 2]) -> u16 {
//    let b0 = buf[0] as u16;
//    let b1 = buf[1] as u16;
//    b0 + (b1 << 8)
//}

//fn u16_to_buf_u8_le(v: u16) -> Vec<u8> {
//    let b0 = (v & 0xff) as u8;
//    let b1 = ((v >> 8) & 0xff) as u8;
//    vec![b0, b1]
//}

//fn write_msg_buf_to_tcp_stream(stream: &mut TcpStream, msg_buf: &[u8]) {
//    let buf_len_data = u16_to_buf_u8_le(msg_buf.len() as u16);
//
//    stream
//        .write_all(buf_len_data.as_ref())
//        .expect("tickle_ipchnl_deser: Couldn't write length");
//    stream
//        .write_all(msg_buf)
//        .expect("tickle_ipchnl_deser: Couldn't write data");
//}

//struct ServiceRec {
//    name: String,
//    tx: Sender<BoxMsgAny>,
//    msg_deser_map: HashMap::<String, FromSerdeJsonBuf>,
//}

//fn ipchnl_deser(ip_address_port: &str, service_rec: ServiceRec) -> Result<Receiver<BoxMsgAny>, Box<dyn Error>> {
//    let (status_tx, status_rx) = bounded(1);
//    let (tx, rx) = unbounded::<BoxMsgAny>();
//
//    thread::spawn(move || {
//        println!("ipchnl_deser:+");
//
//        // Ignore errors for the moment
//        let listener = TcpListener::bind(ip_address_port).unwrap();
//
//        // Indicate we're ready
//        status_tx
//            .send("ready".to_owned())
//            .expect("inter_process_channel_receiver: Unable to indicate we're ready");
//
//        for stream in listener.incoming() {
//            match stream {
//                Ok(mut tcp_stream) => {
//                    // For now spin up a separate thread for each connection
//                    // ALTHOUGH, there is only one connection ATM
//                    //let stream_status_tx = status_tx.clone();
//                    thread::spawn(move || {
//                        println!("ipchnl_deser stream:+");
//
//                        loop {
//                            // TODO: Probably need a signature and version indicator too.
//                            let mut msg_len_buf = [0u8; 2];
//                            if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
//                                println!("ipchnl_deser stream: stream closed reading msg_len, stopping");
//                                break;
//                            }
//
//                            let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
//                            println!("ipchnl_deser stream: msg_len={msg_len}");
//
//                            // We need to initialize the Vec so read_exact knows how much to read.
//                            // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
//                            let mut msg_buf = vec![0; msg_len];
//                            if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
//                                println!("ipchnl_deser stream: stream close reading msg_buf, stopping");
//                                break;
//                            }
//
//                            print!("ipchnl_deser stream: msg_buf=");
//                            let msg_buf_str = if let Ok(str) = from_utf8(&msg_buf) {
//                                println!("{str}");
//                            } else {
//                                panic!("Expected msg_buf to be utf8, but it was {msg_buf:x?}");
//                            };
//
//                            //// Convert the buffer as msg any
//                            //let id_str = get_
//                            //service_rec.msg_deser_map.get()
//                            //if let Some(msg1) = Msg1::from_serde_json_buf(&msg_buf) {
//                            //    println!("msg1={:?}", Msg1::from_box_msg_any(&msg1).unwrap());
//                            //} else if let Some(msg2) = Msg2::from_serde_json_buf(&msg_buf) {
//                            //    println!("msg2={:?}", Msg2::from_box_msg_any(&msg2).unwrap());
//                            //} else {
//                            //    println!("Error converting serde_json");
//                            //}
//
//                            //stream_status_tx.clone().send("completed".to_owned()).expect("inter_process_channel_receiver: Unable to indicate we're completed");
//                        }
//                        println!("ipchnl_deser stream:-");
//                    });
//                }
//                Err(why) => {
//                    println!("ipchnl_deser stream: Error accepting connection: {why}");
//                }
//            }
//        }
//
//        println!("ipchnl_deser:-");
//    });
//
//    Ok(rx)
//}
//
//// Test that we can use a channel to receive BoxMsgAny across thread boundaries
//fn ipchnl(_msg_list: Vec<MsgId>) -> (Sender<BoxMsgAny>, Receiver<String>) {
//    let (tx, rx) = unbounded::<BoxMsgAny>();
//
//    let (complete_tx, status_rx) = bounded(1);
//    thread::spawn(move || {
//        println!("c2n_ipchnl:+");
//        let mut c2n = Client::new("c2n", Client::state0);
//
//        // Indicate we're ready
//        complete_tx
//            .send("ready".to_owned())
//            .expect("c2n_ipchnl: Unable to indicate we're ready");
//
//        println!("c2n_ipchnl: Waiting  msg");
//        while let Ok(msg) = rx.recv() {
//            println!("c2n_ipchnl: Received msg");
//            c2n.process_msg_any(msg);
//            println!("c2n_ipchnl: Waiting  msg");
//            complete_tx
//                .send("completed".to_owned())
//                .expect("c2n_ipchnl: Unable to indicate we're completed processing");
//        }
//        println!("c2n_ipchnl:-");
//    });
//
//    (tx, status_rx)
//}
//
//fn tickle_ipchnl() {
//    println!("tickle_ipchnl:+");
//    let msg1 = Box::<Msg1>::default();
//    let msg2 = Box::<Msg2>::default();
//
//    // Start inter_process_channel
//    let msg_ids = vec![MSG1_ID, MSG2_ID];
//    let (tx, status_rx) = ipchnl(msg_ids);
//
//    let msg = status_rx
//        .recv()
//        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
//    assert_eq!("ready", msg.as_str());
//    println!("tickle_ipchnl: inter_process_channel is READY");
//
//    // Send msg1 wait for it to be processed
//    _ = tx.send(msg1.clone());
//    let msg = status_rx
//        .recv()
//        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
//    assert_eq!("completed", msg.as_str());
//    println!("tickle_ipchnl: completed msg1");
//
//    // Send msg2 wait for it to be processed
//    _ = tx.send(msg2.clone());
//    status_rx
//        .recv()
//        .expect("tickle_ipchnl: Error waiting for inter_process_channel to be ready");
//    assert_eq!("completed", msg.as_str());
//    println!("tickle_ipchnl: completed msg2");
//
//    drop(msg1);
//    drop(msg2);
//    println!("tickle_ipchnl:-");
//}

//fn tickle_ipchnl_deser() {
//    println!("tickle_ipchnl_deser:+");
//
//    // Start inter_process_channel_receiver
//    let (ip_address_port, status_rx) = ipchnl_deser();
//    let msg = status_rx
//        .recv()
//        .expect("tickle_ipchnl_deser: Error waiting for ipchnl_deser to be ready");
//    assert_eq!("ready", msg.as_str());
//
//    let mut stream =
//        TcpStream::connect(ip_address_port).expect("tickle_ipchnl_deser: Could not connect to ipchnl_deser");
//
//    let msg1 = Box::<Msg1>::default();
//    let msg_buf =
//        Msg1::to_serde_json_buf(msg1.clone()).expect("tickle_ipchnl_deser: Could not serialize msg1");
//    write_msg_buf_to_tcp_stream(&mut stream, &msg_buf);
//
//    let msg = status_rx
//        .recv()
//        .expect("tickle_ipchnl_deser: Error waiting for ipchnl_deser to receive the msg");
//    assert_eq!("completed", msg.as_str());
//
//    let msg2 = Box::<Msg2>::default();
//    let msg_buf =
//        Msg2::to_serde_json_buf(msg2.clone()).expect("tickle_ipchnl_deser: Could not serialize msg2");
//    write_msg_buf_to_tcp_stream(&mut stream, &msg_buf);
//
//    let msg = status_rx
//        .recv()
//        .expect("tickle_ipchnl_deser: Error waiting for ipchnl_deser to receive the msg");
//    assert_eq!("completed", msg.as_str());
//
//    drop(msg1);
//    drop(msg2);
//    println!("tickle_ipchnl_deser:-");
//}

fn main() {
    println!("main:+");
    //env_logger_init("error");

    //tickle_ipchnl();

    //tickle_ipchnl_deser();

    println!("main:-");
}
