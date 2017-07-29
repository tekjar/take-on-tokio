extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = err::<i32, i32>(-1);
    
    // OrElse Combinator:
    // ------------------
    // fn or_else<F, B>(self, f: F) -> OrElse<Self, B, F> 
    // where                                 * F get's executed only when Self(Future) resolves to an error.
    //   F: FnOnce(Self::Error) -> B,        * F takes self's error and should produce
    //   B: IntoFuture<Item = Self::Item>,   * a (Into)Future B whose item type is same as that of Self's.
    //   Self: Sized,                        * Self should also be Sized


    // NOTES: 
    // 1. or_else executes the closures only when future results an error or else passes the result along unaltered
    // 2. closure should return an IntoFuture (Result implements it) whose 'item' is same as that of f1's 
    // 3. error type of intofuture return from closure can be anything
    let f = f1.or_else(|v| {
        println!("{:?}", v);
        
        // Ok::<(), f32>(())         // doesn't work (2)
        // Ok::<i32, i32>(100)       // works (2)
        // Ok(())                    // doesn't work (2)
        Err(false)                   // works (2)
        // Ok::<i32, ()>(100)        // works (2) (3)
    });

    assert_eq!(Err(false), reactor.run(f)); 

    
    let f2 = ok::<i32, i32>(1);
    let f = f2.or_else(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok::<_, i32>(-1)
    });

    assert_eq!(Ok(1), reactor.run(f)); // reactor still return future::ok(1) which is coerced to Ok(1)
}