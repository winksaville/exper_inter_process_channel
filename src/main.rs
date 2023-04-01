use an_id::AnId;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use msg_header::{get_msg_id_str_from_buf, BoxMsgAny, FromSerdeJsonBuf, MsgHeader, ToSerdeJsonBuf};

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

fn write_msg_buf_to_tcp_stream(
    stream: &mut TcpStream,
    msg_buf: &[u8],
) -> Result<(), Box<dyn Error>> {
    let buf_len_data = u16_to_buf_u8_le(msg_buf.len() as u16);

    stream.write_all(buf_len_data.as_ref())?;
    stream.write_all(msg_buf)?;

    Ok(())
}

pub struct IpchnlDeserializer {
    pub name: String,                                     // Name of this deserializer
    pub ip_address_port: String,                          // IP Address of this deserializer
    pub tx: Sender<BoxMsgAny>,                            // A channel to send messages to
    pub msg_deser_map: HashMap<String, FromSerdeJsonBuf>, // Map of MsgId of each message
}

#[allow(unused)]
impl IpchnlDeserializer {
    pub fn new(name: &str, ip_address_port: &str, tx: Sender<BoxMsgAny>) -> Self {
        Self {
            name: name.to_owned(),
            ip_address_port: ip_address_port.to_owned(),
            tx,
            msg_deser_map: HashMap::<String, FromSerdeJsonBuf>::new(),
        }
    }

    pub fn add_from_serde_json_buf(
        &mut self,
        msg_id: AnId,
        from_serde_json_buf: FromSerdeJsonBuf,
    ) -> Option<FromSerdeJsonBuf> {
        self.msg_deser_map
            .insert(msg_id.to_string(), from_serde_json_buf)
    }

    /// Reads messages from a TcpStream, deserializes them and sends them to an associated channel
    pub fn deserializer(self) -> Result<Receiver<BoxMsgAny>, Box<dyn Error>> {
        let (tx, rx) = unbounded::<BoxMsgAny>();
        let (status_tx, status_rx) = bounded::<String>(1);

        let self_name = self.name.clone();
        thread::spawn(move || {
            println!("{}::deserializer:+", &self_name);

            // Ignore errors for the moment
            let listener = TcpListener::bind(self.ip_address_port).unwrap();

            // Indicate we're ready
            status_tx.send("ready".to_owned()).unwrap_or_else(|_| {
                panic!(
                    "{}::deserializer: Unable to indicate we're ready",
                    &self.name
                )
            });

            for stream in listener.incoming() {
                match stream {
                    Ok(mut tcp_stream) => {
                        // TODO: Make async, but for now spin up a separate thread for each connection
                        let tx = self.tx.clone();
                        let msg_deser_map = self.msg_deser_map.clone();
                        let self_name = self_name.clone();
                        thread::spawn(move || {
                            println!("{}::deserializer stream:+", &self_name);

                            loop {
                                // TODO: Probably need a signature and version indicator too.
                                let mut msg_len_buf = [0u8; 2];
                                if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
                                    println!(
                                        "{}::deserializer stream: stream closed reading msg_len, stopping",
                                        &self_name
                                    );
                                    break;
                                }

                                let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
                                println!("{}::deserializer stream: msg_len={msg_len}", &self_name);

                                // We need to initialize the Vec so read_exact knows how much to read.
                                // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
                                let mut msg_buf = vec![0; msg_len];
                                if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
                                    println!(
                                        "{}::deserializer stream: stream close reading msg_buf, stopping",
                                        &self_name
                                    );
                                    break;
                                }

                                let id_str = get_msg_id_str_from_buf(&msg_buf);
                                println!("{}::deserializer stream: id_str={id_str}", &self_name);
                                let fn_from_serde_json_buf = msg_deser_map.get(id_str).unwrap();
                                let msg_serde_box_msg_any =
                                    (*fn_from_serde_json_buf)(&msg_buf).unwrap();
                                println!(
                                    "{}::deserializer stream: msg_serde_box_msg_any: {:p} {:p} {} {msg_serde_box_msg_any:?}",
                                    &self_name,
                                    msg_serde_box_msg_any,
                                    &*msg_serde_box_msg_any,
                                    std::mem::size_of::<BoxMsgAny>()
                                );

                                match tx.send(msg_serde_box_msg_any) {
                                    Ok(_) => (),
                                    Err(why) => {
                                        println!(
                                            "{}::deserializer stream: tx.send failed: {why}",
                                            &self_name
                                        );
                                        break;
                                    }
                                }
                            }
                            println!("{}::deserializer stream:-", &self_name);
                        });
                    }
                    Err(why) => {
                        println!("{}::deserializer ipchnl_deser stream: Error accepting connection: {why}", &self_name);
                    }
                }
            }

            println!("{}::deserializer:-", &self_name);
        });

        // Wait for outer thread to be running
        status_rx
            .recv()
            .expect("{}::dserializer error, loop must have died");

        Ok(rx)
    }
}

