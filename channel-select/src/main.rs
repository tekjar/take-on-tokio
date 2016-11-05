extern crate futures;
extern crate tokio_core;

use std::thread;
use std::time::Duration;

use futures::stream::Stream;
use futures::Future;

use tokio_core::channel::channel;
use tokio_core::reactor::{Core, Interval};

fn main() {
    let mut main_loop = Core::new().unwrap();
    let handle = main_loop.handle();

    let (tx1, rx1) = channel::<i32>(&handle).unwrap();
    let (tx2, rx2) = channel::<i32>(&handle).unwrap();
    let interval = Interval::new(Duration::new(1, 0), &handle).unwrap();

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
    });

    let future = future1.select(future3);

    thread::spawn(move || {
        for i in 0..10 {
            tx1.send(i).unwrap();
            tx2.send(-i).unwrap();
        }
        thread::sleep(Duration::from_millis(10000));
    });


    let _ = main_loop.run(future);
}
