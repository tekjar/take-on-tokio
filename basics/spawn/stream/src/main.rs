extern crate futures;
extern crate tokio_core;

use futures::future::*;
use futures::{Stream, Sink};
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;

fn main() {
    let mut reactor = Core::new().unwrap();
    let handle = reactor.handle();

    let (mut tx, rx) = mpsc::channel(10);
    let (mut tx1, rx1) = mpsc::channel(10);

    thread::spawn(move || {
        let tx = &mut tx;

        for i in 0..10 {
            if let Err(e) = tx.send(i).wait() {
                println!("{}", e);
            }
        }
    });

    thread::spawn(move || {
        let tx = &mut tx1;

        for i in 0..10 {
            thread::sleep_ms(1000);
            tx.send(i).wait().unwrap();
        }
    });

    // slow receiver
    let rx = rx.for_each(|v| {
        println!("1. {}", v);
        ok(())
    });

    // fast receiver
    let rx1 = rx1.then(|result| {
        match result {
            Ok(e) => ok(e + 3),
            Err(()) => err(4),
        }
    });

    // spawn<F>(&self, f: F)
    // where F: Future<Item=(), Error=()> + 'static     * Takes a `Future`

    handle.spawn(rx);
    
    // NOTE:
    // 'rx1' doesn't work here like future::Then because stream::Then is not a `Future`
    // both 'spawn' and 'run' takes only `Future`
    // handle.spawn(rx1);
    // reactor.run(rx1);

    let rx1 = rx1.for_each(|_| ok(()));
    reactor.run(rx1);
}