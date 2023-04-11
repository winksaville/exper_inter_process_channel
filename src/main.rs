use an_id::AnId;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use msg1::{Msg1, MSG1_ID};
use std::{
    collections::HashMap,
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use box_msg_any::BoxMsgAny;
use msg_header::{get_msg_id_str_from_buf, FromSerdeJsonBuf, MsgHeader, ToSerdeJsonBuf};

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

    pub fn add_msg_id_from_serde_json_buf(
        &mut self,
        msg_id: AnId,
        from_serde_json_buf: FromSerdeJsonBuf,
    ) -> Option<FromSerdeJsonBuf> {
        self.msg_deser_map
            .insert(msg_id.to_string(), from_serde_json_buf)
    }

    /// Reads messages from a TcpStream, deserializes them and sends them to an associated channel
    pub fn deserializer(self) -> Result<Receiver<BoxMsgAny>, Box<dyn Error>> {
        println!("{}::deserializer:+", &self.name);
        let (tx, rx) = unbounded::<BoxMsgAny>();
        let (status_tx, status_rx) = bounded::<String>(1);

        let self_name = self.name.clone();
        thread::spawn(move || {
            println!("{}::deserializer_thread:+", &self_name);

            // Ignore errors for the moment
            let listener = TcpListener::bind(self.ip_address_port).unwrap();

            // Indicate we're ready
            status_tx.send("ready".to_owned()).unwrap_or_else(|_| {
                panic!(
                    "{}::deserializer_thread: Unable to indicate we're ready",
                    &self_name
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
                            println!("{}::deserializer_thread stream:+", &self_name);

                            loop {
                                // TODO: Probably need a signature and version indicator too.
                                let mut msg_len_buf = [0u8; 2];
                                if tcp_stream.read_exact(&mut msg_len_buf).is_err() {
                                    println!(
                                        "{}::deserializer_thread stream: stream closed reading msg_len, stopping",
                                        &self_name
                                    );
                                    break;
                                }

                                let msg_len = buf_u8_le_to_u16(&msg_len_buf) as usize;
                                println!(
                                    "{}::deserializer_thread stream: msg_len={msg_len}",
                                    &self_name
                                );

                                // We need to initialize the Vec so read_exact knows how much to read.
                                // TODO: Consider using [read_buf_exact](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_buf_exact).
                                let mut msg_buf = vec![0; msg_len];
                                if tcp_stream.read_exact(msg_buf.as_mut_slice()).is_err() {
                                    println!(
                                        "{}::deserializer_thread stream: stream close reading msg_buf, stopping",
                                        &self_name
                                    );
                                    break;
                                }

                                let id_str = get_msg_id_str_from_buf(&msg_buf);
                                println!(
                                    "{}::deserializer_thread stream: id_str={id_str}",
                                    &self_name
                                );
                                let fn_from_serde_json_buf = msg_deser_map.get(id_str).unwrap();
                                let msg_serde_box_msg_any =
                                    (*fn_from_serde_json_buf)(&msg_buf).unwrap();
                                println!(
                                    "{}::deserializer_thread stream: msg_serde_box_msg_any: {:p} {:p} {} {msg_serde_box_msg_any:?}",
                                    &self_name,
                                    msg_serde_box_msg_any,
                                    &*msg_serde_box_msg_any,
                                    std::mem::size_of::<BoxMsgAny>()
                                );

                                match tx.send(msg_serde_box_msg_any) {
                                    Ok(_) => (),
                                    Err(why) => {
                                        println!(
                                            "{}::deserializer_thread stream: tx.send failed: {why}",
                                            &self_name
                                        );
                                        break;
                                    }
                                }
                            }
                            println!("{}::deserializer_thread stream:-", &self_name);
                        });
                    }
                    Err(why) => {
                        println!("{}::deserializer_thread ipchnl_deser stream: Error accepting connection: {why}", &self_name);
                    }
                }
            }

            println!("{}::deserializer_thread:-", &self_name);
        });

        // Wait for outer thread to be running
        println!(
            "{}::deserializer: Wait for thread to be running",
            &self.name
        );
        status_rx
            .recv()
            .expect("{}::dserializer error, loop must have died");
        println!("{}::deserializer: thread running", &self.name);

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

    pub fn add_msg_id_to_serde_json_buf(
        &mut self,
        msg_id: AnId,
        to_serde_json_buf: ToSerdeJsonBuf,
    ) -> Option<ToSerdeJsonBuf> {
        self.msg_serializer_map.insert(msg_id, to_serde_json_buf)
    }

    /// Receive messages on a channel, serializes them and then writes them to TcpStream
    pub fn serializer(self) -> Result<(), Box<dyn Error>> {
        println!("{}::serializer:+", &self.name);
        let (status_tx, status_rx) = bounded(1);
        let self_name = self.name.clone();
        thread::spawn(move || {
            println!("{}::serializer_thread:+", &self_name);

            // Indicate ready to receive messages
            status_tx.send(()).unwrap_or_else(|_| {
                panic!(
                    "{}::serializer_thread: erroring sending status ready",
                    &self_name
                )
            });

            // Ignore errors for the moment
            let mut writer = TcpStream::connect(self.deser_ip_address_port).unwrap();

            println!("{}::serializer_thread: Waiting  BoxMsgAny", &self_name);
            while let Ok(msg) = self.rx.recv() {
                println!("{}::serializer_thread: Received msg", &self_name);

                let msg_id = MsgHeader::get_msg_id_from_boxed_msg_any(&msg);
                println!("{}::serializer_thread: msg_id={msg_id:?}", &self_name);
                let fn_to_serde_json_buf = self.msg_serializer_map.get(msg_id).unwrap();
                let buf = (*fn_to_serde_json_buf)(msg).unwrap();
                println!(
                    "{}::serializer_thread: {buf:x?}\n{}",
                    &self_name,
                    std::str::from_utf8(&buf).unwrap(),
                );

                match write_msg_buf_to_tcp_stream(&mut writer, &buf) {
                    Ok(_) => (),
                    Err(why) => panic!("{}::serializer_thread: {why}", &self_name),
                }
            }
            println!("{}::serializer_thread:-", &self_name);
        });

        // Wait for thread to be running
        println!(
            "{}::serializer: Waiting for thread to be running",
            &self.name
        );
        status_rx
            .recv()
            .expect("{}::serializer error, loop must have died");
        println!("{}::serializer:- thread running", &self.name);

        Ok(())
    }
}

