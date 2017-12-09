extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);

    // AndThen Combinator:
    // -------------------
    // fn and_then<F, B>(self, f: F) -> AndThen<Self, B, F> 
    // where                                  * F get's executed when Self (Future) resolves successfully.
    //   F: FnOnce(Self::Item) -> B,          * F takes self's item when it is successful
    //   B: IntoFuture<Error = Self::Error>,  * F should produce a (Into)Future B whose error type is same as that of Self's.
    //   Self: Sized,                         * Self should also be Sized
    
    
    
    // NOTES: 
    // 1. AndThen executes the closure only when the future is successful or else the result is passed along
    // 2. closure should return an IntoFuture (Result implements it) and it's error type should be same as f1's error type
    // 3. item type of intofuture return from closure can be anything
    let f = f1.and_then(|v| {
        println!("{:?}", v);
        // Ok::<(), f32>(())    // doesn't work (2)
        // Ok::<i32, i32>(100)  // works (2)
        // Err(100)             // works (2)
        Ok(true)                // works (2) (3)
    });
    
    // Type of f = AndThen<FutureResult<i32, i32>, Result<bool, i32>, [closure@src/main.rs:25:25: 31:6]>
    // Note that event loop will try to resolve 'FutureResult' first. Only when the the future result is
    // ready, the AndThen combinator will be applied. If the future is not ready, it will be put on to 
    // readiness queues to be notified and polled again
    // In this case
    // Future<i32, i32> -> Ready(i32) -> AndThen -> Result<bool, i32>

    assert_eq!(Ok(true), reactor.run(f));

    let f2 = err::<i32, i32>(-1);

    let f = f2.and_then(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok(())
    });

    assert_eq!(Err(-1), reactor.run(f));
}