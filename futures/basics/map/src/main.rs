extern crate futures;
extern crate tokio_core;

use futures::future::*;
use tokio_core::reactor::Core;

fn main() {
    let mut reactor = Core::new().unwrap();
    let f1 = ok::<i32, i32>(1);

    // NOTES: 
    // 1. map executes the closures only when future results ok or else passes the future along unaltered
    // 2. doesn't care about item, error type matching b/w f1 & closure because of below point 
    //    ( but Map item type will be closure return type & error type will be same as that of f1)
    // 3. closure return value doesn't have to be intofuture
    //    ( but Map will take care of converting closure return type to Result (which is an intofuture) by
    //      With Ok::<_, f1's error type>(closure return) /)
    let f = f1.map(|v| {
        println!("{:?}", v);
        true // (2), (3)
    });

    assert_eq!(Ok(true), reactor.run(f)); // Map poll will wrap 'true' to Ok(true)

    let f1 = ok::<i32, i32>(1);
    let f = f1.map(|v| {
        println!("{:?}", v);
        Ok::<_, ()>(true)
    });

    assert_eq!(Ok(Ok(true)), reactor.run(f));

    let f1 = err::<i32, i32>(-1);
    let f = f1.map(|v| {
        println!("{:?}", v); // doesn't get executed
        Ok::<_, ()>(true)
    });

    assert_eq!(Err(-1), reactor.run(f));
}
