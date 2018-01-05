extern crate futures;
extern crate tokio_core;

use futures::{future, Stream, Sink};
use futures::stream;
use futures::Future;
use futures::sync::mpsc;
use tokio_core::reactor::Core;

use std::thread;
use std::time::Duration;

/// Poll<T, E>
/// --------------------
/// Ok(Async::Ready(t))
/// Ok(Async::NotReady)
/// Err(e)

/// Stream Poll => Poll<Option<T>, E>
/// -------------------
/// Ok(Async::Ready(Some(t)))
/// Ok(Async::Ready(None))
/// Ok(Async::NotReady)
/// Err(e)

/// ForEach is a `Future`.
/// ForEach poll loops and calls Stream poll to get next item
/// ForEach poll loop ends when Stream poll returns Ok(None) or Err

fn main() {
    let mut reactor = Core::new().unwrap();
    let (mut tx, rx) = mpsc::channel(100);
    let mut tx1 = tx.clone();

    thread::spawn(move || {
        for i in 0..10 {
            tx = tx.send(i).wait().unwrap();
            thread::sleep(Duration::new(1, 0));
        }
        thread::sleep(Duration::from_millis(10000));
    });
    
    let s = stream::iter_result(vec![Ok(1), Ok(2), Ok(3), Ok(5)]);
    
    let fut1 = s.filter(move |v| *v == 3).for_each(move |v| {
        let tx1 = tx1.clone();
        tx1.send(1000).map(move |tx| ()).map_err(|e| ())
    });

    let fut2 = rx.for_each(|v| {
        println!("{}", v);
        future::ok(())
    });
    
    let fut = fut1.join(fut2);

    // ForEach's poll will loop and call its Stream's poll to get next element
    // http://alexcrichton.com/futures-rs/src/futures/stream/for_each.rs.html#36
    // 
    // 
    // Stream poll will return next element --> Some(next element)
    // When Stream poll return's None, ForEach poll will return --> Ok(Async::Ready(()))

    // stream::ForEach.poll(
    //     loop {
    //         result = try_ready!(NumberStream.poll())
    //         for_each_closure(result)
    //     }
    // )

    // ForEach will be done and return in 3 cases
    //
    // 1. Stream poll returned a Ok(None) --> ForEach returns Ok(())
    // 2. Stream poll returns an Error --> ForEach returns Error
    // 3. ForEach closure returns an Error -->  ForEach returns the closure Error
    
    reactor.run(fut);
}