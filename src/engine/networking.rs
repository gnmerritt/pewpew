use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;
use std::str;
use std::time::Duration;

use bytes::{ByteOrder, LittleEndian};
use futures;
use futures::{Future};
use futures::stream::Stream;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Interval};
use tokio_io::io;
use tokio_io::{AsyncRead};

use game::board::Board;

pub fn launch_server(board: Board) {
    let addr = "127.0.0.1:8888".parse().unwrap();
    println!("Started and listening on {}", addr);
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = TcpListener::bind(&addr, &handle).unwrap();

    let connections = Rc::new(RefCell::new(HashMap::new()));
    let connections1 = connections.clone();

    let srv = socket.incoming().for_each(move |(stream, addr)| {
        println!("New Connection: {}", addr);
        let (_, writer) = stream.split();

        let (tx, rx) = futures::sync::mpsc::unbounded::<Vec<u8>>();
        connections1.borrow_mut().insert(addr, tx);

        // TODO: read from connections too, close when they EOF

        let socket_writer = rx.fold(writer, |writer, msg| {
            // TODO: let len = msg.len();
            let amt = io::write_all(writer, msg);
            let amt = amt.map(|(writer, _)| writer);
            amt.map_err(|_| ())
        });

        let connections = connections1.clone();
        handle.spawn(socket_writer.then(move |_| {
            connections.borrow_mut().remove(&addr);
            println!("Connection {} closed.", addr);
            Ok(())
        }));

        Ok(())
    });

    let handle = core.handle();
    let interval = Interval::new(Duration::from_millis(50), &handle).unwrap();
    let heartbeat = interval.for_each(move |_| {
        for (_, tx) in connections.borrow().deref() {
            let board_bytes = board.to_bytes();
            let to_send = len_encode_bytes(board_bytes);
            tx.send(to_send).unwrap();
        }
        futures::future::ok(())
    });

    core.run(srv.join(heartbeat)).unwrap();
}

/// Prepend a vec of bytes with it's length (4 bytes little endian)
fn len_encode_bytes(mut to_write: Vec<u8>) -> Vec<u8> {
    let len = to_write.len();
    let mut len_buf = [0; 4];
    LittleEndian::write_u32(&mut len_buf, len as u32);
    let mut vec = Vec::with_capacity(4 + len);
    vec.write_all(&len_buf).unwrap();
    vec.append(&mut to_write);
    vec
}

#[cfg(test)]
mod test {
    use std::io::{Read, Write};
    use std::thread;
    use std::net::TcpStream;
    use std::time::Duration;
    use super::*;

    use engine::engine::Round;
    use game::ship::Ship;

    #[test]
    fn test_echo_server() {
        let mut round = Round::new();
        round.board.add_ship(1, Ship::at_origin());
        round.board.add_ship(2, Ship::at_origin());
        let board_bytes = len_encode_bytes(round.board.to_bytes());
        let board = round.board;
        thread::spawn(|| { launch_server(board); });

        thread::sleep(Duration::from_millis(10));
        let client = connect();
        let client2 = connect();

        // wait for the heartbeat to fire, verify both clients received it
        thread::sleep(Duration::from_millis(50));
        verify_heartbeat(&client, &board_bytes);
        verify_heartbeat(&client2, &board_bytes);

        // Should receive a second one
        thread::sleep(Duration::from_millis(50));
        verify_heartbeat(&client, &board_bytes);
        verify_heartbeat(&client2, &board_bytes);
    }

    fn connect() -> TcpStream {
        let client = TcpStream::connect("127.0.0.1:8888").unwrap();
        client.set_read_timeout(Some(Duration::from_secs(1))).expect("setting read timeout failed");
        client.set_nodelay(true).expect("disabling nagle's alg failed");
        client
    }

    fn verify_heartbeat(mut client: &TcpStream, expected: &Vec<u8>) {
        let mut buffer = [0; 512];
        let bytes_read = match client.read(&mut buffer) {
            Ok(read) => read,
            Err(e) => { println!("Got error reading {}", e); 0 }
        };
        assert_eq!(bytes_read, expected.len(), "wrong number of heartbeat bytes");
        let mut read_vec = Vec::with_capacity(bytes_read);
        read_vec.write_all(&buffer[..bytes_read]).expect("made vector");
        assert_eq!(&read_vec, expected);
    }
}
