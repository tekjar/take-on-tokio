extern crate tokio_core;
#[macro_use]
extern crate quick_error;
extern crate futures;

use std::time::Duration;
use std::thread;

use futures::sync::mpsc;
use futures::Sink;
use futures::Future;

pub mod error;
pub mod codec;
pub mod actor;

use actor::Actor;

fn main() {
    let (mut command_tx, command_rx) = mpsc::channel::<String>(1000);

    thread::spawn(move || { Actor::run(command_rx); });

    for i in 0..100 {
        let message = format!("{}. hello", i);
        command_tx = command_tx.send(message).wait().unwrap();
        thread::sleep(Duration::new(1, 0));
    }

    thread::sleep(Duration::new(100, 0));
    println!("Done!");
}
