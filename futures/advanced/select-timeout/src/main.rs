#[macro_use]
extern crate futures;
extern crate tokio_core;
extern crate tokio_timer;

use std::thread;
use std::time::Duration;

use futures::{Future, Stream, Sink, Async, Poll};
use futures::sync::mpsc::{self, Receiver};

use tokio_core::reactor::{Core, Handle, Timeout};

pub struct TimeoutClient {
    timeout: Timeout,
    receiver: Receiver<i32>,
    handle: Handle
}

impl TimeoutClient {
    fn new(rx: Receiver<i32>, handle: Handle) -> TimeoutClient{
        TimeoutClient {
            timeout: Timeout::new(Duration::new(3, 0), &handle).unwrap(),
            receiver: rx,
            handle: handle,
        }
    }
}

impl Stream for TimeoutClient {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<i32>, ()> {
        let _ = try_ready!(self.timeout.poll());

        // Reset the timeout
        self.sleep = self.sleep.timer().sleep(self.duration);

        Ok(Async::Ready(Some(1)))
    }
}


fn main() {
    let mut reactor = Core::new().unwrap();
    let handle = reactor.handle();

    let (mut tx, rx) = mpsc::channel::<i32>(16);

    let timeout = Timeout::new(Duration::new(8, 0), &handle);

    // rx future has item = i32 & error = ()
    let receiver = rx.for_each(|num| {
        println!("{:?}", num);
        Ok(())
    }); 
    
    thread::spawn(move || {
        for i in 0..10 {
            tx = tx.send(i).wait().unwrap();
            thread::sleep(Duration::new(1, 0));
        }
        thread::sleep(Duration::from_millis(10000));
    });


    let _ = reactor.run(receiver);
}