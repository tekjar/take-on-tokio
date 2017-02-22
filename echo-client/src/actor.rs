use std::net::SocketAddr;
use std::thread;

use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_core::io::Io;
use futures::{Sink, Future, Stream};
use futures::sync::mpsc::{self, Receiver, Sender};

use error::Error;
use codec::LineCodec;

use std::time::Duration;
pub struct Actor;

impl Actor {
    pub fn start(addr: String) -> Result<Sender<String>, Error> {
        let (command_tx, command_rx) = mpsc::channel::<String>(1000);
        thread::spawn(move || { 
            Self::run(&addr, command_rx);
        });
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