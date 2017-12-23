extern crate futures;
extern crate tokio_core;

use futures::{future, Stream};
use futures::stream;
use tokio_core::reactor::Core;


fn main() {
    let mut reactor = Core::new().unwrap();
    let mut s = stream::iter_result(vec![Ok(1), Ok(2), Ok(3), Err(false), Ok(5)]);

    /// streams operate just like stdlib. All the 'next()'s are
    /// wrapped with Some() until the end after which None is returned.
    /// The combinator iteration stops when None is reached.
    /// 
    /// 
    /// for_each also stops when it encounters Err
    let fut = s.for_each(|v| {
        println!("{}", v);
        future::ok(())
    });
    
    reactor.run(fut);
}
