use std::net::SocketAddr;
use std::thread;
use std::io;
use std::mem;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpStream;
use tokio_io::AsyncRead;
use tokio_io::codec::Framed;

use futures::sync::mpsc::{self, Receiver, Sender};
use futures::{Future, Sink, Stream, Poll, StartSend, Async};

use error::Error;
use codec::LineCodec;

use std::time::Instant;
pub struct Connection;

impl Connection {
    pub fn start(addr: String) -> Result<Sender<String>, Error> {
        let (command_tx, command_rx) = mpsc::channel::<String>(1000);
        thread::spawn(move || { Self::run(&addr, command_rx); });
        Ok(command_tx)
    }

    fn run(addr: &str, command_rx: Receiver<String>) -> Result<(), Error> {
        let addr: SocketAddr = addr.parse()?;
        let mut reactor = Core::new()?;
        let tcp = TcpStream::connect(&addr, &reactor.handle());
        let handle = reactor.handle();
        let addr = addr.clone();

        let client = tcp.map_err(|_| Error::Line).and_then(|connection| {
            let framed = connection.framed(LineCodec);
            let mqtt_stream = LineStream::new(framed, handle, addr);

            let (network_sender, network_receiver) = mqtt_stream.split();
            let receiver_future = network_receiver.for_each(|msg| {
                    println!("REPLY: {:?}", msg);
                    Ok(())
                })
                .map_err(|e| Error::Io(e));

            let client_to_tcp = command_rx.map_err(|_| Error::Line)
                .and_then(|p| Ok(p))
                .forward(network_sender);
            receiver_future.join(client_to_tcp).map_err(|e| Error::Line)
        });

        let _ = reactor.run(client);
        println!("@@@@@@@@@@@@@@@@@@@@");
        Ok(())
    }
}

pub struct LineStream {
    inner: Framed<TcpStream, LineCodec>,
    last_ping: Instant,
    handle: Handle,
    addr: SocketAddr,
}

impl LineStream {
    fn new(inner: Framed<TcpStream, LineCodec>, handle: Handle, addr: SocketAddr) -> Self {
        LineStream {
            inner: inner,
            last_ping: Instant::now(),
            handle: handle,
            addr: addr,
        }
    }
}

impl Stream for LineStream {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match try_ready!(self.inner.poll()) {
                Some(m) => {
                    println!("***************");
                    return Ok(Async::Ready(Some(m)));
                }
                None => {
                    let mut stream = TcpStream::connect(&self.addr, &self.handle);
                    println!("---------------");
                    let stream = try_ready!(stream.poll());
                    println!("+++++++++++++++");
                    let framed = stream.framed(LineCodec);
                    mem::replace(&mut self.inner, framed);
                    return Ok(Async::NotReady);
                }
            }
        }
    }
}

impl Sink for LineStream {
    type SinkItem = String;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(self.inner.start_send(item)?)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(self.inner.poll_complete()?)
    }
}