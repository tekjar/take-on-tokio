use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::sync::mpsc::Receiver;

use error::Error;

pub struct Actor;



impl Actor {
    pub fn run(command_rx: Receiver<String>) -> () {
        let mut reactor = Core::new().unwrap();

        let executor = command_rx.for_each(|msg| {
                println!("COMMAND: {:?}", msg);
                Ok(())
            })
            .boxed();

        let _ = reactor.run(executor);
    }
}
