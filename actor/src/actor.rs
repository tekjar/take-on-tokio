use std::thread;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::sync::mpsc::{self, Receiver, Sender};
use futures::future::{Loop, loop_fn};

use error::Error;

pub struct Actor;

fn run(command_rx: Receiver<String>) -> Box<Future<Item = (), Error = ()>> {

    let executor = command_rx.for_each(|msg| {
        println!("COMMAND: {:?}", msg);
        Ok(())
    });

    executor.boxed()
}

impl Actor {
    pub fn start() -> Result<Sender<String>, Error> {
        let (command_tx, command_rx) = mpsc::channel::<String>(1000);
        thread::spawn(move || {
            let e = run(command_rx);
            let mut reactor = Core::new().unwrap();
            let handle = reactor.handle();

            let client = loop_fn((), |_| {
                run(command_rx).map(|_| -> Loop<(), ()> { Loop::Continue(()) })
            });

            reactor.run(client).unwrap();
        });
        Ok(command_tx)
    }
}
