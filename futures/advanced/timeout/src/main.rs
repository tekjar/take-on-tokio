extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use futures::future::Future;
use futures::{Poll, Async};
use tokio_core::reactor::Core;
use tokio_timer::{Timer, Sleep};

use std::time::Duration;

// An integer future that becomes ready after 5 secs
struct I32 {
    num: i32,
    sleep: Sleep,
}

impl I32 {
    fn new(num: i32) -> Self {
        I32 {
            num: num,
            timer: Timer::default().sleep(Duration::new(5, 0)),
        }
    }
}

impl Future for I32 {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<i32, ()> {
        // Leaf future (sleep) will register the root future (I32) for notification if `NotReady`
        // If I had used my own thread::sleep() (on a seperate thread) to make the 
        // future ready after certain time, then I would have to manually do all 
        // the work that timer crate is doing for me i.e putting back `I32` back on
        // event loop queue for it to be polled again
        match self.sleep.poll() {
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Ok(Async::Ready(_)) => Ok(Async::Ready(self.num)),
            Err(e) => Err(()),
        }
    }
}

fn main() {
    let num = I32::new(10);
    let mut reactor = Core::new().unwrap();
    let future = num.and_then(|n| {
                                  println!("{:?}", n);
                                  Ok(())
                              });

    reactor.run(future).unwrap();
}
