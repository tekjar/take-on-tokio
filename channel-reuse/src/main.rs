#![recursion_limit = "1024"]

extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;
#[macro_use]
extern crate error_chain;
pub mod errors;

use std::thread;
use std::time::Duration;

use futures::stream::Stream;
use futures::{Future, Sink};
use futures::sync::mpsc;

use tokio_core::reactor::Core;
use tokio_timer::Timer;
use errors::*;


fn start() -> Result<()> {
    let mut main_loop = Core::new().unwrap();

    let (mut tx1, rx1) = mpsc::channel::<i32>(16);

    thread::spawn(move || {
        for i in 0..10 {
            tx1 = tx1.send(i).wait().unwrap();
            thread::sleep(Duration::new(1, 0));
        }
        thread::sleep(Duration::from  millis(10000));
    });

    let future = rx1.for_each(|num| {
        println!("{:?}", num);
        Ok(())
    });

    let future = future.boxed();
    
    loop {
        //
        // reconnect to the server and return a new `framed`
        //
        // let (sender, receiver) = framed.split();
        // let future = future.forward(sender);

        let f = main_loop.run(future);
        let () = f;
    }
    Ok(())
}

fn main() {
    start();
}