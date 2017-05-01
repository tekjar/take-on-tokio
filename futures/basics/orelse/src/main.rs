extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = err::<i32, i32>(-1);

    // NOTES: 
    // 1. or_else executes the closures only when future results an error or else passes the future along unaltered
    // 2. closure should return an IntoFuture (Result implements it) whose 'item' is same as that of f1's 
    // 3. error type of intofuture return from closure can be anything
    let f = f1.or_else(|v| {
        println!("{:?}", v);
        
        // Ok::<(), f32>(()) // doesn't work (2)
        // Ok::<i32, i32>(100)  // works (2)
        // Ok(()) // doesn't work (2)
        Ok::<i32, ()>(100) // works (2) (3)
    });

    assert_eq!(Ok(100), reactor.run(f)); 

    
    let f2 = ok::<i32, i32>(1);
    let f = f2.or_else(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok::<_, i32>(-1)
    });

    assert_eq!(Ok(1), reactor.run(f)); // reactor still return future::ok(1) which is coerced to Ok(1)
}
