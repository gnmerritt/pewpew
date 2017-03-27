use std::io::{Read, Write};
use std::time::Duration;
use std::mem;
use std::net::TcpStream;
use bytes::{ByteOrder, LittleEndian};

use super::frame::Frame;

pub struct Client {
    connection: Option<TcpStream>, // TODO: figure out a better way to mock this
    msg_start: Vec<u8>,
    current_frame: Option<Frame>,
}

impl Client {
    pub fn connect() -> Client {
        let client = TcpStream::connect("127.0.0.1:8888").unwrap();
        client.set_read_timeout(Some(Duration::from_millis(1))).expect("setting read timeout failed");
        client.set_nodelay(true).expect("disabling nagle's alg failed");

        Client {
            connection: Some(client),
            msg_start: Vec::with_capacity(4),
            current_frame: None,
        }
    }

    /// Reads and returns any complete frames that have been received by the tcp connection
    pub fn read_frames(&mut self) -> Vec<Frame> {
        let mut buffer = [0; 512];
        let bytes_read;
        {
            let mut connection = self.connection.as_ref().expect("not connected");
            bytes_read = match connection.read(&mut buffer) {
                Ok(read) => read,
                Err(e) => { println!("Got error reading {}", e); 0 }
            };
        }
        let mut results = Vec::new();
        self.process_frame(&buffer[..bytes_read], &mut results);
        results
    }

    fn process_frame(&mut self, buffer: &[u8], results: &mut Vec<Frame>) {
        let len = buffer.len();
        let frame = mem::replace(&mut self.current_frame, None);
        match frame {
            Some(mut frame) => {
                let extra = frame.read(buffer);
                if frame.is_complete() {
                    results.push(frame);
                } else {
                    self.current_frame = Some(frame);
                }
                match extra {
                    Some(bytes) => { self.process_frame(bytes, results) },
                    None => {},
                }
            }
            None => {
                let needed = 4 - self.msg_start.len();
                if len >= needed {
                    self.msg_start.write_all(&buffer[..needed]).unwrap();
                    let frame_len = LittleEndian::read_u32(&self.msg_start);
                    self.msg_start.clear();
                    self.current_frame = Some(Frame::new(frame_len as usize));
                    self.process_frame(&buffer[needed..], results);
                } else {
                    self.msg_start.write_all(buffer).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::networking;
    use super::*;

    fn client() -> Client {
        Client {
           connection: None,
           msg_start: Vec::with_capacity(4),
           current_frame: None,
       }
    }

    #[test]
    fn test_empty() {
        let mut client = client();
        let bytes = Vec::new();
        let mut frames = Vec::new();

        client.process_frame(&bytes, &mut frames);

        assert!(frames.is_empty());
        assert!(client.msg_start.is_empty());
        assert!(client.current_frame.is_none());
    }

    #[test]
    fn test_msg_start() {
        let mut client = client();
        let mut frames = Vec::new();
        let bytes = vec![2, 0, 0];

        client.process_frame(&bytes, &mut frames);

        assert!(frames.is_empty());
        assert!(client.current_frame.is_none());
        assert_eq!(client.msg_start, bytes);
    }

    #[test]
    fn test_partial_data() {
        let data = vec![1, 2, 3, 4, 5];
        let bytes = networking::len_encode_bytes(data.clone());
        assert_eq!(bytes.len(), 4 + 5); // length u32 plus 5 1-byte nums
        let mut client = client();
        let mut frames = Vec::new();

        // Read the first chunk
        client.process_frame(&bytes[..6], &mut frames);
        {
            assert!(frames.is_empty());
            let frame = client.current_frame.as_ref().expect("should have in-progress frame");
            assert!(!frame.is_complete());
            assert_eq!(frame.len(), data.len());
            assert_eq!(frame.bytes(), &vec![1, 2]);
        }

        // and then the rest
        client.process_frame(&bytes[6..], &mut frames);

        assert_eq!(1, frames.len());
        let frame = &frames[0];
        assert_eq!(frame.bytes(), &data);
    }

    #[test]
    fn test_frame_reader() {
        let data = vec![1, 2, 3, 4, 5];
        let bytes = networking::len_encode_bytes(data.clone());
        assert_eq!(bytes.len(), 4 + 5); // length u32 plus 5 1-byte nums
        let mut client = client();
        let mut frames = Vec::new();

        client.process_frame(&bytes, &mut frames);

        assert_eq!(1, frames.len());
        let frame = &frames[0];
        assert!(frame.is_complete());
        assert_eq!(frame.bytes(), &data);
        assert_eq!(frame.len(), data.len());
    }

    #[test]
    fn test_multiple_frames() {
        let data1 = vec![1, 2, 3, 4, 5];
        let mut bytes1 = networking::len_encode_bytes(data1.clone());
        let data2 = vec![6, 7, 8, 9, 10, 11];
        let mut bytes2 = networking::len_encode_bytes(data2.clone());
        let mut client = client();
        let mut frames = Vec::new();
        let mut both = Vec::with_capacity(bytes1.len() + bytes2.len());
        both.append(&mut bytes1);
        both.append(&mut bytes2);

        client.process_frame(&both, &mut frames);

        assert_eq!(2, frames.len());
        assert_eq!(data1.len(), frames[0].len());
        assert_eq!(data2.len(), frames[1].len());
    }
}
