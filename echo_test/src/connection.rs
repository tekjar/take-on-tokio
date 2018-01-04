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

use codec::LineCodec;
use error::Error;
use futures::future;

use std::io::Error as StdError;

pub struct Connection;

use futures::stream;

impl Connection {
    pub fn start(addr: String) -> Result<Sender<String>, Error> {
        let (command_tx, command_rx) = mpsc::channel::<String>(1000);
        thread::spawn(move || { Self::run(&addr, command_rx); });
        Ok(command_tx)
    }

    fn run(addr: &str, mut command_rx: Receiver<String>) {
        let addr: SocketAddr = addr.parse().unwrap();
        let mut reactor = Core::new().unwrap();
        let handle = reactor.handle();
        let addr = addr.clone();

        let tcp = TcpStream::connect(&addr, &reactor.handle());
        let con_future = tcp.and_then(|connection| {
            let framed = connection.framed(LineCodec);
            let future_mqtt_connect = framed.send("sdf".to_string());
            future_mqtt_connect.and_then(|framed| {
                framed.into_future().and_then(|(res, stream)| Ok((res, stream))).map_err(|(err, _stream)| err)
            })
        });

        let response = reactor.run(con_future);

        let (packet, frame) = response.unwrap();

        let (mut sender, receiver) = frame.split();

        let client_rx = command_rx.by_ref();

        // receiver


        let for_each_fut = receiver.map_err(|e| {
            println!("Map err before");e}).for_each(|s| {
            println!("MSG {:?}", s);
            future::ok(())
        }).map_err(|e| {
            panic!("sdfkjhs");
            println!("Disconnected {:?}", e);
        });

        handle.spawn(for_each_fut);
        
        let v = client_rx.map(|s| {
            println!("Client msg {:?}", s);
            s
        }).map_err(|e| {
            println!("Map err");
            StdError::new(ErrorKind::Other, "oh no!")
            })
        .forward(sender);

        let res = reactor.run(v);
    }
}

pub fn test_future() {
    let mut stream = stream::iter_ok::<_, ()>(1..8).and_then(|e| {
        // e+2
        if e == 4 {
            return future::err(())
        }
        future::ok(e)
    });
    // let number_stream = stream::iter::<_, _, ()>((0..6).map(Ok);.and_then(|e| {
    //     e+2;
        // if e == 2 {
        //    return Err(e)
        // }
        // Ok(e)
        // }));

    let mut reactor = Core::new().unwrap();
    let handle = reactor.handle();

    let next_fut = stream.then(|e| {
        println!("Inner then");
        future::ok(e)
    }).for_each(|e| {
        println!("{:?}", e);
        if e == Ok(2) {
            return future::err(())
        }
        future::ok::<_,()>(())
    });

    let last_fut = next_fut.then(|e| {
        match e {
            Err(e) => {
                println!("Stream ended {:?}", e)
            }
            Ok(v) => println!("End success")
            }
        future::ok::<_,()>(())
    });

    let val = reactor.run(last_fut);
}