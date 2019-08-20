use std::net::SocketAddr;
use std::thread;
use std::io::{self, ErrorKind};
use std::mem;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpStream, TcpStreamNew};
use tokio_io::AsyncRead;
use tokio_io::codec::Framed;

use tokio_timer::*;
use std::time::Duration;

use futures::sync::mpsc::{self, Receiver, Sender};
use futures::{Future, Sink, Poll, StartSend, Async, Stream, AsyncSink};

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

        let client = tcp.map_err(|_| Error::Line)
            .and_then(|connection| {
                let framed = connection.framed(LineCodec);
                let mqtt_stream = LineStream::new(framed, handle, addr);
                let (network_sender, network_receiver) = mqtt_stream.split();
                let receiver_future = network_receiver
                    .for_each(|msg| {
                                  println!("REPLY: {:?}", msg);
                                  Ok(())
                              })
                    .map_err(|e| Error::Io(e));

                let client_to_tcp = command_rx
                    .map_err(|e| {
                        println!("command rx error: {:?}", e);
                        Error::Line
                    })
                    .and_then(|p| Ok(p))
                    .forward(network_sender)
                    .then(|e| {
                        Ok(())
                    }); //ignore errors here

                receiver_future
                    .select(client_to_tcp)
                    .map_err(|e| {
                        Error::Line
                    })
            });

        let _ = reactor.run(client).unwrap();
        Ok(())
    }
}

#[derive(Eq, PartialEq)]
enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
}

pub struct LineStream {
    inner: Framed<TcpStream, LineCodec>,
    last_ping: Instant,
    handle: Handle,
    addr: SocketAddr,
    new: Option<TcpStreamNew>,
    connection: ConnectionState,
    sleep: Option<Sleep>,
}

impl LineStream {
    fn new(inner: Framed<TcpStream, LineCodec>, handle: Handle, addr: SocketAddr) -> Self {
        LineStream {
            inner: inner,
            last_ping: Instant::now(),
            handle: handle,
            addr: addr,
            new: None,
            connection: ConnectionState::Connected,
            sleep: None,
        }
    }
}

impl Stream for LineStream {
    type Item = String;
    type Error = io::Error;


    // Handle reconnections and pings here
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.connection {
                ConnectionState::Connected => {
                    match self.inner.poll() {
                        Ok(Async::Ready(Some(m))) => return Ok(Async::Ready(Some(m))),
                        Ok(Async::Ready(None)) => {
                            println!("ready none");
                            self.connection = ConnectionState::Disconnected;
                            self.sleep = Some(Timer::default().sleep(Duration::new(5, 0)));
                            continue;
                        }
                        // hits this when stream has terminated
                        Ok(Async::NotReady) => {
                            println!("not ready");
                            return Ok(Async::NotReady);
                        }
                        // hits this when there are other errors
                        Err(e) => {
                            println!("poll error = {:?}", e);
                            return Err(e);
                        }
                    }
                }
                ConnectionState::Connecting => {
                    if let Some(ref mut new) = self.new {
                        match new.poll() {
                            Ok(Async::NotReady) => {
                                println!("**************");
                                return Ok(Async::NotReady)
                            }
                            Ok(Async::Ready(stream)) => {
                                let framed = stream.framed(LineCodec);
                                mem::replace(&mut self.inner, framed);
                                self.connection = ConnectionState::Connected;
                                continue;
                            }
                            Err(e) => {
                                println!("reconnect poll error = {:?}", e);
                                self.connection = ConnectionState::Disconnected;
                                self.sleep = Some(Timer::default().sleep(Duration::new(5, 0)));
                            }
                        }
                    }
                }
                ConnectionState::Disconnected => {
                    if let Some(ref mut sleep) = self.sleep {
                        match sleep.poll() {
                            Ok(Async::NotReady) => {
                                println!("--------------------");
                                return Ok(Async::NotReady)
                            }
                            Ok(Async::Ready(_)) => {
                                println!("++++++++++++++++++++");
                                self.new = Some(TcpStream::connect(&self.addr, &self.handle));
                                self.connection = ConnectionState::Connecting;
                                continue;
                            }
                            Err(e) => {
                                println!("!!!!!!!!!!!!!!!!!!!");
                                return Err(io::Error::new(ErrorKind::Other, "Timer Sleep Failed"))
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Sink for LineStream {
    type SinkItem = String;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        match self.inner.start_send(item) {
            Ok(AsyncSink::NotReady(t)) => Ok(AsyncSink::NotReady(t)),
            Ok(AsyncSink::Ready) => Ok(AsyncSink::Ready),
            Err(e) => {
                println!("sink error: {:?}", e);
                Err(e.into())
            }
        }
        //Ok(self.inner.start_send(item)?)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        match self.inner.poll_complete() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(t)) => Ok(Async::Ready(t)),
            Err(e) => {
                println!("sink error: {:?}", e);
                if self.connection == ConnectionState::Connecting || self.connection == ConnectionState::Disconnected {
                    
                }
                Ok(Async::NotReady)
            }
        }
        // Ok(self.inner.poll_complete()?)
    }
}
