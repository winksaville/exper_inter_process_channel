use std::{error::Error, io::Write, net::TcpStream};

pub fn buf_u8_le_to_u16(buf: &[u8; 2]) -> u16 {
    let b0 = buf[0] as u16;
    let b1 = buf[1] as u16;
    b0 + (b1 << 8)
}

pub fn u16_to_buf_u8_le(v: u16) -> [u8; 2] {
    let b0 = (v & 0xff) as u8;
    let b1 = ((v >> 8) & 0xff) as u8;
    [b0, b1]
}

pub fn write_msg_buf_to_tcp_stream(
    stream: &mut TcpStream,
    msg_buf: &[u8],
) -> Result<(), Box<dyn Error>> {
    let buf_len_data = u16_to_buf_u8_le(msg_buf.len() as u16);

    stream.write_all(buf_len_data.as_ref())?;
    stream.write_all(msg_buf)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{io::Read, net::TcpListener, thread};

    use super::*;

    #[test]
    fn test_buf_u8_conversions() {
        println!("\ntest_buf_u8_conversions:+");

        let buf = u16_to_buf_u8_le(513);
        let v = super::buf_u8_le_to_u16(&buf);
        assert_eq!(v, 513);

        println!("test_buf_u8_conversions:-");
    }

    #[test]
    fn test_write_msg_buf_to_tcp_stream() {
        println!("\ntest_write_msg_buf_to_tcp_stream:+");

        let (tx, rx) = std::sync::mpsc::channel();
        let (status_tx, status_rx) = std::sync::mpsc::channel();

        // Spawn a thread that will:
        //  * write to status_tx so main thread knows we're ready
        //  * listen for a connection
        //  * use write_msg_buf_to_tcp_stream to send data
        //  * wait for main thread to read the data
        thread::spawn(move || {
            println!("test_write_msg_buf_to_tcp_stream thread:+");
            let listener = TcpListener::bind("127.0.0.1:12345").unwrap();

            // Tell the main thread that we are ready.
            status_tx.send(()).unwrap();

            let (mut stream, _) = listener.accept().unwrap();

            println!("test_write_msg_buf_to_tcp_stream thread: connected, write data");
            let msg_buf = vec![0x01, 0x02, 0x03];
            super::write_msg_buf_to_tcp_stream(&mut stream, msg_buf.as_ref()).unwrap();

            // Wait for the main thread to read the data.
            println!("test_write_msg_buf_to_tcp_stream thread: wait for main thread");
            _ = rx.recv().unwrap();
            println!("test_write_msg_buf_to_tcp_stream thread:-");
        });

        // Wait for the thread to be ready.
        println!("test_write_msg_buf_to_tcp_stream: wait for thread");
        _ = status_rx.recv().unwrap();

        println!("test_write_msg_buf_to_tcp_stream: connect to thread");
        // Create a TcpStream that is backed by an in memory buffer.
        let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap();

        println!("test_write_msg_buf_to_tcp_stream: read length");
        let mut buf = [0; 2];
        stream.read_exact(&mut buf).unwrap();
        println!(
            "test_write_msg_buf_to_tcp_stream: read buffer buf={:x?}",
            buf
        );
        let len = buf_u8_le_to_u16(&buf);

        println!("test_write_msg_buf_to_tcp_stream: read buffer len={len} and verify");
        let mut buf = vec![0; len as usize];
        stream.read_exact(&mut buf).unwrap();
        assert_eq!(buf, vec![0x01, 0x02, 0x03]);

        // Tell the thread to terminate.
        tx.send(()).unwrap();
        println!("test_write_msg_buf_to_tcp_stream:-");
    }
}
