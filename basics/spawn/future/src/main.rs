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
    
    // This wouldn't work because spawn expects a Future whose Item = () & Error = ()
    // AndThen results in a future whose Item = bool & Error = fut's error i.e i32
    // let fut = fut.and_then(|v| err::<bool, _>(-100))

    // This modifies fut's item and error type to ()
    let fut = fut.and_then(|v| err::<bool, _>(-100)).map(|v| ()).map_err(|e| ());
    handle.spawn(fut);

    let fut = ok::<i32, i32>(100);
    let fut = fut.then(|result| {
        match result {
            Ok(val) => ok(true),
            Err(e) => err(false),
        }
    }).map(|v| ()).map_err(|e| ());

    handle.spawn(fut)
}
