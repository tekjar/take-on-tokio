use std::net::SocketAddr;
use std::thread;

use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_core::io::Io;
use futures::{Sink, Future, Stream};
use futures::sync::mpsc::{self, Receiver, Sender};

use error::Error;
use codec::LineCodec;

pub struct Actor;

impl Actor {
    pub fn start(addr: String) -> Result<Sender<String>, Error> {
        let (command_tx, command_rx) = mpsc::channel::<String>(1000);

        thread::spawn(move || { Self::run(&addr, command_rx); });

        Ok(command_tx)
    }

    fn run(addr: &str, command_rx: Receiver<String>) -> Result<(), Error> {
        let addr: SocketAddr = addr.parse()?;
        let mut reactor = Core::new()?;

        let tcp = TcpStream::connect(&addr, &reactor.handle());

        let client = tcp.and_then(|connection| {
            let framed = connection.framed(LineCodec);
            let f1 = framed.send("start".to_string());

            f1.and_then(|framed| {
                    framed.into_future()
                        .and_then(|(res, stream)| Ok((res, stream)))
                        .map_err(|(err, _stream)| err)
                })
                .boxed()
        });

        let response = reactor.run(client);
        let (packet, frame) = response?;
        println!("Start ...");

        let handle = reactor.handle();
        let (network_sender, network_receiver) = frame.split();

        let receiver_future = network_receiver.for_each(|msg| {
                println!("REPLY: {:?}", msg);
                Ok(())
            })
            .map_err(|e| Error::Io(e));


        let sender_future = command_rx.map(|r| {
                r
            })
            .map_err(|_| Error::Line)
            .and_then(|p| Ok(p))
            .forward(network_sender);

        let future = receiver_future.join(sender_future);

        let _ = reactor.run(future);
        println!("@@@@@@@@@@@@@@@@@@@@");
        Ok(())
    }
}