struct IpchnlSerializer {
    pub name: String,                                  // Name of this serializer
    pub deser_ip_address_port: String,                 // IP Address of IpchnlDeserialize
    pub rx: Receiver<BoxMsgAny>,                       // A channel to receive messages
    msg_serializer_map: HashMap<AnId, ToSerdeJsonBuf>, // Map of AnId to a ToSerdeJsonBuf function
}

#[allow(unused)]
impl IpchnlSerializer {
    pub fn new(name: &str, deser_ip_address_port: &str, rx: Receiver<BoxMsgAny>) -> Self {
        Self {
            name: name.to_owned(),
            deser_ip_address_port: deser_ip_address_port.to_owned(),
            rx,
            msg_serializer_map: HashMap::<AnId, ToSerdeJsonBuf>::new(),
        }
    }

    pub fn add_to_serde_json_buf(
        &mut self,
        msg_id: AnId,
        to_serde_json_buf: ToSerdeJsonBuf,
    ) -> Option<ToSerdeJsonBuf> {
        self.msg_serializer_map.insert(msg_id, to_serde_json_buf)
    }

    /// Receive messages on a channel, serializes them and then writes them to TcpStream
    pub fn serializer(self) -> Result<(), Box<dyn Error>> {
        let (status_tx, status_rx) = bounded(1);
        thread::spawn(move || {
            println!("{}::serializer:+", &self.name);

            // Indicate ready to receive messages
            status_tx
                .send(())
                .expect("{}::serializer: erroring sending status ready");

            // Ignore errors for the moment
            let mut writer = TcpStream::connect(self.deser_ip_address_port).unwrap();

            println!("{}::serializer: Waiting  BoxMsgAny", &self.name);
            while let Ok(msg) = self.rx.recv() {
                println!("{}::serializer: Received msg", &self.name);

                let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg);
                println!("{}::serializer: msg_id={msg_id:?}", &self.name);
                let fn_to_serde_json_buf = self.msg_serializer_map.get(msg_id).unwrap();
                let buf = (*fn_to_serde_json_buf)(msg).unwrap();
                println!(
                    "{}::serializer: {buf:x?}\n{}",
                    &self.name,
                    std::str::from_utf8(&buf).unwrap(),
                );

                match write_msg_buf_to_tcp_stream(&mut writer, &buf) {
                    Ok(_) => (),
                    Err(why) => panic!("{}::serializer: {why}", &self.name),
                }
            }
            println!("{}::serializer:-", &self.name);
        });

        // TODO: Waiting for the serilizer likely not necessary, but what about starting "hangs" or "dies" right away?
        //       if/when this is async we should be able to handle it "properly".
        status_rx
            .recv()
            .expect("{}::serializer error, loop must have died");

        Ok(())
    }
}

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
