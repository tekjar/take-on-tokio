extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let handle = reactor.handle();

    let fut = ok::<i32, i32>(100);
    
    // let fut = fut.map(|v| v * 3);
    // let result = reactor.run(fut);

    let fut = fut.and_then(|v| err::<i32, _>(-100));
    let result = reactor.run(fut);
    
    println!("{:?}", result);
}
