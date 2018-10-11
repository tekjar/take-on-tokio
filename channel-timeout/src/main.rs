extern crate futures;
extern crate tokio; // 0.1.8 // 0.1.23

use std::thread;
use std::time::Duration;

use futures::{future, stream};
use futures::{Future, Stream};
use futures::sync::mpsc;
use tokio::runtime::current_thread;
use tokio::timer::Timeout;

fn main() {
    let mut rt = current_thread::Runtime::new().unwrap();

    let (tx, rx) = mpsc::channel::<i32>(10);
    let rx = Timeout::new(rx, Duration::new(3, 0));
    let rx = rx.or_else(|e| {
        println!("1. {:?}", e);
        future::ok::<_, i32>(-1)
    });
    let rx_future = rx.for_each(|v| {
        println!("2. {:?}", v);
        future::ok(())
    });
    
    let out = rt.block_on(rx_future);
    println!("3. {:?}", out);
}