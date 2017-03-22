use std::io;
use std::str;
use bytes::{BytesMut};
use futures::{future, Future, BoxFuture};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
       if let Some(i) = buf.iter().position(|&b| b == b'\n') {
           // remove the serialized frame from the buffer.
           let line = buf.split_to(i);

           // Also remove the '\n'
           buf.split_to(1);

           // Turn this data into a UTF string and return it in a Frame.
           match str::from_utf8(&line) {
               Ok(s) => Ok(Some(s.to_string())),
               Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                            "invalid UTF-8")),
           }
       } else {
           Ok(None)
       }
   }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
       buf.extend(msg.as_bytes());
       buf.extend(b"\n");
       Ok(())
   }
}

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = String;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

pub struct Echo;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}

pub struct Server;

impl Server {
    pub fn listen() {
        let addr = "127.0.0.1:8888".parse().unwrap();
        let server = TcpServer::new(LineProto, addr);
        server.serve(|| Ok(Echo));
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::net::TcpStream;
    use std::time::Duration;
    use std::io::{Read, Write};
    use super::*;

    #[test]
    fn test_echo_server() {
        thread::spawn(|| { Server::listen(); });
        thread::sleep(Duration::from_millis(50));

        // Client code:
        let mut client = connect();
        client = verify_echo(client, "some characters to send in\n");

        let client2 = connect();
        verify_echo(client2, "test string to client 2\n");

        verify_echo(client, "more things on client one\n");
    }

    fn connect() -> TcpStream {
        let client = TcpStream::connect("127.0.0.1:8888").unwrap();
        client.set_read_timeout(Some(Duration::from_secs(1))).expect("setting read timeout failed");
        client.set_nodelay(true).expect("disabling nagle's alg failed");
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
        assert_eq!(bytes_read, length, "didn't read right number of bytes");
        let parsed = String::from_utf8_lossy(&buffer[0 .. length]);
        assert_eq!(parsed.into_owned(), test_str, "didn't get correct string echoed");

        client
    }
}
