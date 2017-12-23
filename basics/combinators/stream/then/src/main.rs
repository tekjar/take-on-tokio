extern crate futures;
extern crate tokio_core;

use futures::{future, Stream, Future};
use futures::stream;
use tokio_core::reactor::Core;


fn main() {
    let mut reactor = Core::new().unwrap();
    let mut number_stream = stream::iter_result(vec![Ok(1), Ok(2), Ok(3), Err(false), Ok(5)]);

    /// streams operate just like stdlib. All the 'next()'s are
    /// wrapped with Some() until the end after which None is returned.
    /// The combinator iteration stops when None is reached.
    /// 
    /// 
    /// stream 'Then' is a Stream but not a Future. So you can't `run` 
    let fut = number_stream.then(|v| {
        println!("then --> {:?}", v);
        v
    });
    
    // reactor.run(fut);

    let fut = fut.for_each(|v| {
        println!("each --> {:?}", v);
        future::ok(())
    });

    // Final Future --> ForEach<stream::Then<NumberStream>>
    // 
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
    // 
    // The combinator after ForEach are again Future combinators, Not Stream combinators.
    // So the 'Then' below is executed when ForEach returns something

    // ForEach will be done and return in 3 cases
    //
    // 1. Stream poll returned a Ok(None) --> ForEach returns Ok(())
    // 2. Stream poll returns an Error --> ForEach returns Error
    // 3. ForEach closure returns an Error -->  ForEach returns the closure Error

  

    reactor.run(fut.then(|r| {
        println!("{:?}", r);
        future::ok::<_, ()>(())
    }));
}
