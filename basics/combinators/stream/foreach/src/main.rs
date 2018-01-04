extern crate futures;
extern crate tokio_core;

use futures::{future, Stream};
use futures::stream;
use tokio_core::reactor::Core;

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
    let s = stream::iter_result(vec![Ok(1), Ok(2), Ok(3), Err(false), Ok(5)]);

    let fut = s.for_each(|v| {
        println!("{}", v);
        future::ok(())
    });

    // ForEach's poll will loop and call its Stream's poll to get next element
    // http://alexcrichton.com/futures-rs/src/futures/stream/for_each.rs.html#36
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
    
    let val = reactor.run(fut);
    println!("{:?}", val);
}
