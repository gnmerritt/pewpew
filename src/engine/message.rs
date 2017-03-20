use std::io::Write;

/**
 * A helper struct to aid in reading messages of a known length from a stream of bytes
 */
pub struct Message {
    length: usize,
    read: usize,
    bytes: Vec<u8>,
}

impl Message {
    pub fn new(length: usize) -> Message {
        Message {
            length: length,
            read: 0,
            bytes: Vec::with_capacity(length),
        }
    }

    pub fn read(&mut self, new_bytes: &[u8]) -> Option<Vec<u8>> {
        let needed = self.length - self.read;
        let incoming = new_bytes.len();

        if incoming <= needed {
            self.bytes.write_all(new_bytes).unwrap();
            self.read += incoming;
            None
        } else {
            let (to_read, extra) = new_bytes.split_at(needed);
            self.bytes.write_all(to_read).unwrap();
            self.read += needed;
            let mut extra_vec = Vec::with_capacity(extra.len());
            extra_vec.write_all(extra).unwrap();
            Some(extra_vec)
        }
    }

    fn is_complete(&self) -> bool {
        self.length == self.read
    }
}


#[cfg(test)]
mod test {
    use bincode::{serialize, deserialize, SizeLimit};
    use super::Message;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestObj {
        x: i32,
        y: f32
    }

    #[test]
    fn empty_msg() {
        let message = Message::new(35);
        assert_eq!(35, message.length);
        assert_eq!(0, message.read);
        assert!(message.bytes.is_empty());
        assert!(!message.is_complete());
    }

    #[test]
    fn read_from_bytes() {
        let obj = TestObj { x: 15, y: 9.3 };
        let encoded: Vec<u8> = serialize(&obj, SizeLimit::Infinite).expect("serialization failed");
        let total_length = encoded.len();
        assert_eq!(total_length, 8);

        let mut with_extra = encoded.clone();
        with_extra.append(&mut encoded.clone());
        assert_eq!(with_extra.len(), 2 * encoded.len());

        let mut message = Message::new(total_length);
        let extra = message.read(with_extra.as_slice());
        assert_eq!(extra.unwrap(), encoded);
        assert!(message.is_complete());
        assert_eq!(encoded, message.bytes);

        let decoded: TestObj = deserialize(&message.bytes).unwrap();
        assert_eq!(obj, decoded);
    }

    #[test]
    fn read_partial() {
        let obj = TestObj { x: 999, y: -9.3 };
        let encoded: Vec<u8> = serialize(&obj, SizeLimit::Infinite).expect("serialization failed");
        let total_length = encoded.len();
        assert_eq!(total_length, 8);

        let mut first = encoded.clone();
        let second = first.split_off(4);

        let mut message = Message::new(total_length);
        let extra = message.read(first.as_slice());
        assert!(extra.is_none());
        assert!(!message.is_complete());
        assert_eq!(message.bytes, first);
        assert_eq!(message.read, 4);

        message.read(second.as_slice());
        assert!(extra.is_none());
        assert!(message.is_complete());
        assert_eq!(message.bytes, encoded);

        let decoded: TestObj = deserialize(&message.bytes).unwrap();
        assert_eq!(obj, decoded);
    }
}
