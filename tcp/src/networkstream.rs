use std::io::{self, Write, Read, ErrorKind};
use std::net::Shutdown;

use tokio_core::net::TcpStream;
use futures::{Future, Poll, Async};
use error::Error;

pub struct NetworkStream {
    stream: TcpStream,
    request: Vec<u8>,
}

impl NetworkStream {
    pub fn new(stream: TcpStream) -> Self {
        NetworkStream {
            stream: stream,
            request: Vec::with_capacity(4096),
        }
    }

    /// Read from the socket until the status is NotReady
    fn read(&mut self) -> Poll<(), io::Error> {
        println!("read()");
        loop {
            match self.stream.poll_read() {
                Async::Ready(_) => {
                    let n = {
                        try_nb!(self.stream.read(&mut self.request[..]))
                    };
                    if n == 0 {
                        return Err(io::Error::new(ErrorKind::Other, "connection closed"));
                    }
                    println!("{:?}", self.request);
                    // self.request.extend_from_slice(&self.request[0..n]);
                }
                _ => return Ok(Async::NotReady),
            }
        }
    }
}

impl Future for NetworkStream {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            let read = self.read();

            // if the client connection has closed, close the server connection too
            match &read {
                &Err(ref e) => {
                    println!("Client closed connection: {}", e);
                    let _ = self.stream.shutdown(Shutdown::Write);
                }
                _ => {}
            }
            try_ready!(read);
        }
    }
}