fn main() {
    println!("main:+");

    let (supervisor_tx, serializer_rx) = unbounded::<BoxMsgAny>();
    let (deseriailzer_tx, supervisor_rx) = unbounded::<BoxMsgAny>();

    // Create deserializer
    let mut deserializer =
        IpchnlDeserializer::new("serializer", "127.0.0.1:12345", deseriailzer_tx.clone());

    // Add the message types that can be deserialized
    deserializer.add_msg_id_from_serde_json_buf(MSG1_ID, Msg1::from_serde_json_buf);

    // Start the deserializer
    deserializer.deserializer().unwrap();

    // Create serializer
    let mut serializer = IpchnlSerializer::new("serializer", "127.0.0.1:12345", serializer_rx);

    // Add the message types that can be serialized
    serializer.add_msg_id_to_serde_json_buf(MSG1_ID, Msg1::to_serde_json_buf);

    // Start the serializer
    serializer.serializer().unwrap();

    // Create a supervisor instance id
    let supervisor_instance_id = AnId::new();

    // Create a boxed msg1 and send it to the serialzer
    let msg1 = Box::new(Msg1::new(&supervisor_instance_id, 123));
    supervisor_tx.send(msg1).unwrap();

    // Wait for the deserializer to receive the msg and send it back to the supervisor
    println!("main: Waiting for msg1 to be received by the deserializer");
    let msg_any = supervisor_rx.recv().unwrap();

    // Convert msg_any to msg1
    println!("main: Converting msg_any to msg1");
    let msg = Msg1::from_box_msg_any(&msg_any).unwrap();

    // Veriy the msg1
    println!("main: verifying msg1");
    assert_eq!(&supervisor_instance_id, &msg.header.src_id);
    assert_eq!(123, msg.v);

    println!("main:-");
}
