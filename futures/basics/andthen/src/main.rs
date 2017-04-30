extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);

    // NOTES: 
    // 1. and_then executes the closures only when future is successful
    // 2. closure should return an IntoFuture (Result implements it) whose error is same as that of f1's 
    let f = f1.and_then(|v| {
        println!("{:?}", v);
        
        // Ok::<(), f32>(()) // doesn't work (2)
        // Ok::<i32, i32>(100)  // works (2)
        // Ok(()) // works (2)
        Ok(100) // works (2)
    });

    assert_eq!(Ok(100), reactor.run(f));

    let f2 = err::<i32, i32>(-1);

    let f = f2.and_then(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok(())
    });

    assert_eq!(Err(-1), reactor.run(f));
}
