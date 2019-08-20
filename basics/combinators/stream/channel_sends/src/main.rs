
extern crate futures;
extern crate tokio_core;

use futures::{future, Stream};
use futures::stream;
use tokio_core::reactor::Core;

use futures::sync::mpsc::channel;
use futures::sync::mpsc::Sender;
use futures::sync::mpsc::Receiver;
use futures::Sink;
use std::io::{Error, ErrorKind};
use futures::future::FutureResult;
use futures::Future;
use futures::future::Either;

// fn returns_future(mut tx: Sender<u32>) -> Box<Future<Item=(), Error=Error>> { 
//     let raw = tx.send(2).map(|_| ()).map_err(|e| Error::new(ErrorKind::Other, "Error receiving client msg"));
//     Box::new(raw)
// }

enum Msg {
    Publish,
    Puback,
    Error
}

fn return_boxed_future(rx: Receiver<Msg>) -> Box<Future<Item=(), Error=()>> {
    // match 
    let leaf_future = rx.then(|result| {
        match result {
            Ok(m) => {
                let raw = future::ok(()).map_err(|_:()| ());
                return Box::new(Either::A(raw));
            }
            Err(_) => {
                let raw = future::err(()).map_err(|_| ());
                return Box::new(Either::B(raw));
            }
        }
    }).for_each(|_| future::ok(()));

    Box::new(leaf_future)
}

fn main() {
    let mut reactor = Core::new().unwrap();
    let (tx,rx) = channel(10);

    tx.clone().send(Msg::Publish).wait().unwrap();
    tx.clone().send(Msg::Puback).wait().unwrap();
    tx.clone().send(Msg::Error).wait().unwrap();
    
    // tx.send(Ok()).wait().unwrap();
    // tx.send(Ok(3)).wait().unwrap();

    // let val = returns_future(tx);
// 
    // val.for_each(|e| future::ok(()));

    let val = reactor.run(return_boxed_future(rx));
    // println!("{:?}", val);
}
