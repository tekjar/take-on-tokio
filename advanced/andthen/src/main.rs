extern crate futures;
extern crate tokio_core;

use futures::future::Future;
use futures::{Poll, Async};
use futures::task::Task;
use futures::
use tokio_core::reactor::Core;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct I32 {
    num: i32,
    complete: Arc<AtomicBool>,
    task: Option<Task>
}

impl I32 {
    fn new(num: i32) -> Self {
        let complete = Arc::new(AtomicBool::new(false));
        let c1 = complete.clone();

        thread::spawn(move || {
            for _ in 0..10 {
                thread::sleep(Duration::new(1, 0));
            }
            c1.store(true, Ordering::SeqCst);
        });

        I32 {
            num: num,
            complete: complete,
            task: None,
        }
    }
}

impl Future for I32 {
    type Item = i32;
    type Error = ();

    fn poll(&mut self) -> Poll<i32, ()> {
        if self.complete.load(Ordering::SeqCst) == true {
            Ok(Async::Ready(self.num))
        } else {
            Ok(Async::NotReady)
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
