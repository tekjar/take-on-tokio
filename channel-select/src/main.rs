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
    let handle = main_loop.handle();

    let (mut tx1, rx1) = mpsc::channel::<i32>(16);
    let (mut tx2, rx2) = mpsc::channel::<i32>(16);

    let timer = Timer::default();
    let interval = timer.interval(Duration::new(1, 0));

    let future1 = rx1.for_each(|num| {
        println!("{:?}", num);
        Ok(())
    });

    let future2 = rx2.for_each(|num| {
        println!("{:?}", num);
        Ok(())
    });

    let future3 = interval.for_each(|_| {
        println!("helloooo");
        Ok(())
    }).map_err(|_| ());

    let future = future1.join3(future2, future3);

    thread::spawn(move || {
        for i in 0..10 {
            tx1 = tx1.send(i).wait().unwrap();
            tx2 = tx2.send(-i).wait().unwrap();
            thread::sleep(Duration::new(1, 0));
        }
        thread::sleep(Duration::from_millis(10000));
    });


    let _ = main_loop.run(future);
    Ok(())
}

fn main() {
    start();
}
