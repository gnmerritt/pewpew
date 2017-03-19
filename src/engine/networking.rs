use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Result};


fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!("server got {} bytes", bytes_read);
        stream.write(&buf[..bytes_read])?;
    }
}

pub struct Server {
    listener: TcpListener,
    children: Vec<thread::JoinHandle<()>>,
}

impl Server {
    pub fn listen() -> Server {
        let addr = "127.0.0.1:8888";
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening on addr: {}", addr);

        Server {
            listener: listener,
            children: vec![],
        }
    }

    pub fn accept_connections(&mut self) {
        println!("Server accepting connections");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Server got a client connection");
                    stream.set_read_timeout(None).expect("set read timeout failed");
                    stream.set_write_timeout(None).expect("write timeout failed");
                    self.children.push(thread::spawn(move || {
                        handle_client(stream).unwrap();
                    }));
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_echo_server() {
        let mut server = Server::listen();
        thread::spawn(move || {
            server.accept_connections();
            ()
        });

        // Client code:

        let mut client = connect();
        client = verify_echo(client, "some characters to send in");

        let mut client2 = connect();
        verify_echo(client2, "test string to client 2");

        verify_echo(client, "more things on client one");
    }

    fn connect() -> TcpStream {
        let mut client = TcpStream::connect("127.0.0.1:8888").unwrap();
        client.set_read_timeout(Some(Duration::from_secs(1))).expect("setting read timeout failed");
        client
    }

    fn verify_echo(mut client: TcpStream, test_str: &str) -> TcpStream {
        // write
        let length = test_str.len();
        client.write(test_str.as_bytes()).expect("write failed");

        // read echo back
        let mut buffer = [0; 512];
        let bytes_read = match client.read(&mut buffer) {
            Ok(read) => read,
            Err(e) => { println!("Got error reading {}", e); 0 }
        };
        assert_eq!(bytes_read, length);
        let parsed = String::from_utf8_lossy(&buffer[0 .. length]);
        assert_eq!(parsed.into_owned(), test_str);

        client
    }
}
