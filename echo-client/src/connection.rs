use std::net::SocketAddr;
use std::thread;

use tokio_core::reactor::Core;
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


        let client = tcp.map_err(|_| Error::Line).and_then(|connection| {
            let framed = connection.framed(LineCodec);
            let (network_sender, network_receiver) = framed.split();
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

pub struct MqttStream {
    inner: Framed<TcpStream, LineCodec>,
    last_ping: Instant,
}

impl MqttStream {
    fn new(inner: Framed<TcpStream, LineCodec>) -> Self {
        MqttStream {
            inner: inner,
            last_ping: Instant::now(),
        }
    }
}

impl Stream for MqttStream {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.last_ping.elapsed().as_secs() >= 10 {
            self.close()?;
            return Err(Error::ConnectionReset);
        }

        loop {
            match try_ready!(self.inner.poll()) {
                Some(ref message) if message == "PING" => {
                    self.last_ping = Instant::now();

                    let result = self.inner.start_send("PONG".to_string())?;

                    assert!(result.is_ready());

                    self.inner.poll_complete()?;
                }
                message => return Ok(Async::Ready(message)),
            }
        }
    }
}

impl Sink for MqttStream
{
    type SinkItem = String;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(self.inner.start_send(item)?)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(self.inner.poll_complete()?)
    }
}